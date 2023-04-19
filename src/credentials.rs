use anyhow;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{error, fmt, fs, io, path, result};

pub type AppResult<T> = result::Result<T, anyhow::Error>;

pub trait BufWriter {
    fn write(&self, p: &path::PathBuf) -> AppResult<()>;
}

pub trait FromUserPrompt {
    type T;
    fn from_prompt() -> Self::T;
}

/// Application credentials
#[derive(Serialize, Deserialize, Debug)]
pub struct Auth0M2MCredentials {
    client_id: String,
    client_secret: String,
    audience: String,
    domain: String,
    grant_type: String,
}

impl Auth0M2MCredentials {
    pub fn new(
        client_id: String,
        client_secret: String,
        audience: String,
        domain: String,
    ) -> Auth0M2MCredentials {
        Auth0M2MCredentials {
            client_id,
            client_secret,
            audience,
            domain,
            grant_type: "client_credentials".to_string(),
        }
    }
}

impl BufWriter for Auth0M2MCredentials {
    fn write(&self, p: &path::PathBuf) -> AppResult<()> {
        let mut clone = p.clone();
        clone.push("config.json");
        let mut w = io::BufWriter::new(fs::File::create(clone)?);
        serde_json::to_writer_pretty(&mut w, self)?;
        w.write(b"\n")?;
        w.flush()?;
        Ok(())
    }
}

#[derive(Debug)]
struct HomePathError {}
impl error::Error for HomePathError {}

impl fmt::Display for HomePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find home dir.")
    }
}

fn get_home_path(s: &str) -> AppResult<path::PathBuf> {
    if let Some(mut p) = home::home_dir() {
        p.push(s);
        return Ok(p);
    } else {
        Err(HomePathError {}.into())
    }
}

#[derive(Debug)]
pub struct HomePath {
    p: path::PathBuf,
}

impl HomePath {
    pub fn new(s: Option<&str>) -> AppResult<HomePath> {
        let val = s.unwrap_or(".auth0m2m");
        let p = get_home_path(val)?;
        Ok(HomePath { p })
    }

    pub fn home(&self) -> &path::PathBuf {
        return &self.p;
    }

    pub fn app_dir(&self, dir_name: &str) -> path::PathBuf {
        let mut clone = self.p.clone();
        clone.push(&dir_name);
        clone
    }
}

/// Reads in one line from the prompt and returns a Result<String>
fn do_prompt(p: &str) -> AppResult<String> {
    let mut out = String::new();
    print!("{}> ", p);
    io::stdout().flush()?;
    io::stdin().read_line(&mut out)?;
    Ok(out.trim().into())
}

impl FromUserPrompt for Auth0M2MCredentials {
    type T = AppResult<Auth0M2MCredentials>;

    fn from_prompt() -> AppResult<Auth0M2MCredentials> {
        let client_id = do_prompt("client_id")?;
        let client_secret = do_prompt("client_secret")?;
        let audience = do_prompt("audience")?;
        let domain = do_prompt("domain")?;

        Ok(Auth0M2MCredentials::new(
            client_id,
            client_secret,
            audience,
            domain,
        ))
    }
}

pub fn create_dir_if_not_exists(p: &path::PathBuf) -> AppResult<()> {
    fs::create_dir_all(p)?;
    Ok(())
}

pub fn check_path_exists(p: &path::PathBuf) -> bool {
    p.exists()
}

pub fn confirm(prompt: &str) -> AppResult<()> {
    print!("{} Continue? [y/N] ", prompt);
    io::stdout().flush()?;

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    let val = user_input.trim().to_ascii_lowercase();

    match &*val {
        "y" => Ok(()),
        _ => {
            eprintln!("Aborting.");
            std::process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use tempfile::TempPath;

    #[test]
    fn test_credentials() {
        let cred = Auth0M2MCredentials::new(
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        );
        let tmp_path = TempPath::from_path(Path::new("tmp.txt"));
        cred.write(&tmp_path.to_path_buf()).unwrap();
    }

    #[test]
    fn test_home_path_exists() {
        let home_path: HomePath = HomePath::new(None).expect("Home not working.");
        assert_eq!(false, check_path_exists(home_path.home()));

        let _ = home_path
            .home()
            .to_str()
            .unwrap()
            .find(".auth0m2m")
            .unwrap();
    }
}
