pub trait WordTree<'nodes, T, U>
where
    U: IntoIterator<Item = T>,
    T: PartialEq + PartialOrd + PartialEq + Eq,
{
    /// T = Node generic
    /// U = Word generic
    fn build<I>(words: I) -> Self
    where
        I: IntoIterator<Item = U>;
    fn find_word(&self, word: U) -> Option<Vec<T>>;
    fn add_word(&mut self, word: U);
    fn delete_word(&self, word: U);
}
