use std::fmt::Debug;

const MAX_BRANCHING_FACTOR: usize = 26;
const MAX_WORD_LEN: usize = 20;

pub trait WordTree<'nodes, T, U> {
    /// T = Node generic
    /// U = Word generic
    fn build<I>(tree: ArenaTrie<T>, words: I) -> Self
    where
        I: IntoIterator<Item = U>,
        U: IntoIterator<Item = T>,
        T: PartialEq + PartialOrd + PartialEq + Eq;
    fn find_word(&self, word: U) -> Option<Vec<T>>;
    fn add_word(&mut self, word: U);
    fn delete_word(&self, word: U);
}

#[derive(Debug)]
pub struct ArenaNode<T> {
    index: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> ArenaNode<T>
where
    T: PartialEq,
{
    /// The default children capacity of 26 accounts for the maximium possible branching factor of an English Trie
    /// This prevents children Vecs from ever needing to be resized
    ///
    /// TODO: the average/median branching factor should be calcualted from some dictionaries to
    /// get a better number for `Vec::with_capacity()` so as not to waste memory
    fn new(index: usize, value: T) -> Self {
        ArenaNode {
            index,
            value,
            parent: None,
            children: Vec::with_capacity(MAX_BRANCHING_FACTOR),
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct ArenaTrie<T> {
    arena: Vec<ArenaNode<T>>,
}

impl<T> ArenaTrie<T>
where
    T: PartialEq + Debug + Default,
{
    fn size(&self) -> usize {
        self.arena.len()
    }

    fn get_new_node_index(&self) -> usize {
        self.arena.len()
    }

    fn default() -> Self {
        let mut trie = ArenaTrie {
            arena: Vec::with_capacity(MAX_BRANCHING_FACTOR),
        };

        // Add root node
        // trie.arena[0] = ArenaNode::new(0, T::default());
        trie.arena.push(ArenaNode::new(0, T::default()));
        trie
    }

    /// Creates a new child node with the given `value` under `parent`
    /// If the `parent` doesn't exist, the node is considered the root and no
    /// parent is assigned to it. However if the `parent` does exist it will be
    /// assigned to the `parent` property of the new `ArenaNode`.
    ///
    /// Note: that if a child node already exists under the given `parent` with the given `value`,
    /// it's index will be returned instead of creating a new node. This makes sense in the context
    /// of a Trie where we never want to add duplicate nodes under a parent.
    fn add_node(&mut self, value: T, parent: Option<usize>) -> usize {
        if let Some(parent_idx) = parent {
            // Identify any existing nodes under the parent that match the given value
            let parent = &self.arena[parent_idx];
            for child_idx in &parent.children {
                if self.arena[*child_idx].value == value {
                    return *child_idx;
                }
            }

            // No child matches the given value, so create a new node
            let index = self.get_new_node_index();
            self.arena.push(ArenaNode::new(index, value));

            // Link to parent node
            self.arena[index].parent = Some(index);
            self.arena[parent_idx].children.push(index);
            return index;
        } else {
            // Root node
            let index = self.get_new_node_index();
            self.arena.push(ArenaNode::new(index, value));
            return index;
        }
    }
}

impl<'nodes, T, U> WordTree<'nodes, T, U> for ArenaTrie<T>
where
    T: Copy + Debug + Default + Eq + PartialEq + PartialOrd + PartialEq,
    U: IntoIterator<Item = T> + Debug,
{
    fn build<I>(tree: ArenaTrie<T>, words: I) -> Self
    where
        I: IntoIterator<Item = U>,
    {
        let mut trie = ArenaTrie::default();

        for w in words {
            trie.add_word(w);
        }

        return trie;
    }

    fn find_word(&self, word: U) -> Option<Vec<T>> {
        dbg!(&word);

        let mut matching_word: Vec<T> = Vec::with_capacity(MAX_WORD_LEN);

        let mut prev_idx = 0;
        let mut curr_idx = 0;
        for c in word {
            // dbg!(&self.arena[curr_idx].children);

            prev_idx = curr_idx;
            for idx in &self.arena[curr_idx].children {
                if c == self.arena[*idx].value {
                    curr_idx = *idx;
                    matching_word.push(c);
                    break;
                }
            }

            if curr_idx == prev_idx {
                // Matching node not found in children for previous lookup
                // `word` doesn't exist
                return None;
            }
        }

        Some(matching_word)
    }

    fn add_word(&mut self, word: U) {
        let mut parent_idx = 0;
        for c in word {
            let new_node_idx = self.add_node(c, Some(parent_idx));
            parent_idx = new_node_idx
        }
    }

    fn delete_word(&self, word: U) {
        todo!();
    }
}

pub struct RadixTrie {}
