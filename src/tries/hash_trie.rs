// pub use crate::{NodeType, TrieNode, WordTree};
pub use crate::tries::enums::NodeType;
pub use crate::tries::traits::{TrieNode, WordTree};
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

impl<T> TrieNode<T> for HashNode<T>
where
    T: PartialEq + Copy,
{
    fn value(&self) -> T {
        self.value
    }
}

#[derive(Debug, Default)]
pub struct HashTrie<T> {
    root: HashNode<T>,
}

impl<T> HashTrie<T>
where
    T: Copy + PartialEq + Eq + Default + Hash,
{
    fn size(&self) -> usize {
        self.size_of_subtree(&self.root) + 1
    }

    fn size_of_subtree(&self, node: &HashNode<T>) -> usize {
        let mut node_cnt = node.children.len();

        for n in node.children.values() {
            node_cnt += self.size_of_subtree(n);
        }

        return node_cnt;
    }

    fn default() -> Self {
        HashTrie {
            root: HashNode::default(),
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

    fn find_word(&self, word: U) -> Option<NodeType<T>> {
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
            true => Some(NodeType::CompleteWord(curr_node.value())),
            false => Some(NodeType::IncompleteWord(curr_node.value())),
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

#[cfg(test)]
mod hash_trie_tests {
    use super::*;

    #[test]
    fn test_size() {
        let trie = HashTrie::build(
            vec![
                "aardvark".to_string().chars(),
                "aardvarks".to_string().chars(),
                "aardwolves".to_string().chars(),
                "boarding".to_string().chars(),
            ]
            .into_iter(),
        );

        assert_eq!(24, trie.size())
    }

    #[test]
    fn test_find() {
        let trie = HashTrie::build(
            vec![
                "aardvark".to_string().chars(),
                "aardvarks".to_string().chars(),
                "aardwolves".to_string().chars(),
                "abandon".to_string().chars(),
                "abandons".to_string().chars(),
            ]
            .into_iter(),
        );

        assert!(trie
            .find_word("a".to_string().chars())
            .is_some_and(|nt| match nt {
                NodeType::IncompleteWord('a') => true,
                _ => false,
            }));
        assert!(trie
            .find_word("aardvar".to_string().chars())
            .is_some_and(|nt| match nt {
                NodeType::IncompleteWord('r') => true,
                _ => false,
            }));
        assert!(trie
            .find_word("aardvark".to_string().chars())
            .is_some_and(|nt| match nt {
                NodeType::CompleteWord('k') => true,
                _ => false,
            }));
        assert!(trie
            .find_word("abandons".to_string().chars())
            .is_some_and(|nt| match nt {
                NodeType::CompleteWord('s') => true,
                _ => false,
            }));
        assert!(trie.find_word("abandoning".to_string().chars()).is_none());
    }

    #[test]
    fn test_add_word() {
        let mut trie = HashTrie::build(vec!["a".to_string().chars()].into_iter());
        assert_eq!(2, trie.size());

        trie.add_word("bat".to_string().chars());
        assert_eq!(5, trie.size());

        trie.add_word("and".to_string().chars());
        assert_eq!(7, trie.size());
    }
}
