use clap::{Parser, Subcommand};

use apiconf::commands::keys;

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
    /// Add a key to an application
    Add {
        /// Application name
        app: String,
        /// Key name to add
        key: String,
    },
    /// Show application configuration
    Show {
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
        Commands::Apps { command: _ } => {
            eprintln!("Apps commands not yet implemented");
            std::process::exit(1);
        }
        Commands::Env { app: _ } => {
            eprintln!("Env command not yet implemented");
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}
