use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fmt;

const MIN_LENGTH: usize = 3;
const MIN_OCCURRENCES: usize = 2;

struct Node {
    source: HashSet<usize>,
    listed: bool,
    nodes: HashMap<String, Node>,
    horizontal: HashMap<String, *mut Node>,
}

impl Node {
    pub fn update(&mut self, source_index: usize) {
        self.source.insert(source_index);
    }

    pub fn new() -> Node {
        Node {
            source: HashSet::new(),
            listed: false,
            nodes: HashMap::new(),
            horizontal: HashMap::new(),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(source: {:?}, listed: {:?}), horizontal: {:?}", self.source, self.listed, self.horizontal)
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
    let mut trie = Node::new();
    let mut horizontal_trie_root: HashMap<String, *mut Node> = HashMap::new();
    for (word_index, word) in input.iter().enumerate() {
        println!("word: {}", word);
        build_suffices(&word, &mut trie, word_index, &mut horizontal_trie_root);
    }
    println!("horizontal root is {:?}", horizontal_trie_root);
}

fn build_suffix(substring: &str, trie: &mut Node, word_index: usize, last_suffix_leaf: *mut Node)
                -> *mut Node {
    let mut pointer = trie;
    println!("suffix is {}", substring);
    let mut iter = substring.chars().skip(MIN_LENGTH - 1).enumerate();
    while let Some((index, char)) = iter.next() {
        let char_label = char.to_string();
        let insert_node = Node::new();
        let contains_key = pointer.nodes.contains_key(&char_label);
        let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
        if index + MIN_LENGTH == substring.len() {
            current_node.update(word_index);
            if index != 0 {
                unsafe {
                    (*last_suffix_leaf).horizontal.insert(String::from(substring), current_node as *mut Node);
                    println!("last_suffix_leaf updated {:?}", *last_suffix_leaf)
                };
            }
        }
        pointer = current_node;
        println!("char: {}, at index {}", char, index + MIN_LENGTH);
    }

    return pointer as *mut Node;
}

fn build_suffices(word: &str, trie: &mut Node, word_index: usize, horizontal_root: &mut HashMap<String, *mut Node>) {
    if word.len() < MIN_LENGTH { return; }
    let mut last_suffix_leaf: *mut Node = &mut Node::new() as *mut Node;
    for x in 0..word.len() - MIN_LENGTH + 1 {
        last_suffix_leaf = build_suffix(&word[x..], trie, word_index, last_suffix_leaf);
    }
    horizontal_root.insert(String::from(&word[word.len() - 3..]), last_suffix_leaf);
}
