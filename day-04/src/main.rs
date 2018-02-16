//
// To run part 1:
//     cat puzzle-input.txt | cargo run
//
// To run part 2:
//     cat puzzle-input.txt | cargo run --features part2
//

use std::io::{stdin, Read};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[cfg(feature = "part2")]
fn sort(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

#[cfg(not(feature = "part2"))]
fn sort(word: &str) -> String {
    // For part 1, do not sort
    word.to_string()
}

//
// Return true if each word in the passphrase is unique;
// otherwise, return false
//
fn check_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();

    let mut hm = HashMap::<String, bool>::new();

    for word in words {
        let sorted_word = sort(word);
        match hm.entry(sorted_word) {
            Entry::Occupied(_) => return false,
            Entry::Vacant(v) => v.insert(true),
        };
    }

    true
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let passphrases: Vec<&str> = input.lines().collect();

    let count = passphrases.iter().filter(|p| check_passphrase(p)).count();
    println!("count = {:?}", count);
}
