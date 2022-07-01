
use std::collections::HashMap;
use bitvec::prelude::{BitVec,bitvec};

pub type Link = Option<Box<Node>>;

#[derive(Debug,Clone)]
pub struct Node {
    pub freq: i32,
    pub ch: Option<String>,
    pub left: Link,
    pub right: Link
}

impl Node {
    pub fn is_leaf(&self)->bool {
        self.left.is_some()
    }
}

pub fn new_node(freq: i32, ch: Option<String>) -> Node {
    Node {
        freq: freq, ch: ch,
        left: None, right: None,
    }
}

pub fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

pub fn frequency(s: Vec<String>) -> HashMap<String, u32> {
    let mut h = HashMap::new();
    for ch in s {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;  
    }
    h
}
 
pub fn assign_codes(p: &Box<Node>, h: &mut HashMap<String, BitVec>, s: BitVec ) {

    if let Some(ch) = p.ch.clone() {
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
			let mut sc = s.clone();
			sc.push(false);
            assign_codes(l, h, sc);
        }
        if let Some(ref r) = p.right {
			let mut sc = s.clone();
			sc.push(false);
            assign_codes(r, h, sc);
        }
    }
}
 
pub fn encode_string(s: Vec<String>, h: &HashMap<String, BitVec>) -> BitVec {
    let mut r = BitVec::new();
    let mut t:Option<&BitVec>;

    for ch in s {
        t = h.get(&ch);
        r.extend(t.unwrap().into_iter());
    }
    r
}
 
pub fn decode_string(s: BitVec, root: &Box<Node>) -> Vec<String> {

    let mut retval = Vec::new();
    let mut nodeptr = root;

    for x in s {
        if !x {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        } else {
            if let Some(ref r) = nodeptr.right {
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.ch.clone() {
            retval.push(ch);
            nodeptr = root;
        }
    }
    retval
}
 
/*
pub fn main() {
    let msg = "Huffman coding is fun!";
    let h = frequency(msg);

    let mut p:Vec<Box<Node>> = 
                      h.iter()
                      .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
                      .collect();
    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }

    let root = p.pop().unwrap();
    let mut h:HashMap<char, String> = HashMap::new();

    assign_codes(&root, &mut h, "".to_string()); 
    let enc = encode_string(msg, &h);
    println!("decoded = {:?}", decode_string(&enc, &root));
}
*/

#[cfg(test)]
mod tests {
	use bitvec::prelude::BitVec;
	use crate::parse;
	use crate::data;
	use super::*;
    #[test]
    fn test_bitvec() {
        let bv: BitVec = "000111000111000111000".chars().map(|v| v!='0').collect();
		let bv2: BitVec = Vec::<bool>::from([
			false,false,false,true,true,true,
			false,false,false,true,true,true,
			false,false,false,true,true,true,
			false,false,false
		]).into_iter().collect();
		assert_eq!(bv,bv2);
    }

	#[test]
	fn test_freq() {
		let freq = frequency(data::stringify(data::EXAMPLE_PARSED.to_vec()));
		let exp = data::stringify_hashmap(data::example_freq());
		println!("{:?}",freq)
	}

}