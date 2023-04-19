use anyhow;
use clap::{Parser, Subcommand};

mod credentials;
use credentials::{AppResult, BufWriter, FromUserPrompt};

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new credential set.")]
    New {
        /// The name of the credential set to Create.
        app: String,
    },
    #[command(about = "Log yourself in to a configured application. Prints a token to stdout.")]
    Login {
        /// The name of the application to login with
        app: String,
    },
}

#[derive(Parser)]
#[command(version, about = "Auth0 M2M access token managment")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> AppResult<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New { app } => execute_new(app),
        Commands::Login { app } => execute_login(app),
    }
}

fn execute_new(app: &String) -> AppResult<()> {
    let home_path = credentials::HomePath::new(None)?;
    let app_dir = home_path.app_dir(app);
    let creds = credentials::Auth0M2MCredentials::from_prompt()?;
    credentials::create_dir_if_not_exists(&app_dir)?;
    creds.write(&app_dir)?;
    Ok(())
}

fn execute_login(app: &String) -> AppResult<()> {
    Ok(())
}
