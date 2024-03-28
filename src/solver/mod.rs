pub mod trie;

pub trait WordDictionary {
    fn insert(&self);
    fn get(&self) -> Options<Node>;
    fn build(&self, board: BoggleBoard, max_depth: u32);
}
