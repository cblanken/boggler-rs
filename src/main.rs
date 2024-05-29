use boggler_rs::WordTree;
use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub mod board;
use board::BoggleBoard;

pub mod hash_trie;
use hash_trie::HashTrie;

pub mod arena_trie;
use arena_trie::ArenaTrie;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Boggle board file path
    #[arg(required = true)]
    board_file: PathBuf,

    /// Dictionary file path
    #[arg(required = true)]
    dict_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Load files
    let board = BoggleBoard::build(&args.board_file);
    println!("> Loaded board file: {:?}", args.board_file);

    let dict_file = fs::read_to_string(&args.dict_file)
        .expect(format!("Could not read file: {:?}", args.dict_file).as_str());

    let dict = dict_file.split_whitespace().map(|w| w.chars());

    println!("> Loaded words from dictionary file: {:?}", args.dict_file);

    // let arena_trie = ArenaTrie::build(dict);
    let hash_trie = HashTrie::build(dict);
    dbg!(hash_trie.find_word("aardvark".to_string().chars()));

    Ok(())
}
