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
    label: String,
}

#[derive(Clone)]
struct ResultSubstring {
    source: HashSet<usize>,
    substring: String,
    weight: usize,
}

impl Debug for ResultSubstring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(source: {:?}, substring: {:?}), weight: {:?}", self.source, self.substring, self.weight)
    }
}

impl Node {
    pub fn add_source(&mut self, source_index: usize) {
        self.source.insert(source_index);
    }

    pub fn new(label: String) -> Node {
        Node {
            source: HashSet::new(),
            listed: false,
            nodes: HashMap::new(),
            horizontal: HashMap::new(),
            label
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

fn accumulate_horizontal (horizontal_parent_node: &mut Node) -> (HashSet<usize>, Vec<ResultSubstring>) {

    let (mut children_resources, mut children_results) =  horizontal_parent_node.horizontal.iter_mut().fold((horizontal_parent_node.source.clone(), Vec::new()), |acc:(HashSet<usize>, Vec<ResultSubstring>), (child_node_label, &mut child_node_pointer)| {
        unsafe {
            let (mut child_sources, mut result_vector) = accumulate_horizontal(&mut (*child_node_pointer));
            let (accumulated_source,mut  accumulated_result) = acc;

            let return_sources: HashSet<usize>;
            let mut return_results: Vec<ResultSubstring> = result_vector.clone();

            //merge result vectors first
            return_results.append(&mut accumulated_result);

            //merge sources
            let sources = accumulated_source.union(&child_sources).cloned().collect();

            return (sources, return_results);
        }
    });

    let has_different_leaves = horizontal_parent_node.horizontal.len() != 1;
    let has_different_sources = children_resources != horizontal_parent_node.source;
    if  has_different_sources | has_different_leaves {
        children_resources = children_resources.union(&horizontal_parent_node.source).cloned().collect();
        if children_resources.len() >= MIN_OCCURRENCES {
            children_results.push(ResultSubstring {
                source: children_resources.clone(),
                substring: String::from(&horizontal_parent_node.label),
                weight: &horizontal_parent_node.label.chars().count() * children_resources.len()
            })
        }
    }else {
        //children_resource will be always more than child_sources
    }

    return (children_resources, children_results);
}

fn build_array(input: Vec<&str>) -> (Node, Node) {
    let mut trie = Node::new(String::from(""));
    let mut horizontal_trie_root: Node = Node::new(String::from(""));
    for (word_index, word) in input.iter().enumerate() {
        println!("word: {}", word);
        build_suffices(&word, &mut trie, word_index, &mut horizontal_trie_root);
    }
    accumulate_source(&mut trie);
//    println!("horizontal root is {:?}", horizontal_trie_root);
//    println!("trie root is {:?}", trie);
    let (_, result) = accumulate_horizontal(&mut horizontal_trie_root);
    println!("search result is {:?}", result);
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
        let insert_node = Node::new(String::from(substring));
        let contains_key = pointer.nodes.contains_key(&char_label);
        let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
        if index + MIN_LENGTH == substring.len() {
            current_node.add_source(word_index);
            if substring_index > 0 {
//                unsafe {
                    current_node.horizontal.insert(String::from(&string[(substring_index-1)..]), last_suffix_leaf as *mut Node);
                    println!("last_suffix_leaf updated {:?}", current_node)
//                };
            }
        }
        pointer = current_node;
        println!("char: {}, at index {}", char, index + MIN_LENGTH);
    }

    return pointer as *mut Node;
}

fn build_suffices(word: &str, trie: &mut Node, word_index: usize, horizontal_root: &mut Node) {
    if word.len() < MIN_LENGTH { return; }
    let mut last_suffix_leaf: *mut Node = &mut Node::new(String::from("")) as *mut Node;
    for x in 0..word.len() - MIN_LENGTH - 1 {
        last_suffix_leaf = build_suffix(&word, trie, word_index, last_suffix_leaf, x);
    }
    println!("last insert substring is {}",&word[(word.len() - MIN_LENGTH)..] );
    horizontal_root.horizontal.insert(String::from(&word[(word.len() - MIN_LENGTH)..]), last_suffix_leaf);
}
