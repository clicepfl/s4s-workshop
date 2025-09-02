use super::{AppState,Error};
use crate::{
    game::{GameStatus,GameState,Player},
    api::{
        play::Game,
        submissions::Submission,
    },
    config::config,
};
use std::fs;
use std::collections::HashMap;
use std::fmt::Formatter;
use rand::{seq::SliceRandom, rngs::SmallRng, SeedableRng};
use rocket::{get, post, serde::json::Json};
use serde::{ser::{Serializer, SerializeStruct}, Deserialize, Deserializer, Serialize};
use serde::de::Visitor;
use skillratings::{elo::{elo, EloConfig, EloRating}, Outcomes};

// The original code was clearly never designed for this

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scoreboard {
    scores: HashMap<String,Score>,
}

#[derive(Debug,Copy,Clone,Default)]
pub struct Score {
    elo: EloRating,
    wins: u16,
    losses: u16,
    draws: u16,
}
// For Elo rating
impl Serialize for Score {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut s = serializer.serialize_struct("Score", 4)?;
        s.serialize_field("elo", &self.elo.rating)?;
        s.serialize_field("wins", &self.wins)?;
        s.serialize_field("losses", &self.losses)?;
        s.serialize_field("draws", &self.draws)?;
        s.end()
    }
}


impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScoreVisitor;

        impl<'de> Visitor<'de> for ScoreVisitor {
            type Value = Score;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("struct Score")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Score, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut elo = None;
                let mut wins = None;
                let mut losses = None;
                let mut draws = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "elo" => elo = Some(EloRating { rating: map.next_value()? }),
                        "wins" => wins = Some(map.next_value()?),
                        "losses" => losses = Some(map.next_value()?),
                        "draws" => draws = Some(map.next_value()?),
                        _ => return Err(serde::de::Error::unknown_field(&key, &["elo", "wins", "losses", "draws"])),
                    }
                }

                Ok(Score {
                    elo: elo.unwrap_or_default(),
                    wins: wins.unwrap_or_default(),
                    losses: losses.unwrap_or_default(),
                    draws: draws.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_map(ScoreVisitor)
    }
}

struct AiGame {
    game: Game,
    w_player: Submission,
    b_player: Submission,
    turns: u8,
}

impl AiGame {
    async fn play(&mut self) -> Option<GameResult> {
        let current_player = self.game.checkers.current_player;
        let current_submission =
            if current_player == Player::White { &self.w_player }
            else { &self.b_player };

        let _ = self.game.play_ai(current_submission.clone()).await;

        // Handle draw
        // TODO: Is this needed?
        if self.turns >= 100 {
            return Some(GameResult {
                winner: self.w_player.clone(),
                loser: self.b_player.clone(),
                draw:true
            });
        }

        // Handle victory
        if let GameStatus::Victory(winner) = self.game.checkers.status {
            let (winner,loser) = if winner == Player::White { (self.w_player.clone(), self.b_player.clone()) }
            else { (self.b_player.clone(), self.w_player.clone()) };
            return Some(GameResult {
                winner,
                loser,
                draw: false
            });
        }

        None
    }
}

struct GameResult {
    winner: Submission,
    loser: Submission,
    draw: bool,
}

#[post("/tournament")]
pub async fn run_tournament(state: &AppState) -> Result<Json<Scoreboard>, Error> {
    let mut games = vec![];
    let mut scores: HashMap<String, Score> = HashMap::new();

    print!("Running tournament...");
    // Generate games
    let lock = state.lock()?.clone();
    let contestants = lock.submissions.values();

    contestants.clone().for_each(|c1|
        contestants.clone().for_each(|c2| {
            if c1 == c2 { return; }
            games.push(AiGame {
                game: Game { checkers: GameState::default(), human_player: Player::White },
                w_player: c1.clone(),
                b_player: c2.clone(),
                turns: 0,
            });
        })
    );

    // Ensure fairness for calculating elo
    let mut rng = SmallRng::from_os_rng();
    games.shuffle(&mut rng);

    // Play games
    for mut game in games {
        loop {
            let result = game.play().await;

            if let Some(i) = result {
                let winner_sb = scores.get(&i.winner.name).copied().unwrap_or_default();
                let loser_sb = scores.get(&i.loser.name).copied().unwrap_or_default();

                // Calculate elo
                let (winner_elo, loser_elo) = elo(
                    &winner_sb.elo,
                    &loser_sb.elo,
                    if i.draw { &Outcomes::DRAW } else { &Outcomes::WIN },
                    &EloConfig::default());

                // Generate and insert new scoreboard values
                let (winner_sb,loser_sb) = if i.draw {
                    (Score { draws: winner_sb.draws + 1, elo: winner_elo, ..winner_sb},
                    Score { draws: loser_sb.draws + 1, elo: loser_elo,..loser_sb})
                } else {
                    (Score { wins: winner_sb.wins + 1, elo: winner_elo,..winner_sb},
                    Score { losses: loser_sb.losses + 1, elo: loser_elo,..loser_sb})
                };

                scores.insert(i.winner.name, winner_sb);
                scores.insert(i.loser.name, loser_sb);
                break;
            }
        }
    }
    println!("Done!");
    let scoreboard = Scoreboard{scores};

    // Save tournament results to file
    let json = serde_json::to_string_pretty(&scoreboard).map_err(|_| Error::IO)?;
    fs::write(format!("{}/tournament.json", config().data_dir), json).map_err(|_| Error::IO)?;


    Ok(Json(scoreboard))
}

#[get("/tournament")]
pub async fn get_scoreboard() -> Result<Json<Scoreboard>, Error> {
    println!("Getting scoreboard");
    let json = fs::read_to_string(format!("{}/tournament.json", config().data_dir))
        .map_err(|_| Error::NotFound)?;
    println!("{}", json);
    let scoreboard: Scoreboard = serde_json::from_str(&json)
        .map_err(|e| {
            println!("{:?}", e);
            Error::IO
        })?;
    Ok(Json(scoreboard))
}