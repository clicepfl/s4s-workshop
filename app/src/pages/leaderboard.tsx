import { useState } from "react";
import {getScoreboard} from "@/api/api";
import {Scoreboard} from "@/api/models";

export default function LeaderboardPage() {
  const [scores, setScores] = useState<Scoreboard | null>(null);
    getScoreboard().then((data) => setScores(data.scores));

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
