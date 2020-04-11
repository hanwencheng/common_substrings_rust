# Find all common substrings

A method for finding all common strings, particularly quick for large string samples. It only use the Rust `std` library.

The algorithms uses a two dimension trie to get all the fragment. The vertical one is the standard suffix trie, but all the node of the last word in each suffix is linked, which I call them virtually horizontally linked.

## Usage

Use the function `get_substrings` to get all the common strings in the strings list,

### Example
```rust
use common_substrings::get_substrings;
let input_strings = vec!["java", "javascript", "typescript", "coffeescript", "coffee"];
let result_substrings = get_substrings(input_strings, 2, 3);
```

which gives the result list of 
```shell
Substring(sources: {2, 3}, name: escript, weight: 14)
Substring(sources: {1, 0}, name: java, weight: 8)
Substring(sources: {4, 3}, name: coffee, weight: 12)
```

### Arguments

* `input` - The target input string vector.
* `min_occurrences` The minimal occurrence of the captured common substrings.
* `min_length` The minimal length of the captured common substrings.

## Algorithm

Explanation [here](https://github.com/hanwencheng/gists/blob/master/find-all-common-substrings.md)

## Other implementations

* [Javascript](https://github.com/hanwencheng/CommonSubstrings)

## License
Apache-2.0
