// accept string - return first word
use std::io;

fn first_word(s: &String) -> usize {
    // use as_bytes() to convert string into an array of bytes
    // so we can go through it element by element
    // and check for spaces
    let bytes = s.as_bytes();

    // create an iterator over the array of bytes using iter method
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // return space position
        }
    }

    s.len() // no space found - return index of whole word
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read line");

    let word = first_word(&s);
    let word = &s[..word];

    println!("first word is {}", word);
}
