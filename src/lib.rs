use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

const MIN_LENGTH: usize = 3;
const MIN_OCCURRENCES: usize = 2;

struct Node {
    source: HashSet<usize>,
    listed: bool,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
    horizontal: HashMap<String, Rc<RefCell<Node>>>,
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

    pub fn set_listing(&mut self, shoud_listing: bool) {
        self.listed = shoud_listing;
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


fn list_sources(trie_ref: Rc<RefCell<Node>>,  result_vec: &mut Vec<ResultSubstring>) {
    trie_ref.borrow_mut().nodes.iter_mut().for_each(|(_child_node_label, child_node_ref)| {
        list_sources(child_node_ref.clone(), result_vec);
    
        if child_node_ref.borrow().listed == true {
            println!("child node is {}", child_node_ref.borrow());
            result_vec.push(ResultSubstring {
                source: child_node_ref.borrow().source.clone(),
                substring: String::from(child_node_ref.borrow().label.clone()),
                weight: child_node_ref.borrow().label.chars().count() * child_node_ref.borrow().source.len(),
            })
        }
    })
}

fn accumulate_vertical(trie_ref: &Rc<RefCell<Node>>) -> HashSet<usize> {
    let accumulated = trie_ref.borrow_mut().nodes.iter_mut().fold(HashSet::new(), |acc: HashSet<usize>, (_child_node_label, child_node_pointer)| {
        let child_sources = accumulate_vertical(child_node_pointer);
        return acc.union(&child_sources).cloned().collect();
    });

    let remained_occurrence:HashSet<usize> = trie_ref.borrow().source.difference(&accumulated).cloned().collect();
    if remained_occurrence.len() >= MIN_OCCURRENCES {
        trie_ref.borrow_mut().set_listing(true);
        trie_ref.borrow().source.clone()
    } else {
        accumulated
    }
}

fn accumulate_horizontal(horizontal_parent_node_ref: &Rc<RefCell<Node>>) -> HashSet<usize> {
    println!("start horizontal with label {:?}", horizontal_parent_node_ref.borrow().label);
    let accumulated = horizontal_parent_node_ref.borrow().horizontal.iter().fold(HashSet::new(), |acc: HashSet<usize>, (_child_node_label, child_node_pointer)| {
        unsafe {
            println!("start loop with node {} and node is {} ", _child_node_label, child_node_pointer.borrow());
            let child_sources = accumulate_horizontal(child_node_pointer);
            drop(child_node_pointer as &Rc<RefCell<Node>>);
            println!("child_sources source is {:?} for horizontal child label {}", child_sources,  _child_node_label);
            return acc.union(&child_sources).cloned().collect();
        }
    });

    let remained_occurrence:HashSet<usize> = horizontal_parent_node_ref.borrow().source.difference(&accumulated).cloned().collect();
    // println!("accumulated source is {:?} remained source is {:?} for node {}", accumulated, remained_occurrence, horizontal_parent_node.label);

    if remained_occurrence.len() >= MIN_OCCURRENCES {
        horizontal_parent_node_ref.borrow().source.clone()
    } else {
        horizontal_parent_node_ref.borrow_mut().set_listing(false);
       
        accumulated
    }
}

/* main entry */
pub fn get_substrings(input: Vec<&str>) {
    let trie = Rc::new(RefCell::new(Node::new("")));
    let horizontal_trie_root = Rc::new(RefCell::new(Node::new("")));
    let result_vector= &mut Vec::new();
    for (word_index, word) in input.iter().enumerate() {
        // println!("word: {}", word);
        build_suffices(&word, &trie, word_index,&horizontal_trie_root);
    }
    println!("horizontal is {} ", horizontal_trie_root.borrow());
    accumulate_vertical(&trie);

    accumulate_horizontal( &horizontal_trie_root);
    // println!("trie root is {}", trie);
    list_sources(trie, result_vector);
    unsafe {
        (*result_vector).iter().for_each(|it| {
            println!("{}", it);
        });
    }
}

// for each word add all the suffices into the trie
fn build_suffices(word: &str, trie: &Rc<RefCell<Node>>, word_index: usize, horizontal_root: &Rc<RefCell<Node>>) {
    let mut last_suffix_leaves: LinkedList<Rc<RefCell<Node>>> = LinkedList::new();
    if word.len() >= MIN_LENGTH {
        for x in 0..(word.len() - MIN_LENGTH + 1) {
            build_suffix(&word, trie, word_index, &mut last_suffix_leaves, x, horizontal_root);
        }
    }
    assert!(last_suffix_leaves.len() == 0);
}

// add one suffix of a word into trie, iterate each char of the suffix
fn build_suffix(word: &str, trie:  &Rc<RefCell<Node>>, word_index: usize, last_suffix_leaves: &mut LinkedList<Rc<RefCell<Node>>>, first_char_index: usize, horizontal_root:  &Rc<RefCell<Node>>){
    // let suffices_branch_length = word.len() - MIN_LENGTH + 1;
    let suffix = &word[first_char_index..];
    println!("suffix is {}", suffix);
    suffix.chars().enumerate().fold(trie.clone(), |pointer: Rc<RefCell<Node>>, (current_char_index, char)| {
        let current_branch_length = current_char_index + 1;
        let char_label = char.to_string();
        let current_branch_label = &suffix[0..current_branch_length];
        println!("start for branch : {}", current_branch_label);
        let insert_node = Node::new(current_branch_label);
        let mut pointer_node = pointer.borrow_mut();
        let current_node_ref = pointer_node.nodes.entry(char_label).or_insert(Rc::new(RefCell::new(insert_node)));
        //if is is the end letter of the substring
        if current_branch_length < MIN_LENGTH {
            return current_node_ref.clone();
        }

        //consume last verical
        if first_char_index >= 1 { // so that the last_suffix_leaves is not empty
            let suffix_label_in_last_branch = &word[(first_char_index - 1)..(first_char_index + current_branch_length)];
            unsafe {
                let last_ptr = last_suffix_leaves.pop_front().unwrap();
                current_node_ref.borrow_mut().horizontal.insert(String::from(suffix_label_in_last_branch), last_ptr);
                // println!("cosume pointer: {} and linked with {}", suffix_label_in_last_branch, last_ptr);
            }
        }
        
        current_node_ref.borrow_mut().add_source(word_index);
        let vertical_pointer = current_node_ref.clone();
        if current_branch_length > MIN_LENGTH{
            last_suffix_leaves.push_back(vertical_pointer);
            // println!("add pointer: {}",  current_node_ref);
        } else if current_branch_length == MIN_LENGTH {
            // if it is the last min length suffix of the whole word, then add it to root
            horizontal_root.borrow_mut().horizontal.insert(String::from(current_branch_label),vertical_pointer);
            // println!("add last pointer to root: {}, {:?}", current_node_ref.borrow().label, current_node_ref.borrow().horizontal);
        }
        current_node_ref.clone()
    });

    println!("-finish suffix {}", suffix);
}
