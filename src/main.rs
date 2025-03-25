mod cli_args;
mod generated;

use clap::Parser as ClapParser;
use log::info;
use std::{
    env, fs,
    io::{self, BufRead, BufReader, Read},
    path::PathBuf,
};

use ast2mdlib::{DataExtractor, Lexer};
use cli_args::{CliArgs, Command};

fn main() {
    let cli = CliArgs::parse();

    env::set_var("RUST_LOG", cli.level.to_string());
    env_logger::init();

    match cli.command {
        Command::Manual { filename: filepath } => {
            run_manual(filepath);
        }
        Command::Generated { filename: filepath } => {
            run_generated(filepath);
        }
    }
}

fn run_manual(filepath: Option<PathBuf>) {
    let reader: Box<dyn BufRead> = match filepath {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filepath) => Box::new(BufReader::new(fs::File::open(filepath).unwrap())),
    };

    let mut lexer = Lexer::new(reader.bytes());
    let tokens_and_error = lexer.tokenize();
    info!("Tokens: {tokens_and_error:?}\n\n***\n");

    let mut parser = ast2mdlib::Parser::new(tokens_and_error.0.into_iter());
    let result_tree = parser.create_dump_ast();
    info!("\n\n***\n\nDump AST tree: {result_tree:?}");
    if let Ok(ast_tree) = result_tree {
        let mut data_extractor = DataExtractor::new(ast_tree.nodes);
        let type_data = data_extractor.extract_data();
        print!("{}", serde_json::to_string(&type_data).unwrap());
    } else {
        eprintln!("Err: {:?}", result_tree.err());
    }
}

fn run_generated(filepath: Option<PathBuf>) {
    let reader: Box<dyn BufRead> = match filepath {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filepath) => Box::new(BufReader::new(fs::File::open(filepath).unwrap())),
    };
    generated::pest_parser(reader);
}
