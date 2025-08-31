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

export default function LeaderboardPage() {
  const [scores, setScores] = useState<Record<string, PlayerStats> | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchLeaderboard = async () => {
      try {
        const res = await fetch("/tournament", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ /* add request body if needed */ }),
        });

        if (!res.ok) {
          throw new Error("Failed to fetch leaderboard");
        }

        const data: ScoresResponse = await res.json();
        setScores(data.scores);
      } catch (err) {
        if (err instanceof Error) {
          setError(err.message);
        } else {
          setError("Unknown error");
        }
      } finally {
        setLoading(false);
      }
    };

    fetchLeaderboard();
  }, []);

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">Tournament Leaderboard</h1>

      {loading && <p>Loading...</p>}
      {error && <p style={{ color: "red" }}>{error}</p>}

      {scores && (
        <table className="mt-6 border-collapse border border-gray-300">
          <thead>
            <tr>
              <th className="border px-4 py-2">User</th>
              <th className="border px-4 py-2">ELO</th>
              <th className="border px-4 py-2">Wins</th>
              <th className="border px-4 py-2">Losses</th>
              <th className="border px-4 py-2">Draws</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(scores)
              .sort(([, a], [, b]) => b.elo - a.elo) // sort by ELO desc
              .map(([username, stats]) => (
                <tr key={username}>
                  <td className="border px-4 py-2">{username}</td>
                  <td className="border px-4 py-2">{stats.elo.toFixed(1)}</td>
                  <td className="border px-4 py-2">{stats.wins}</td>
                  <td className="border px-4 py-2">{stats.losses}</td>
                  <td className="border px-4 py-2">{stats.draws}</td>
                </tr>
              ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
