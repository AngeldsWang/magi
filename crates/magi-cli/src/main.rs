use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use magi_dsl::parse_agent;
use std::fs;

#[derive(Parser)]
#[command(name = "magi")]
#[command(about = "Magi Agent OS CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a .mi agent file and print its structure
    Parse {
        /// Path to the .mi file
        #[arg(short, long)]
        file: String,

        /// Output format: json or yaml
        #[arg(short = 'o', long, default_value = "json")]
        format: OutputFormat,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file, format } => {
            let contents = fs::read_to_string(file)?;
            let agent = parse_agent(&contents)?;

            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&agent)?),
                OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&agent)?),
            }
        }
    }

    Ok(())
}
