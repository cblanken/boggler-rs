pub enum NodeType<T>
where
    T: Copy,
{
    CompleteWord(T),
    IncompleteWord(T),
}
