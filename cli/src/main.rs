use clap::{Parser, Subcommand};

use apiconf::commands::{apps, env, keys};

#[derive(Parser)]
#[command(name = "apiconf")]
#[command(about = "Manage API keys and configuration for your applications")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage API keys
    Keys {
        #[command(subcommand)]
        command: KeysCommand,
    },
    /// Manage application profiles
    Apps {
        #[command(subcommand)]
        command: AppsCommand,
    },
    /// Export environment variables for an app
    Env {
        /// Application name
        app: String,
    },
}

#[derive(Subcommand)]
enum KeysCommand {
    /// Add a new API key
    Add {
        /// Provider name (e.g., anthropic, openai)
        provider: String,
        /// Custom key name (defaults to provider name)
        #[arg(short, long)]
        name: Option<String>,
        /// Overwrite existing key
        #[arg(short, long)]
        force: bool,
    },
    /// List all configured keys
    List,
    /// Remove an API key
    Remove {
        /// Key name to remove
        name: String,
    },
}

#[derive(Subcommand)]
enum AppsCommand {
    /// Create a new application profile
    Create {
        /// Application name
        name: String,
    },
    /// List all application profiles
    List,
    /// Add a provider key to an application
    Add {
        /// Application name
        app: String,
        /// Provider name (e.g., anthropic, openai)
        provider: String,
        /// Key name (defaults to provider name)
        #[arg(short, long)]
        key: Option<String>,
    },
    /// Show application configuration
    Show {
        /// Application name
        app: String,
    },
    /// Remove an application profile
    Remove {
        /// Application name
        app: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Keys { command } => match command {
            KeysCommand::Add {
                provider,
                name,
                force,
            } => keys::add(&provider, name.as_deref(), force),
            KeysCommand::List => keys::list(),
            KeysCommand::Remove { name } => keys::remove(&name),
        },
        Commands::Apps { command } => match command {
            AppsCommand::Create { name } => apps::create(&name),
            AppsCommand::List => apps::list(),
            AppsCommand::Add { app, provider, key } => apps::add(&app, &provider, key.as_deref()),
            AppsCommand::Show { app } => apps::show(&app),
            AppsCommand::Remove { app } => apps::remove(&app),
        },
        Commands::Env { app } => env::export(&app),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}
