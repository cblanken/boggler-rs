use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
}

type BoardData = Vec<Vec<String>>;

pub struct BoggleBoard {
    height: usize,
    width: usize,
    board: BoardData,
}

impl BoggleBoard {
    pub fn new(board: Vec<Vec<String>>) -> Self {
        let height = board.len();
        let width = board[0].len();
        BoggleBoard {
            height,
            width,
            board,
        }
    }

    pub fn read_board_file(filepath: PathBuf) -> Result<BoardData, io::Error> {
        let file_str = read_to_string(filepath)?;
        Ok(file_str
            .trim()
            .split('\n')
            .map(|line| {
                line.split(',')
                    .map(|cell| cell.trim().to_string())
                    .collect()
            })
            .collect())
    }

    fn get_adjacent_indexes(&self, row: usize, col: usize) -> Vec<Pos> {
        let mut indexes: Vec<Pos> = vec![];

        if row > 0 {
            indexes.push(Pos::new(row - 1, col)); // up
            if col > 0 {
                indexes.push(Pos::new(row - 1, col - 1)); // up-left
            }
            if col < self.width - 1 {
                indexes.push(Pos::new(row - 1, col + 1)); // up-right
            }
        }

        if row < self.height - 1 {
            indexes.push(Pos::new(row + 1, col)); // down
            if col > 0 {
                indexes.push(Pos::new(row + 1, col - 1)); // down-left
            }
            if col < self.width - 1 {
                indexes.push(Pos::new(row + 1, col + 1)); // down-right
            }
        }

        if col > 0 {
            indexes.push(Pos::new(row, col - 1)) // left
        }

        if col < self.width - 1 {
            indexes.push(Pos::new(row, col + 1)) // right
        }

        indexes
    }
}
