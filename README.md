# md-word-count

`md-word-count` is a library and binary that will perform a word count on a Markdown file. It will strip out comments and symbols in order to produce a word count that matches common word processors.

## Usage

```rust
use std::io::{self, Read};

use md_word_count::count_words;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    println!("{}", count_words(&input));
}
```
