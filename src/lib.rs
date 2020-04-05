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

fn list_sources(trie: &mut Node,  result_vec: &mut Vec<ResultSubstring>) {
    trie.nodes.iter_mut().for_each(|(_child_node_label, child_node)| {
        list_sources(child_node, result_vec);
        if child_node.listed {
            result_vec.push(ResultSubstring {
                source: child_node.source.clone(),
                substring: String::from(&child_node.label),
                weight: child_node.label.chars().count() * child_node.source.len(),
            })
        }
    })
}

fn accumulate_vertical(trie: &mut Node) -> HashSet<usize> {
    let diffrentiate_sources = trie.nodes.iter_mut().fold(trie.source.clone(), |acc: HashSet<usize>, (_child_node_label, child_node)| {
        let child_sources = accumulate_vertical(child_node);
        //if child is listed, 
        if child_sources.len() < MIN_OCCURRENCES {
            return acc;
        }
        return acc.difference(&child_sources).cloned().collect();
    });

    if diffrentiate_sources.len() >= MIN_OCCURRENCES {
        trie.listed = true;
        trie.source.clone()
    } else {
        HashSet::new()
    }
}

fn accumulate_horizontal(horizontal_parent_node: &mut Node) -> HashSet<usize> {
    // println!("start horizontal is {:?}", horizontal_parent_node.horizontal);
    let diffrentiate_sources = horizontal_parent_node.horizontal.iter_mut().fold(horizontal_parent_node.source.clone(), |acc: HashSet<usize>, (child_node_label, &mut child_node_pointer)| {
        unsafe {
            // println!("start loop with node {} and node is {} ", child_node_label, (*child_node_pointer));
            let child_sources = accumulate_horizontal(&mut (*child_node_pointer));
            if child_sources.len() < MIN_OCCURRENCES {
                return acc;
            }
            return acc.difference(&child_sources).cloned().collect();
        }
    });

    if diffrentiate_sources.len() < MIN_OCCURRENCES {
        horizontal_parent_node.listed = false;
        horizontal_parent_node.source.clone()
    } else {
        HashSet::new()
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

fn build_suffices(word: &str, trie: &mut Node, word_index: usize, horizontal_root: &mut Node) {
    if word.len() < MIN_LENGTH { return; }
    let suffices_branch_length = word.len() - MIN_LENGTH + 1;
    let mut last_suffix_leaves: LinkedList<*mut Node> = LinkedList::new();
    for x in 0..(suffices_branch_length - 1) {
        build_suffix(&word, trie, word_index, &mut last_suffix_leaves, x);
    }
}

fn build_suffix(string: &str, trie: &mut Node, word_index: usize, last_suffix_leaves:&mut std::collections::LinkedList<*mut Node>, suffix_first_letter_index: usize){
    let whole_suffix = &string[0..(MIN_LENGTH + suffix_first_letter_index)];
    println!("suffix is {}", suffix);
    let mut pointer = trie;
    // println!("-start suffix {}", suffix);
    let mut iter = suffix.chars().enumerate();
    while let Some((index, char)) = iter.next() {
        // println!("--start char: {}, at index {}", char, index);
        let suffix_substring_length = index + 1;
        let char_label = char.to_string();
        let insert_node = Node::new(&suffix[0..suffix_substring_length]);
        // let contains_key = pointer.nodes.contains_key(&char_label);
        let current_node = pointer.nodes.entry(char_label).or_insert(insert_node);
        //if is is the end letter of the substring
        

        //consume last verical
        if suffix_first_letter_index >= 1 { // so that the last_suffix_leaves is not empty
            let last_branch_label = &string[(suffix_first_letter_index - 1)..(suffix_first_letter_index + index)];
            unsafe {
                let last_ptr = last_suffix_leaves.pop_front().unwrap();
                current_node.horizontal.insert(String::from(last_branch_label), last_ptr);
            }
            println!("cosume first one {}", last_branch_label);
        }
        
        // println!("--index {} + 1  === substring.len() {}", index, suffix.len());
        current_node.add_source(word_index);
        if suffix_substring_length == suffix.len() {
            // println!("--last substring_index {} and substring is {}", suffix_first_letter_index, suffix);
        }
        if index > 0 {
            let vertical_pointer = current_node as *mut Node;
            last_suffix_leaves.push_back(vertical_pointer);
            println!("current label is {}", &suffix[0..suffix_substring_length]);
        }
        pointer = current_node;
    }
    println!("-finish suffix {}", suffix);
}
