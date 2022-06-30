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
    let recon = parse::combine(words.clone());
    println!("{}",recon);

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