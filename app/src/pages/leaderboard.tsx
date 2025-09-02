import { useState, useEffect } from "react";

type PlayerStats = {
  elo: number;
  wins: number;
  losses: number;
  draws: number;
};

type ScoresResponse = {
  scores: Record<string, PlayerStats>;
};

// Example JSON for testing
const exampleData: ScoresResponse = {
  scores: {
    user1: { elo: 1037.4, wins: 15, losses: 13, draws: 5 },
    user2: { elo: 1005.3, wins: 13, losses: 11, draws: 9 },
    user3: { elo: 1100.1, wins: 20, losses: 5, draws: 3 },
    user4: { elo: 950.7, wins: 10, losses: 12, draws: 4 },
  },
};

export default function LeaderboardPage() {
  const [scores, setScores] = useState<Record<string, PlayerStats> | null>(null);

  useEffect(() => {
    // Instead of fetching, just set the example JSON
    setScores(exampleData.scores);
  }, []);

  return (
    <div className="leaderboard-container">
      <h1 className="leaderboard-title">Tournament Leaderboard</h1>

      {scores && (
        <table className="leaderboard-table">
          <thead>
            <tr className="leaderboard-header-row">
              <th className="leaderboard-header-cell">User</th>
              <th className="leaderboard-header-cell">ELO</th>
              <th className="leaderboard-header-cell">Wins</th>
              <th className="leaderboard-header-cell">Losses</th>
              <th className="leaderboard-header-cell">Draws</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(scores)
              .sort(([, a], [, b]) => b.elo - a.elo)
              .map(([username, stats]) => (
                <tr key={username} className="leaderboard-row">
                  <td className="leaderboard-cell">{username}</td>
                  <td className="leaderboard-cell">{stats.elo.toFixed(1)}</td>
                  <td className="leaderboard-cell">{stats.wins}</td>
                  <td className="leaderboard-cell">{stats.losses}</td>
                  <td className="leaderboard-cell">{stats.draws}</td>
                </tr>
              ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
