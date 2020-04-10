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
* `min_leng` The minial length of the captured common substrings.

## Algorithm

So the key point is how to find the all the repeated substring and their occurence, and then we will try to find a method to list them.
Here I will start with an example with following 4 words: 

```javascript
let list = ["java", "javascript", "coffeescript", "python"]
```

A suffix trie is the start point for listing all the fragment
For example, we build the trie start with a string, and put the letters one by one in the trie, at the end leaf of the string, we record its source index number in the `list`:

![build_trie](./find-all-common-substrings/process1.png)

then for all the suffix of the string, i.e. `ava`, `va`, `a`, we also add them into the trie, and they have the same source index `1`. the end of the leaf are also linked one by one.

At the end the trie would be:

![add_suffices](./find-all-common-substrings/process2.png)

Then we do the same for all the other strings in the `list`, the built trie is as followings, all the dashed line linked with end of leaves has composite another trie, we call it `verticalTrie`.

![add_all_strings](./find-all-common-substrings/process3.png)

Since now only the longest substring has the source index number, we also need to add the source indexes to the short substrings for next calculation. For example in the first branch in the trie, `t` from `javascript` has source index `2`, but `java` also are included in `javascript`, so we will need to add `2` to the node `a` which is colored with yellow background. We do the same for each branch and then we got the following updated trie. 

![build_trie](./find-all-common-substrings/process4.png)

Now we got all the nodes we interested with source indexes. They are all include

![build_trie](./find-all-common-substrings/process5.png)
![build_trie](./find-all-common-substrings/process6.png)

## Other implementation

* [Javascript](https://github.com/hanwencheng/CommonSubstrings)

## License
Apache-2.0