mod compiler;
mod magi_spec;

use clap::{Parser, Subcommand};

use compiler::compile_from_dsl_file;

#[derive(Parser)]
#[command(name = "magictl")]
#[command(about = "Magi: Declarative AI Agent Framework", long_about = None)]
struct MagiCli {
    #[command(subcommand)]
    command: MagiCommand,
}

#[derive(Subcommand)]
enum MagiCommand {
    /// Validate and plan agent execution DAG
    Plan {
        #[arg(short, long)]
        file: String,
    },

    /// Execute the full agent workflow
    Run {
        #[arg(short, long)]
        file: String,
    },

    /// Trace a previously executed workflow
    Trace {
        #[arg(short, long)]
        run_id: String,
    },
}

fn main() {
    let cli = MagiCli::parse();

    match cli.command {
        MagiCommand::Plan { file } => {
            println!("[magi::plan] Planning from file: {}", file);
            // TODO: parse DSL, generate MagiSpec, validate
            match compile_from_dsl_file(&file) {
                Ok(spec) => println!("Compiled MagiSpec:\n{:#?}", spec),
                Err(e) => eprintln!("Compilation failed: {}", e),
            }
        }
        MagiCommand::Run { file } => {
            println!("[magi::run] Running agents from file: {}", file);
            // TODO: compile DSL to MagiSpec, run DAG engine
            match compile_from_dsl_file(&file) {
                Ok(spec) => match magi::runtime::engine::run(&spec) {
                    Ok(_) => println!("[magi::run] All agents executed successfully."),
                    Err(e) => eprintln!("[magi::run] Runtime error: {}", e),
                },
                Err(e) => eprintln!("Compilation failed: {}", e),
            }
        }
        MagiCommand::Trace { run_id } => {
            println!("[magi::trace] Tracing run ID: {}", run_id);
            // TODO: load trace log and pretty-print
        }
    }
}
