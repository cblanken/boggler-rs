#[derive(Debug)]
pub struct Node {
    children: HashMap<char, Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    fn new(words: Vec<&str>) -> Self {
        Self { root: Node::new() }
    }
    fn insert(&self) {
        todo!()
    }

    fn insert_word(&self, word: &str) -> Node {
        let node = &self.root;
        if word.len() > 0 {
            return self.insert_word(&word[1..]);
        } else {
            return Node::new();
        }
    }

    fn delete(&self, s: &str) {
        todo!()
    }

    fn find(&self) -> Node {
        todo!()
    }
}

pub struct RadixTrie {}
