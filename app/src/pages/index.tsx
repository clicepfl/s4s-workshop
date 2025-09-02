import Head from "next/head";
import { Inter } from "next/font/google";
import Editor from "@monaco-editor/react";
import { useEffect, useState } from "react";
import Board from "@/components/Board";
import {
    AIError,
    AIErrorType,
    Board as BoardState,
    emptyBoard,
    initialBoards, MoveSequence,
    Player, RichMoveSequence,
    SubmissionLanguage,
    TurnStatus,
} from "../api/models";
import {
  createGame,
  loadSubmission,
  requireSession,
  stopGame,
  submitCode,
} from "../api/api";
import SwapIcon from "@/components/icons/SwapIcon";
import { getInitialCode, initFiles } from "@/util/initCodeFiles";
import { rotateBoard } from "@/util/checkersCalculator";

const inter = Inter({ subsets: ["latin"] });

export enum ConsoleMessageType {
  Error = "error",
  Info = "info",
  Warning = "warning",
  Success = "success",
}
export type ConsoleMessage = { msg: string; msgType: ConsoleMessageType };

export default function Home({ username }: { username: string }) {
  const [selectedLang, setLang] = useState(SubmissionLanguage.Java);
  const [file, setFile] = useState("");
  const [gameOngoing, setGameOngoing] = useState(false);
  const [player, setPlayer] = useState(Player.White);
  const [currentTurn, setCurrentTurn] = useState<Player | null>(null);
  const [board, setBoard] = useState(initialBoards[player]);
  const [availableSequences, setAvailableSequences] = useState<RichMoveSequence[]>([]);

  const [consoleOutput, setConsoleOutput] = useState<ConsoleMessage[]>([]);

  const [lastSubmission, setLastSubmission] = useState<{
    lang: SubmissionLanguage;
    code: string;
  } | null>(null);

  useEffect(() => {
    getInitialCode(SubmissionLanguage.Java).then((code) => setFile(code));
  }, []);

  useEffect(() => {
    setBoard(initialBoards[player]);
  }, [player]); // player can only change when the game is not ongoing

  async function changeLang(lang: SubmissionLanguage) {
    const currentInitialCode = await getInitialCode(selectedLang);
    const wantedInitialCode = await getInitialCode(lang);

    if (currentInitialCode === file) {
      // If the current file is the default one, we can safely change the language
      setLang(lang);
      setFile(wantedInitialCode);
    } else {
      // Otherwise, we ask for confirmation
      if (
        confirm(
          "Are you sure ? Changing the language will overwrite your current code."
        )
      ) {
        setLang(lang);
        setFile(wantedInitialCode);
      }
    }
  }

  function updateGame(
    turnStatus: TurnStatus | AIError,
    initialConsoleOutput?: ConsoleMessage[]
  ) {
    let newConsoleOutput = initialConsoleOutput ?? consoleOutput;

    if ("error" in turnStatus) {
      switch (turnStatus.error) {
        case AIErrorType.NoSubmission:
          newConsoleOutput = newConsoleOutput.concat({
            msg: "No submission found",
            msgType: ConsoleMessageType.Error,
          });
          break;
        case AIErrorType.InvalidMove:
          newConsoleOutput = newConsoleOutput
            .concat({
              msg: "AI played invalid move",
              msgType: ConsoleMessageType.Error,
            })
            .concat({
              msg: turnStatus.move
                ? turnStatus.move.length == 1
                  ? `Move from ${turnStatus.move[0].from} to ${turnStatus.move[0].to} is invalid`
                  : "Sequence of moves is invalid : \n" +
                    turnStatus.move
                      .map((m) => `-> Move from ${m.from} to ${m.to}`)
                      .join("\n")
                : "No move provided",
              msgType: ConsoleMessageType.Warning,
            });
          break;
        case AIErrorType.InvalidOutput:
          newConsoleOutput = newConsoleOutput.concat({
            msg: "AI error or invalid output",
            msgType: ConsoleMessageType.Error,
          });
          break;
      }

      setGameOngoing(false);
    } else {
      setBoard(rotateBoard(turnStatus.game.board, player)); // update board with server response
      setAvailableSequences(turnStatus.available_moves);
      setCurrentTurn(turnStatus.game.current_player);
    }

    if (turnStatus.ai_output && turnStatus.ai_output.length > 0) {
      newConsoleOutput = newConsoleOutput
        .concat({
          msg: turnStatus.ai_output,
          msgType: ConsoleMessageType.Warning,
        });
    }

    setConsoleOutput(newConsoleOutput);
  }

  return (
    <>
      <Head>
        <title>CLIC Workshop</title>
        <meta name="description" content="Checkers Algo Workshop for S4S" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <main>
        <div className="topbar">
          <img className="logo" src="s4s/clic.svg" />
          <h1>Workshop Jeu de Dames</h1>
          <div className="toolbar">
            <button
              className="button"
              disabled={gameOngoing}
              onClick={() =>
                setPlayer(player === Player.White ? Player.Black : Player.White)
              }
            >
              <SwapIcon />
            </button>
            <button
              className="button"
              onClick={() => {
                if (gameOngoing) {
                  stopGame(username);
                  setGameOngoing(false);
                } else {
                  setBoard(initialBoards[player]);
                  setConsoleOutput([]);
                  createGame(username, player == Player.White).then(
                    (turnStatus) => {
                      setGameOngoing(true);
                      updateGame(turnStatus, []);
                    },
                    (err) => alert(err.message)
                  );
                }
              }}
            >
              {gameOngoing ? "Stop Game" : "Start Game"}
            </button>
            <button
              className="button"
              onClick={() =>
                submitCode(selectedLang, file, username).then(
                  () => {
                    alert("Code submitted !");
                    setLastSubmission({ lang: selectedLang, code: file });
                  },
                  (err) => alert(err.message)
                )
              }
            >
              Submit {(lastSubmission == null || lastSubmission.lang != selectedLang || lastSubmission.code != file) ? "✗" : "✓"} 
            </button>

            <button
              className="button"
              onClick={() => {
                if (
                  confirm(
                    "Are you sure ? Loading your last submission will overwrite your current code."
                  )
                ) {
                  loadSubmission(username).then(
                    ({ lang, code }) => {
                      setLang(lang);
                      setFile(code);
                    },
                    (err) => alert(err.message)
                  );
                }
              }}
            >
              Load
            </button>

            <select
              className="lang-select"
              value={selectedLang}
              onChange={(e) => changeLang(e.target.value as SubmissionLanguage)}
            >
              {Object.values(SubmissionLanguage).map((lang) => (
                <option key={lang} value={lang}>
                  {initFiles[lang].name}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div className="main-content">
          <div className="side-panel">
            <div className="instructions">
              <p>Bienvenue {username} !</p>
              <p>
                Votre objectif est de programmer une <b>IA qui joue au Dames</b>
                . A chaque tour, la fonction <code>find_move</code> que vous allez coder
                sera appelée pour déterminer le coup (ou la séquence de coups) à
                jouer. Vous pouvez jouer manuellement contre l’IA pour la
                tester.
              </p>
              <p>
                Pour commencer, voici le lien vers les{" "}
                <b>
                  <a
                    href="http://www.ffjd.fr/fichiers/livres/livret-ffjd.pdf"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    règles du Jeu de Dames International
                  </a>
                </b>
                {". "}
                Attention! il existe plusieurs variantes du Jeu de Dames. En cas
                de doute, fiez vous à ces règles.
              </p>
              <p>
                On vous fournit une <b>liste de sequences de coups valides</b>
                en argument à votre fonction. Votre but est de choisir les
                coups (ou enchaînements de coups) qui permettront à votre IA de
                remporter un maximum de duels contre celles des autres joueurs.
              </p>
              <p>
                Note : les coups a retourner sont au format row, column à chaque
                fois.
              </p>
            </div>
            <div className="game">
              <div className="simulation">
                <Board
                  username={username}
                  player={player}
                  gameOngoing={gameOngoing}
                  currentTurn={currentTurn}
                  setCurrentTurn={setCurrentTurn}
                  board={board}
                  setBoard={setBoard}
                  updateGame={updateGame}
                  availableSequences={availableSequences}
                />
              </div>
              <div className="game-info">
                <p>
                  <b>Game State:</b> {gameOngoing ? "Ongoing" : "Stopped"}
                </p>
                <p>
                  <b>Current Turn:</b> {currentTurn ?? "None"}
                </p>
                <p>
                  <b>Human Player:</b> {player}
                </p>
                {consoleOutput.map((line, i) => (
                  <pre key={i} className={line.msgType}>
                    {line.msg}
                  </pre>
                ))}
              </div>
            </div>
          </div>
          <div className="editor">
            <Editor
              theme="vs-dark"
              loading="Loading Editor..."
              language={selectedLang}
              path={selectedLang}
              value={file}
              onChange={(code: string | undefined) =>
                code ? setFile(code) : null
              }
            />
          </div>
        </div>
      </main>
    </>
  );
}

export const getServerSideProps = requireSession(async (context, session) => {
  return { props: { username: session } };
});
