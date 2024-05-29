use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;

/// A position on a 2D board
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
}

/// Wrapper for collection of cells in board
type BoardData = Vec<Vec<String>>;

/// Boggle board data
pub struct BoggleBoard {
    height: usize,
    width: usize,
    board: BoardData,
}

impl BoggleBoard {
    pub fn new(board: BoardData) -> Self {
        let height = board.len();
        let width = board[0].len();
        BoggleBoard {
            height,
            width,
            board,
        }
    }

    /// Builds a [BoggleBoard] from a board file
    pub fn build(filepath: &PathBuf) -> Self {
        let board_data = Self::read_board_file(filepath);
        Self::new(board_data.unwrap())
    }

    /// Read [BoardData] from file
    pub fn read_board_file(filepath: &PathBuf) -> Result<BoardData, io::Error> {
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

    /// Detect available cells adjacent to a given [Pos]
    fn get_adjacent_indexes(&self, pos: Pos) -> Vec<Pos> {
        let mut indexes: Vec<Pos> = vec![];

        if pos.row > 0 {
            indexes.push(Pos::new(pos.row - 1, pos.col)); // up
            if pos.col > 0 {
                indexes.push(Pos::new(pos.row - 1, pos.col - 1)); // up-left
            }
            if pos.col < self.width - 1 {
                indexes.push(Pos::new(pos.row - 1, pos.col + 1)); // up-right
            }
        }

        if pos.row < self.height - 1 {
            indexes.push(Pos::new(pos.row + 1, pos.col)); // down
            if pos.col > 0 {
                indexes.push(Pos::new(pos.row + 1, pos.col - 1)); // down-left
            }
            if pos.col < self.width - 1 {
                indexes.push(Pos::new(pos.row + 1, pos.col + 1)); // down-right
            }
        }

        if pos.col > 0 {
            indexes.push(Pos::new(pos.row, pos.col - 1)) // left
        }

        if pos.col < self.width - 1 {
            indexes.push(Pos::new(pos.row, pos.col + 1)) // right
        }

        indexes
    }
}
