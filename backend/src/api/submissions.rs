use crate::config::config;

use super::{AppState, Error, User};
use async_process::{Child, Command};
use base64::{prelude::BASE64_STANDARD, Engine};
use rocket::{
    get, post,
    serde::json::Json,
    tokio::{
        fs::{self, File},
        io::{AsyncReadExt, AsyncWriteExt},
    },
};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fmt::Display,
    fs::{read_dir, remove_file},
    path::PathBuf,
    process::Stdio,
    str::FromStr,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Cpp,
    Java,
    Python,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Language::Cpp => "cpp",
            Language::Java => "java",
            Language::Python => "python",
        })
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpp" => Ok(Self::Cpp),
            "java" => Ok(Self::Java),
            "python" => Ok(Self::Python),
            _ => Err(Error::InvalidLanguage),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Submission {
    pub name: String,
    pub lang: Language,
    pub code: PathBuf,
}

impl Submission {
    pub fn empty(name: String) -> Result<Self, Error> {
        let path = PathBuf::from_str(format!("{}/{}", config().data_dir, name).as_str()).unwrap();

        std::fs::File::create(path.clone())?;

        Ok(Self {
            name,
            lang: Language::Cpp,
            code: path,
        })
    }

    pub async fn start(&self, socket_adr: String) -> Result<Child, Error> {
        let metadata = fs::metadata(self.code.clone()).await;
        if metadata.is_err() || metadata.is_ok_and(|m| m.len() == 0) {
            return Err(Error::AIFailed {
                error: super::AIError::EmptySubmission,
                ai_output: "".to_owned(),
                move_: None,
            });
        }

        let base_code = {
            let mut code = String::new();
            File::open(self.code.clone())
                .await?
                .read_to_string(&mut code)
                .await?;

            BASE64_STANDARD.encode(code.as_bytes())
        };

        let image = config().runner_image.as_str();
        let command = match self.lang {
            Language::Cpp => 
                format!("echo {base_code} | base64 -d > /base.hpp && cp /deps/cpp/* / && g++ /main.cpp -o /a.out && /a.out"),
            // The additional directory is required because of java packages
            Language::Java => 
                format!("mkdir /s4s && echo {base_code} | base64 -d > /s4s/Base.java && cp /deps/java/* /s4s/ && javac s4s/*.java && java s4s.Main"),
            Language::Python => 
                format!("echo {base_code} | base64 -d > /script.py && cp /deps/python/* / && python /main.py"),
        };

        let socket_arg = "SOCK=".to_owned() + &socket_adr;
        Command::new("docker")
            .args([
                "run",
                "--rm",
                "-u",
                "root",
                "-e",
                &socket_arg,
                "-v",
                &format!("{}:{}", config().socks_vol, config().socks_dir),
                "-i",
                image,
                "sh",
                "-c",
                command.as_str(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(Error::from)
    }
}

#[derive(Serialize, Debug)]
pub struct SubmissionStatus {
    code: String,
    lang: Language,
}

#[get("/submission")]
pub async fn get_submission(user: User, state: &AppState) -> Result<Json<SubmissionStatus>, Error> {
    let submission = {
        let lock = state.lock()?;
        lock.submissions
            .get(&user.name)
            .ok_or(Error::NotFound)?
            .clone()
    };

    let mut code = String::new();
    File::open(submission.code)
        .await?
        .read_to_string(&mut code)
        .await?;

    Ok(Json(SubmissionStatus {
        code,
        lang: submission.lang,
    }))
}

#[post("/submission?<lang>", data = "<code>")]
pub async fn post_submission(
    user: User,
    lang: String,
    code: &[u8],
    state: &AppState,
) -> Result<(), Error> {
    let lang = Language::from_str(lang.as_str())?;

    let path = PathBuf::from_str(format!("{}/{}.{}", config().data_dir, user.name, lang).as_str())
        .unwrap();

    // This doesn't guarantee removal if other file descriptors are open
    read_dir(&config().data_dir)
        .unwrap()
        .filter(|f| {
            let f = f.as_ref().unwrap().file_name();
            f == OsString::from(user.name.clone() + ".python")
                || f == OsString::from(user.name.clone() + ".cpp")
                || f == OsString::from(user.name.clone() + ".java")
        })
        .filter(|f| f.as_ref().unwrap().path() != path)
        .for_each(|f| remove_file(f.unwrap().path()).expect("could not delete old submission"));

    File::create(path.clone())
        .await
        .inspect_err(|e| println!("{e:#?}"))?
        .write_all(code)
        .await
        .inspect_err(|e| println!("{e:#?}"))?;

    state.lock().unwrap().submissions.insert(
        user.name.clone(),
        Submission {
            name: user.name,
            lang,
            code: path,
        },
    );

    Ok(())
}
