struct BoggleBoard {
    height: usize,
    width: usize,
    board: Vec<Vec<String>>,
}

struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
}

impl BoggleBoard {
    fn new(board: Vec<Vec<String>>) -> Self {
        let height = board.len();
        let width = board[0].len();
        BoggleBoard {
            height,
            width,
            board,
        }
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

    fn all_paths_iter(&self) -> std::slice::Iter<'_, Node<'_>> {
        todo!()
    }
}

pub trait WordDictionary {
    fn insert_node(&self);
    fn insert_word(&self);
    fn find(&self) -> Node;
    fn build(&self, board: BoggleBoard, max_depth: u32);
}

struct Node<'a> {
    value: &'a str,
    next: &'a Node<'a>,
}

struct Trie {}

impl WordDictionary for Trie {
    fn insert_node(&self) {
        todo!()
    }

    fn insert_word(&self) {
        todo!()
    }

    fn find(&self) -> Node {
        todo!()
    }

    fn build(&self, board: BoggleBoard, max_depth: u32) {
        todo!()
    }
}

struct RadixTrie {}
