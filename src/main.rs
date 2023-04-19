use anyhow::Context;
use clap::{Parser, Subcommand};

mod credentials;

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

fn main() -> credentials::AppResult<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New { app } => execute_new(app),
        Commands::Login { app } => execute_login(app),
    }
}

fn execute_new(app: &String) -> credentials::AppResult<()> {
    let home_path = credentials::HomePath::new(None).with_context(|| "Home path not found.")?;
    let mut app_dir = home_path.app_dir(app);

    if credentials::check_path_exists(&app_dir) {
        credentials::confirm(&format!(
            "{} already exists. Continuing will overwrite your config.",
            app_dir.display()
        ))
        .with_context(|| "Failed during confirm.")?;
    }

    let creds = credentials::Auth0M2MCredentials::from_prompt()
        .with_context(|| "Read from prompt failed.")?;
    credentials::create_dir_if_not_exists(&app_dir).with_context(|| "Create dir failed.")?;

    app_dir.push("config.json");

    creds
        .to_json(&app_dir)
        .with_context(|| format!("Write to {} failed.", app_dir.display()))?;
    Ok(())
}

fn execute_login(app: &String) -> credentials::AppResult<()> {
    let home_path = credentials::HomePath::new(None).with_context(|| "Home path not found.")?;
    let mut app_dir = home_path.app_dir(app);

    if !credentials::check_path_exists(&app_dir) {
        eprintln!("{} does not exist.", app_dir.display());
        std::process::exit(1);
    }

    app_dir.push("config.json");

    let creds = credentials::Auth0M2MCredentials::from_json(&app_dir)
        .with_context(|| "Could not load json config file.")?;

    // This response needs some handling. Look up the deref context in reqwest.
    let token = creds.fetch().with_context(|| "Failed to fetch token.")?;

    println!("{}", serde_json::to_string(&token)?);

    Ok(())
}
