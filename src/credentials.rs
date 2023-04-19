use anyhow;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{error, fmt, fs, io, path, result};

pub type AppResult<T> = result::Result<T, anyhow::Error>;

/// Application credentials
#[derive(Serialize, Deserialize, Debug)]
pub struct Auth0M2MCredentials {
    client_id: String,
    client_secret: String,
    audience: String,
    domain: String,
    grant_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    access_token: String,
    token_type: String,
}

impl Token {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

impl Auth0M2MCredentials {
    pub fn new(client_id: String, client_secret: String, audience: String, domain: String) -> Self {
        Self {
            client_id,
            client_secret,
            audience,
            domain,
            grant_type: "client_credentials".to_string(),
        }
    }

    pub fn to_json(&self, p: &path::PathBuf) -> AppResult<()> {
        let mut w = io::BufWriter::new(fs::File::create(p)?);
        serde_json::to_writer_pretty(&mut w, self)?;
        w.write(b"\n")?;
        w.flush()?;
        Ok(())
    }

    pub fn from_json(p: &path::PathBuf) -> AppResult<Self> {
        let f = fs::File::open(p)?;
        let rdr = io::BufReader::new(f);
        let obj: Auth0M2MCredentials = serde_json::from_reader(rdr)?;
        Ok(obj)
    }

    pub fn fetch(&self) -> AppResult<Token> {
        let url = format!("https://{}/oauth/token", self.domain);

        let client = reqwest::blocking::Client::new();
        let response = client.post(&url).json(self).send()?;

        match response.error_for_status_ref() {
            Ok(_) => (),
            Err(err) => {
                return Err(err.into());
            }
        }

        let token: Token = response.json()?;
        Ok(token)
    }

    pub fn from_prompt() -> AppResult<Auth0M2MCredentials> {
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
        cred.to_json(&tmp_path.to_path_buf()).unwrap();
    }

    #[test]
    fn test_home_path_exists() {
        let _: HomePath = HomePath::new(None).expect("Home not working.");
    }
}
