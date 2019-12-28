use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::fmt;

const MIN_LENGTH: usize = 3;
const MIN_OCCURRENCES: usize = 2;

struct Node {
    source: Option<Vec<usize>>,
    listed: Option<Vec<usize>>,
}

impl Node {
    pub fn new(char: &str, source_index: usize) -> Node {
        Node {
            source: Some(vec![source_index]),
            listed: Some(vec![]),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(source: {:?}, listed: {:?})", self.source, self.listed)
    }
}

struct Trie {
    root: Node
}

struct Structure {}

struct HorizontalStructure {}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn build_array(input: Vec<&str>) {
//    let mut stucture: Structure = {};
    let mut trie: BTreeMap<String, Node> = BTreeMap::new();
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
    for (word_index, word) in input.iter().enumerate() {
        println!("word: {}", word);
        build_string(&word, &mut trie, word_index);
    }
}

fn build_string(word: &str, trie: &mut BTreeMap<String, Node>, word_index: usize) {
    let mut iter = word.chars().skip(MIN_LENGTH - 1).enumerate();
    while let Some((index, char)) = iter.next() {
        let char_label = char.to_string();
        let insert_node = Node::new(&char_label, word_index);
        let insert_result = trie.insert(char_label, insert_node);
        if let Some(node) = insert_result {
            println!("char insert node {:?}", node);
        }
        println!("char: {}, at index {}", char, index + MIN_LENGTH);
    }
}
