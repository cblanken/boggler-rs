use boggler_rs::WordTree;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

const MAX_WORD_LEN: usize = 20;

/// Node for [HashTrie] trie variant
/// Noteably it utilizes a hashmap for associating parents to children nodes
#[derive(Debug, Default)]
pub struct HashNode<T> {
    value: T,
    children: HashMap<T, HashNode<T>>,
    is_word: bool,
}

impl<T> HashNode<T>
where
    T: PartialEq + Eq + Default + Hash,
{
    fn new(value: T, is_word: bool) -> Self {
        HashNode {
            value,
            children: HashMap::<T, HashNode<T>>::new(),
            is_word,
        }
    }

    fn default() -> Self {
        HashNode::new(T::default(), false)
    }

    /// Creates a new child node with the given `value` under `parent`
    fn add_child(&mut self, value: T, is_word: bool) -> &mut HashNode<T>
    where
        T: Copy,
    {
        self.children
            .entry(value)
            .or_insert(HashNode::new(value, is_word))
    }
}

#[derive(Debug, Default)]
pub struct HashTrie<T> {
    root: HashNode<T>,
    size: usize,
}

impl<T> HashTrie<T>
where
    T: Copy + PartialEq + Eq + Default + Hash,
{
    fn size(&self) -> usize {
        return self.size;
    }

    fn default() -> Self {
        HashTrie {
            root: HashNode::default(),
            size: 1,
        }
    }
}

impl<'nodes, T, U> WordTree<'nodes, T, U> for HashTrie<T>
where
    T: Copy + Debug + Default + Eq + PartialEq + PartialOrd + PartialEq + Hash,
    U: IntoIterator<Item = T> + Debug,
{
    fn build<I>(words: I) -> Self
    where
        I: IntoIterator<Item = U>,
    {
        let mut trie = HashTrie::default();

        for w in words {
            trie.add_word(w);
        }

        return trie;
    }

    fn find_word(&self, word: U) -> Option<Vec<T>> {
        let mut matching_word: Vec<T> = Vec::with_capacity(MAX_WORD_LEN);

        let mut curr_node = &self.root;
        for c in word {
            if let Some(child) = curr_node.children.get(&c) {
                curr_node = child;
                matching_word.push(curr_node.value)
            } else {
                return None;
            }
        }

        match curr_node.is_word {
            true => Some(matching_word),
            false => None,
        }
    }

    fn add_word(&mut self, word: U) {
        let mut curr_node = &mut self.root;
        for c in word {
            curr_node = curr_node.add_child(c, false)
        }

        curr_node.is_word = true;
    }

    fn delete_word(&self, word: U) {
        todo!();
    }
}

pub struct RadixTrie {}
