use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fmt;

const MIN_LENGTH: usize = 3;
const MIN_OCCURRENCES: usize = 2;

struct Node {
    source: HashSet<usize>,
    listed: bool,
    nodes: HashMap<String, Node>
}

impl Node {
    pub fn new(source_index: usize) -> Node {
        let mut source:HashSet<usize> = HashSet::new();
        source.insert(source_index);
        Node {
            source,
            listed: false,
            nodes: HashMap::new()
        }
    }
    pub fn root() -> Node {
        Node {
            source: HashSet::new(),
            listed: false,
            nodes: HashMap::new()
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
    let mut trie= Node::root();
    for (word_index, word) in input.iter().enumerate() {
        println!("word: {}", word);
        build_string(&word, &mut trie, word_index);
    }
}

fn build_string(word: &str, trie: &mut Node, word_index: usize) {
    let mut pointer = trie;
    for x in 0 .. word.len() - MIN_LENGTH + 1 {
        let substring = &word[x..];
        println!("suffix is {}", substring);
        let mut iter = substring.chars().skip(MIN_LENGTH - 1).enumerate();
        while let Some((index, char)) = iter.next() {
            let char_label = char.to_string();
            let insert_node = Node::new(word_index);
            let contains_key = pointer.nodes.contains_key(&char_label);
            let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
            &current_node.source.insert(word_index);
            pointer = current_node;
            println!("char: {}, at index {}, {:?}", char, index + MIN_LENGTH, &pointer.source);
        }
    }
}
