use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

pub mod board;
use board::BoggleBoard;

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
    println!("{:?}", args.board_file);

    println!("{:?}", args.dict_file);

    let board = BoggleBoard::read_board_file(args.board_file);
    dbg!(board.unwrap());

    Ok(())
}
