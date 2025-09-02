import {
  GetServerSideProps,
  GetServerSidePropsContext,
  GetServerSidePropsResult,
} from "next";
import { API_URL, SESSION_COOKIE_NAME } from "./config";
import {AIError, GameState, MoveSequence, Scoreboard, SubmissionLanguage, TurnStatus} from "./models";

export function requireSession<T extends { [key: string]: any }>(
  onSuccess: (
    context: GetServerSidePropsContext,
    session: string
  ) => Promise<GetServerSidePropsResult<T>>,
  onFailure?: GetServerSideProps<T>
): GetServerSideProps<T> {
  return async (context) => {
    const session = context.req.cookies[SESSION_COOKIE_NAME];

    if (!session) {
      return onFailure !== undefined
        ? await onFailure(context)
        : { redirect: { destination: "/login", permanent: false } };
    } else {
      return await onSuccess(context, session);
    }
  };
}

async function apiCall(
  uri: string,
  { session, body, method }: { session?: string; body?: any; method?: string }
) {
  let headers = {};
  if (body !== undefined) {
    headers = { ...headers, "Content-Type": "application/json" };
  }
  if (session !== undefined) {
    headers = { ...headers, Authorization: `Bearer ${session}` };
  }

  return await fetch(`${API_URL}/${uri}`, {
    method: method || "GET",
    body:
    body ? (typeof body === "string" ? body : JSON.stringify(body)) : undefined,
    headers,
  });
}

export async function login(username: string): Promise<boolean> {
  return (
    await apiCall(`login?name=${username}`, {
      method: "POST",
    })
  ).ok;
}

export async function createGame(
  session: string,
  isFirstPlayer: boolean
): Promise<TurnStatus | AIError> {
  return await (
    await apiCall(`game/start?is_first_player=${isFirstPlayer}`, {
      session,
      method: "POST",
    })
  ).json();
}

export async function getGameState(
  session: string
): Promise<TurnStatus | AIError> {
  return await (await apiCall("game", { session, method: "GET" })).json();
}

export async function makeMove(
  moves: MoveSequence,
  session: string
): Promise<TurnStatus | AIError> {
  return await (
    await apiCall("game", { session, body: moves, method: "POST" })
  ).json();
}

export async function stopGame(session: string): Promise<void> {
  await apiCall("game/stop", { session, method: "POST" });
}

export async function loadSubmission(session: string): Promise<{
  lang: SubmissionLanguage;
  code: string;
}> {
  return await (await apiCall("submission", { session, method: "GET" })).json();
}

export async function getScoreboard(): Promise<{scores: Scoreboard}> {
    return await (await apiCall("tournament", { method: "GET", body: null })).json();
}

export async function submitCode(
  language: SubmissionLanguage,
  code: string,
  session: string
): Promise<void> {
  console.log(code);
  await apiCall(`submission?lang=${language}`, {
    session,
    body: code,
    method: "POST",
  });
}
