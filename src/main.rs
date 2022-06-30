#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs;

mod huffman;
mod parse;

fn main() {
    let filename = "hello.ils";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let words = parse::parse(contents);
    println!("{:?}",words);
    let counts = huffman::generate_counts(words.clone());
    let recon = parse::combine(words);
    println!("{}",recon);
    println!("{:?}",counts);

    let tree = huffman::generate_tree(counts);
    println!("{:?}",tree)

}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/