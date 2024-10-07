use clap::Parser;
use env_logger;
use env_logger::Env;
use log::error;
use log::info;
use std::fs;
mod interpreter;
mod parser;

/// Simple Brainfuck Interpreter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input Brainfuck file
    #[arg(short, long)]
    input: String,
    /// Output file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    // Initialize logger
    let env = Env::default()
        .filter_or("RUST_LOG_LEVEL", "trace")
        .write_style_or("RUST_LOG_WRITE_STYLE", "always");
    env_logger::init_from_env(env);

    // Parse command-line arguments
    let args = Args::parse();

    info!("Starting Brainfuck parser");

    // Read the input file
    let source = match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read input file: {}", e);
            return;
        }
    };

    // Lexing
    let tokens = parser::tokenize(&source);

    info!("Lexing completed successfully");

    interpreter::interpret(&tokens);
}
