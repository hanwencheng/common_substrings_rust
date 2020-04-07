use std::collections::{HashMap, HashSet, LinkedList};
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
        write!(f, "(source: {:?}, listed: {:?}), label: {}, horizontal: {:?} with Map \n   {:?}", self.source, self.listed, self.label, self.horizontal, self.nodes)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(source: {:?}, listed: {:?}), label: {}, horizontal: {:?} with Map \n   {:?}", self.source, self.listed, self.label, self.horizontal, self.nodes)
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

fn list_sources(trie: &mut Node,  result_vec: &mut Vec<ResultSubstring>) {
    trie.nodes.iter_mut().for_each(|(_child_node_label, child_node)| {
        list_sources(child_node, result_vec);
        if child_node.listed == true {
            println!("child node is {}", child_node);
            result_vec.push(ResultSubstring {
                source: child_node.source.clone(),
                substring: String::from(&child_node.label),
                weight: child_node.label.chars().count() * child_node.source.len(),
            })
        }
    })
}

fn accumulate_vertical(trie: &mut Node) -> HashSet<usize> {
    let accumulated = trie.nodes.iter_mut().fold(HashSet::new(), |acc: HashSet<usize>, (_child_node_label, child_node)| {
        let child_sources = accumulate_vertical(child_node);
        return acc.union(&child_sources).cloned().collect();
    });

    let remained_occurrence:HashSet<usize> = trie.source.difference(&accumulated).cloned().collect();
    if remained_occurrence.len() >= MIN_OCCURRENCES {
        trie.listed = true;
        trie.source.clone()
    } else {
        accumulated
    }
}

fn accumulate_horizontal(horizontal_parent_node: &mut Node) -> HashSet<usize> {
    println!("start horizontal with label {:?}", horizontal_parent_node.label);
    let accumulated = horizontal_parent_node.horizontal.iter_mut().fold(HashSet::new(), |acc: HashSet<usize>, (_child_node_label, &mut child_node_pointer)| {
        unsafe {
            println!("start loop with node {} and node is {} ", _child_node_label, (*child_node_pointer));
            let child_sources = accumulate_horizontal(&mut (*child_node_pointer));
            println!("child_sources source is {:?} for horizontal child label {}", child_sources,  _child_node_label);
            return acc.union(&child_sources).cloned().collect();
        }
    });

    let remained_occurrence:HashSet<usize> = horizontal_parent_node.source.difference(&accumulated).cloned().collect();
    // println!("accumulated source is {:?} remained source is {:?} for node {}", accumulated, remained_occurrence, horizontal_parent_node.label);
   

    if remained_occurrence.len() >= MIN_OCCURRENCES {
        horizontal_parent_node.source.clone()
    } else {
        horizontal_parent_node.listed = false;
       
        accumulated
    }
}

fn build_array(input: Vec<&str>) -> (Node, Node) {
    let mut trie = Node::new("");
    let mut horizontal_trie_root: Node = Node::new("");
    let result_vector= &mut Vec::new();
    for (word_index, word) in input.iter().enumerate() {
        // println!("word: {}", word);
        build_suffices(&word, &mut trie, word_index,&mut horizontal_trie_root);
    }
    println!("horizontal is {} ", horizontal_trie_root);
    accumulate_vertical(&mut trie);

    accumulate_horizontal(&mut horizontal_trie_root);
    // println!("horizontal root is {}", horizontal_trie_root);
    // println!("trie root is {}", trie);
    list_sources(&mut trie, result_vector);
    unsafe {
        (*result_vector).iter().for_each(|it| {
            println!("{}", it);
        });
    }
    return (trie, horizontal_trie_root);
}

// for each word add all the suffices into the trie
fn build_suffices(word: &str, trie: &mut Node, word_index: usize, horizontal_root: &mut Node) {
    let mut last_suffix_leaves: LinkedList<*mut Node> = LinkedList::new();
    if word.len() >= MIN_LENGTH {
        for x in 0..(word.len() - MIN_LENGTH + 1) {
            build_suffix(&word, trie, word_index, &mut last_suffix_leaves, x, horizontal_root);
        }
    }
    assert!(last_suffix_leaves.len() == 0);
}

// add one suffix of a word into trie, iterate each char of the suffix
fn build_suffix(word: &str, trie: &mut Node, word_index: usize, last_suffix_leaves:&mut std::collections::LinkedList<*mut Node>, first_char_index: usize, horizontal_root: &mut Node){
    // let suffices_branch_length = word.len() - MIN_LENGTH + 1;
    let suffix = &word[first_char_index..];
    println!("suffix is {}", suffix);
    let mut pointer = trie;
    let mut iter = suffix.chars().enumerate();
    while let Some((current_char_index, char)) = iter.next() {
       
        let current_branch_length = current_char_index + 1;
        let char_label = char.to_string();
        let current_branch_label = &suffix[0..current_branch_length];
        println!("start for branch : {}", current_branch_label);
        let insert_node = Node::new(current_branch_label);
        // let contains_key = pointer.nodes.contains_key(&char_label);
        let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
        //if is is the end letter of the substring
        if current_branch_length < MIN_LENGTH {
            pointer = current_node;
            continue;
        }

        //consume last verical
        if first_char_index >= 1 { // so that the last_suffix_leaves is not empty
            let suffix_label_in_last_branch = &word[(first_char_index - 1)..(first_char_index + current_branch_length)];
            unsafe {
                let last_ptr = last_suffix_leaves.pop_front().unwrap();
                // current_node.horizontal.entry(String::from(suffix_label_in_last_branch)).or_insert(last_ptr);
                current_node.horizontal.insert(String::from(suffix_label_in_last_branch), last_ptr);
                println!("cosume pointer: {} and linked with {}", suffix_label_in_last_branch, &mut(*last_ptr));
            }
            
        }
        
        current_node.add_source(word_index);
        let vertical_pointer = current_node as *mut Node;
        if current_branch_length > MIN_LENGTH{
            last_suffix_leaves.push_back(vertical_pointer);
            println!("add pointer: {}", current_node.label);
        } else if current_branch_length == MIN_LENGTH {
            // if it is the last min length suffix of the whole word, then add it to root
            // horizontal_root.horizontal.entry(String::from(current_branch_label)).or_insert(vertical_pointer);
            horizontal_root.horizontal.insert(String::from(current_branch_label),vertical_pointer);
            println!("add last pointer to root: {}, {:?}", current_node.label, current_node.horizontal);
        }
        
        pointer = current_node;
    }
    println!("-finish suffix {}", suffix);
}
