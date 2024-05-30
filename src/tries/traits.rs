use crate::tries::enums::NodeType;

pub trait WordTree<'nodes, T, U>
where
    U: IntoIterator<Item = T>,
    T: PartialEq + PartialOrd + PartialEq + Eq + Copy,
{
    /// T = Node generic
    /// U = Word generic
    fn build<I>(words: I) -> Self
    where
        I: IntoIterator<Item = U>;
    fn find_word(&self, word: U) -> Option<NodeType<T>>;
    fn add_word(&mut self, word: U);
    fn delete_word(&self, word: U);
}

pub trait TrieNode<T>
where
    T: PartialEq + Copy,
{
    fn value(&self) -> T;
}
