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
    pub fn add_source(&mut self, source_index: usize) {
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
        write!(f, "(source: {:?}, listed: {:?}), horizontal: {:?} with Map {:?}", self.source, self.listed, self.horizontal, self.nodes)
    }
}

struct Trie {
    root: Node
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/* main entry */
pub fn get_substrings (input: Vec<&str>) {
    let (mut built_trie, mut horizontal_root) = build_array(input);
}

fn accumulate_source(trie: &mut Node) -> HashSet<usize> {
    let accumulated_sources =  trie.nodes.iter_mut().fold(trie.source.clone(), |acc:HashSet<usize>, (child_node_label, child_node)| {
        let child_sources = accumulate_source(child_node);
        return acc.union(&child_sources).cloned().collect();
    });
    if trie.source.len() != 0 {
        trie.source = accumulated_sources.clone();
    }
    return accumulated_sources;
}

//fn accumulate_horizontal(node: &mut node) -> HashSet<usize> {
//
//}

fn build_array(input: Vec<&str>) -> (Node, HashMap<String, *mut Node>) {
    let mut trie = Node::new();
    let mut horizontal_trie_root: HashMap<String, *mut Node> = HashMap::new();
    for (word_index, word) in input.iter().enumerate() {
        println!("word: {}", word);
        build_suffices(&word, &mut trie, word_index, &mut horizontal_trie_root);
    }
    accumulate_source(&mut trie);
    println!("horizontal root is {:?}", horizontal_trie_root);
    println!("trie root is {:?}", trie);
    return (trie, horizontal_trie_root);
}

fn build_suffix(string: &str, trie: &mut Node, word_index: usize, last_suffix_leaf: *mut Node, substring_index: usize)
                -> *mut Node {
    let substring = &string[substring_index..];
    let mut pointer = trie;
    println!("suffix is {}", substring);
    let mut iter = substring.chars().skip(MIN_LENGTH - 1).enumerate();
    while let Some((index, char)) = iter.next() {
        let char_label = char.to_string();
        let insert_node = Node::new();
        let contains_key = pointer.nodes.contains_key(&char_label);
        let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
        if index + MIN_LENGTH == substring.len() {
            current_node.add_source(word_index);
            if substring_index > 0 {
                unsafe {
                    current_node.horizontal.insert(String::from(&string[(substring_index-1)..]), last_suffix_leaf as *mut Node);
                    println!("last_suffix_leaf updated {:?}", current_node)
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
    for x in 0..word.len() - MIN_LENGTH - 1 {
        last_suffix_leaf = build_suffix(&word, trie, word_index, last_suffix_leaf, x);
    }
    println!("last insert substring is {}",&word[(word.len() - MIN_LENGTH)..] );
    horizontal_root.insert(String::from(&word[(word.len() - MIN_LENGTH)..]), last_suffix_leaf);
}
