use std::fs;

use clap::{command, Parser};
use walkdir::WalkDir;

use microsearch::search_engine::SearchEngine;

#[derive(Parser, Debug)]
#[command()]
struct Cli {
    folder: String,
}

fn main() {
    let mut search_engine = SearchEngine::default();
    let cli = Cli::parse();

    for entry in WalkDir::new(cli.folder).into_iter().filter_map(|e| e.ok()) {
        if let Ok(contents) = fs::read_to_string(entry.path()) {
            search_engine.index(entry.path().display().to_string(), contents);
        }
    }
}
