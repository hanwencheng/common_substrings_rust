use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
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

impl Display for ResultSubstring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Substring(source: {:?}, substring: {}, weight: {})", self.source, self.substring, self.weight)
    }
}

//struct SubstringVector(Vec<ResultSubstring>);
//impl Display for SubstringVector {
//    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
//        let mut comma_separated = String::new();
//
//        for resultSubstring in &self.0[0..self.0.len() - 1] {
//            comma_separated.push_str(&resultSubstring.to_string());
//            comma_separated.push_str(", ");
//        }
//
//        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
//        write!(f, "{}", comma_separated)
//    }
//}

impl Node {
    pub fn add_source(&mut self, source_index: usize) {
        self.source.insert(source_index);
    }

    pub fn new(label: &str) -> Node {
        Node {
            source: HashSet::new(),
            listed: false,
            nodes: HashMap::new(),
            horizontal: HashMap::new(),
            label: String::from(label),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(source: {:?}, listed: {:?}), label: {}, horizontal: {:?} with Map {:?}", self.source, self.listed, self.label, self.horizontal, self.nodes)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(source: {:?}, listed: {:?}), label: {}, horizontal: {:?} with Map {:?}", self.source, self.listed, self.label, self.horizontal, self.nodes)
    }
}

struct Trie {
    root: Node
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/* main entry */
pub fn get_substrings(input: Vec<&str>) {
    let (mut built_trie, mut horizontal_root) = build_array(input);
}

fn accumulate_source(trie: &mut Node) -> HashSet<usize> {
    let accumulated_sources = trie.nodes.iter_mut().fold(trie.source.clone(), |acc: HashSet<usize>, (child_node_label, child_node)| {
        let child_sources = accumulate_source(child_node);
        return acc.union(&child_sources).cloned().collect();
    });
    if trie.source.len() != 0 {
        trie.source = accumulated_sources.clone();
    }
    return accumulated_sources;
}

fn accumulate_horizontal(horizontal_parent_node: &mut Node, result_vector: *mut Vec<ResultSubstring>) -> (HashSet<usize>) {
    println!("start horizontal is {:?}", horizontal_parent_node.horizontal);
    let mut children_resources = horizontal_parent_node.horizontal.iter_mut().fold(horizontal_parent_node.source.clone(), |acc: HashSet<usize>, (child_node_label, &mut child_node_pointer)| {
        unsafe {
            println!("start loop with node {} and node is {} ", child_node_label, (*child_node_pointer));
            let mut child_sources = accumulate_horizontal(&mut (*child_node_pointer), result_vector);
            let accumulated_sources = acc;
//            println!("getAccumulated value with vector {:?}", & *result_vector);
            let return_sources: HashSet<usize>;

            //merge sources
            let sources = accumulated_sources.union(&child_sources).cloned().collect();

            return sources;
        }
    });

    let has_different_leaves = horizontal_parent_node.horizontal.len() > 1;
    let has_different_sources = children_resources != horizontal_parent_node.source;
    println!("label {} , has_differentt_leaves {}, and has_different_sources {}, sources: {:?}", horizontal_parent_node.label, has_different_leaves, has_different_sources, children_resources);
    if has_different_sources | has_different_leaves {
        children_resources = children_resources.union(&horizontal_parent_node.source).cloned().collect();
        if children_resources.len() >= MIN_OCCURRENCES {
            unsafe {
                (*result_vector).push(ResultSubstring {
                    source: children_resources.clone(),
                    substring: String::from(&horizontal_parent_node.label),
                    weight: &horizontal_parent_node.label.chars().count() * children_resources.len(),
                })
            }
        }
    } else {
        //children_resource will be always more than child_sources
    }

    return children_resources;
}

fn build_array(input: Vec<&str>) -> (Node, Node) {
    let mut trie = Node::new("");
    let mut horizontal_trie_root: Node = Node::new("");
    let mut result_vector: *mut Vec<ResultSubstring> = &mut Vec::new() as *mut Vec<ResultSubstring>;
    for (word_index, word) in input.iter().enumerate() {
        println!("word: {}", word);
        build_suffices(&word, &mut trie, word_index, &mut horizontal_trie_root);
    }
    accumulate_source(&mut trie);
    println!("horizontal root is {}", horizontal_trie_root);
    println!("trie root is {}", trie);
    let result = accumulate_horizontal(&mut horizontal_trie_root, result_vector);
    unsafe {
        (*result_vector).iter().for_each(|it| {
            println!("{}", it);
        });
    }
    return (trie, horizontal_trie_root);
}

fn build_suffix(string: &str, trie: &mut Node, word_index: usize, last_suffix_leaf: *mut Node, suffix_first_letter_index: usize)
                -> *mut Node {
    let suffix = &string[suffix_first_letter_index..];
    let mut pointer = trie;
    println!("-start suffix {}", suffix);
    let mut iter = suffix.chars().enumerate();
    while let Some((index, char)) = iter.next() {
        println!("--start char: {}, at index {}", char, index);
        let suffix_substring_length = index + 1;
        let char_label = char.to_string();
        let insert_node = Node::new(&suffix[0..suffix_substring_length]);
        let contains_key = pointer.nodes.contains_key(&char_label);
        let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
        //if is is the end letter of the substring
        println!("--index {} + 1  === substring.len() {}", index, suffix.len());
        if suffix_substring_length == suffix.len() {
            println!("--last substring_index {} and substring is {}", suffix_first_letter_index, suffix);
            current_node.add_source(word_index);
            if suffix_first_letter_index >= 1 { // so that the last_suffix_leaf is not empty node
                unsafe {
                    let last_suffix_label = &string[(suffix_first_letter_index - 1)..];
                    current_node.horizontal.insert(String::from(last_suffix_label), last_suffix_leaf as *mut Node);
                    println!("--currentNode updated {} and last_suffix_leaf is {}", current_node, (*last_suffix_leaf))
                };
            }
        }
        pointer = current_node;
        println!("--finish char: {}, at index {}", char, index);
    }
    println!("-finish suffix {}", suffix);
    return pointer as *mut Node;
}

fn build_suffices(word: &str, trie: &mut Node, word_index: usize, horizontal_root: &mut Node) {
    if word.len() < MIN_LENGTH { return; }
    let mut last_suffix_leaf: *mut Node = &mut Node::new("") as *mut Node;
    for x in 0..(word.len() - MIN_LENGTH + 1) {
        last_suffix_leaf = build_suffix(&word, trie, word_index, last_suffix_leaf, x);
    }
    println!("last insert substring is {}", &word[(word.len() - MIN_LENGTH)..]);
    horizontal_root.horizontal.insert(String::from(&word[(word.len() - MIN_LENGTH)..]), last_suffix_leaf);
}
