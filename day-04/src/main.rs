use std::io::{stdin, Read};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

//
// Return true if each word in the passphrase is unique;
// otherwise, return false
//
fn check_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();

    let mut hm = HashMap::<&str, bool>::new();

    for word in words {
        match hm.entry(word) {
            Entry::Occupied(_) => return false,
            Entry::Vacant(v)   => v.insert(true)
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
