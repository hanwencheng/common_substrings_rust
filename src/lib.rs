use std::collections::{HashMap, BTreeMap};

const MIN_LENGTH: usize = 3;
const MIN_OCCURRENCES: u32 = 2;

struct Node {
    chars: HashMap<char, Node>,
    val: i32,
    horizontal: usize,
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
    let mut movie_reviews= BTreeMap::new();
    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
    for word in input.iter() {
        println!("word: {}", word);
        build_string(&word);
    }
}

pub fn build_string(word: &str) {
    let mut iter = word.chars().skip(MIN_LENGTH - 1).enumerate();
    while let Some((index, char)) = iter.next() {
        println!("char: {}, at index {}", char, index + MIN_LENGTH);
    }
}

fn build_trie(input: Vec<&str>) {}



