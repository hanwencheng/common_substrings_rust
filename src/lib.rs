use std::collections::HashMap;

const MIN_LENGTH : u8 = 3;
const MIN_OCCURRENCES: u8 = 2;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn build_array(input: Vec<&str>) {
    for word in input.iter() {
        println!("{}", word);
        for c in word.chars() {
            // do something with `c
            println!("{}", c);
        }
    }
}

fn build_trie(input: Vec<&str>) {

}

struct Node {
    chars: HashMap<char, Node>,
    val: i32,
}

struct Trie {
    root: Node
}
