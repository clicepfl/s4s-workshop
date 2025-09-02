use play::Game;
use rocket::{
    http::Header,
    post,
    request::{self, FromRequest},
    response::Responder,
    routes, Request, Response, Route,
};
use serde::Serialize;
use std::{collections::HashMap, fs::read_dir, io::Cursor, str::FromStr, sync::Arc};
use submissions::{Language, Submission};

use crate::{config::config, game::Move};
use crate::api::contest::Scoreboard;

pub mod contest;
pub mod play;
pub mod submissions;

#[derive(Default, Debug, Clone)]
pub struct State {
    // There is no reason for this to be a hashmap instead of a vector
    pub submissions: HashMap<String, Submission>,
    pub games: HashMap<String, Arc<rocket::tokio::sync::Mutex<Game>>>,
}

impl State {
    pub fn load() -> Result<Self, Error> {
        Ok(State {
            submissions: read_dir(config().data_dir.clone())?
                .filter_map(|d| d.ok())
                .filter(|d| d.file_type().is_ok_and(|t| t.is_file()))
                .filter_map(|d| {
                    let file_name = d.file_name();
                    let binding = dbg!(file_name.into_string().unwrap());

                    binding.rsplit_once('.').and_then(|(name, lang)| {
                        Some((
                            name.to_string(),
                            Submission {
                                name: name.to_string(),
                                lang: Language::from_str(lang).ok()?,
                                code: d.path(),
                            },
                        ))
                    })
                })
                .collect(),
            ..Default::default()
        })
    }
}

pub type AppState = rocket::State<std::sync::Mutex<State>>;

pub fn routes() -> Vec<Route> {
    routes![
        submissions::get_submission,
        submissions::post_submission,
        play::get_game,
        play::start,
        play::stop,
        play::play,
        login,
        contest::run_tournament,
        contest::get_scoreboard,
    ]
}

#[post("/login?<name>")]
pub async fn login(name: &str, state: &AppState) -> rocket::http::Status {
    let mut lock = state.lock().unwrap();

    if lock.submissions.contains_key(name) {
        rocket::http::Status::Conflict
    } else if let Ok(submission) = Submission::empty(name.to_string()) {
        lock.submissions.insert(name.to_string(), submission);
        rocket::http::Status::Ok
    } else {
        rocket::http::Status::InternalServerError
    }
}

#[derive(Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub enum AIError {
    InvalidMove,
    InvalidOutput,
    EmptySubmission,
}

#[derive(Debug, Serialize,Clone)]
#[serde(untagged)]
pub enum Error {
    IO,
    Poison,
    InvalidLanguage,
    NotFound,
    InvalidMove,
    AIFailed {
        error: AIError,
        ai_output: String,
        #[serde(rename = "move")]
        move_: Option<Vec<Move>>,
    },
    Unauthorized,
    GameAlreadyInProgress,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        use rocket::http::Status;

        Err(match self {
            Error::IO | Error::Poison => Status::InternalServerError,
            Error::NotFound => Status::NotFound,
            Error::InvalidMove | Error::GameAlreadyInProgress | Error::InvalidLanguage => {
                Status::BadRequest
            }
            e @ Error::AIFailed { .. } => {
                return Ok(Response::build()
                    .status(Status::NotAcceptable)
                    .header(Header::new("Content-Type", "application/json"))
                    .streamed_body(Cursor::new(serde_json::to_string(&e).unwrap()))
                    .finalize())
            }
            Error::Unauthorized => Status::Unauthorized,
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        dbg!(value);
        Self::IO
    }
}

impl<Guard> From<std::sync::PoisonError<Guard>> for Error {
    fn from(value: std::sync::PoisonError<Guard>) -> Self {
        dbg!(value);
        Self::Poison
    }
}

pub struct User {
    pub name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = self::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(name) = req
            .headers()
            .get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
        {
            request::Outcome::Success(User {
                name: name.to_owned(),
            })
        } else {
            request::Outcome::Error((rocket::http::Status::Unauthorized, Error::Unauthorized))
        }
    }
}
