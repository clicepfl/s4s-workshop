import { useRouter } from "next/router";
import { useCookies } from "react-cookie";
import { SESSION_COOKIE_NAME } from "../api/config";
import { useState } from "react";
import { login, requireSession } from "../api/api";

export default function Login() {
  const router = useRouter();
  const [cookies, setCookie] = useCookies([SESSION_COOKIE_NAME]);
  const [username, setUsername] = useState("");

  function attemptLogin(username: string) {
    if (username !== "") {
      login(username).then((code) => {
        if (code === 200) {
          setCookie(SESSION_COOKIE_NAME, username);
          router.push("/");
        } else if(code === 409) {
          alert("Username already taken");
        } else {
          alert(`Other error : ${code.toString()}`);
        }
      });
    }
  }

  return (
    <div className="login">
      <div className="login-form">
        <h1>Login</h1>
        <input
          type="text"
          placeholder="Username"
          onChange={(e) => setUsername(e.target.value)}
        />
        <button onClick={() => attemptLogin(username)}>Go</button>
      </div>
    </div>
  );
}

export const getServerSideProps = requireSession(
  async (c) => {
    return { redirect: { destination: "/", permanent: false } };
  },
  async (c) => {
    return { props: {} };
  }
);
