use std::io::{self, Read};

use md_word_count::count_words;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    println!("{}", count_words(&input));
}
