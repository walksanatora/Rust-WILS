use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct Node {
	pub value: String,
	pub left_child: Option<Box<Node>>,
	pub right_child: Option<Box<Node>>,
	pub count: u32
}

pub fn generate_counts(words: Vec<String>) -> HashMap<String,u32> {
	let mut counts = HashMap::<String,u32>::new();

	for word in words{
		let stat = counts.entry(word).or_insert(0);
		*stat += 1;
	}

	return counts
}

pub fn generate_tree(counts: HashMap<String,u32>) -> Node {
	//step 1 create nodes for each value
	let mut nodes = Vec::<Node>::new();
	for obj in counts {
		nodes.push(
			Node { 
				value: obj.0, 
				left_child: None, 
				right_child: None,
				count: obj.1
			}
		)
	}
	//step 2 build huffman tree
	while nodes.len() > 1{
		println!("{}",nodes.len());
		nodes.sort_by(|s1, s2| s1.count.cmp(&s2.count));
		let left = nodes[0].clone();
		let right = nodes[1].clone();
		
		let new = Node {
			count: left.count + right.count,
			value: "".to_string(),
			left_child: Some(Box::new(left)),
			right_child: Some(Box::new(right))
		};

		nodes.push(new);
		nodes.remove(1);
		nodes.remove(0);
	}

	return nodes[0].clone();

}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_count() {
        let data = vec!["IF", "OP", "0", "VAR", "std.init", "\n", "IMPORT", "STR", "std", "\n", "ES", "\n", "CALL", "STR", "print", "STR", "Hello World!", "writes", "Hello", "World!", "\n", "\n", "\n", "DEF", "STR", "print_times", "STR", "str", "times", "\n", "LBL", "STR", "lp", "\n", "CALLN", "print", "VAR", "str", "\n", "MATH", "1", "times", "1", "\n", "IF", "OP", "0", "OP", "2", "times", "0", "\n", "GOTO", "STR", "lp", "\n", "ES", "\n", "ES", "\n", "\n", "CALL", "STR", "print_times", "STR", "Hello World!", "10", "\n", "EOF", "\n"];
		let mut to_count: Vec<String> = Vec::<String>::new();
		for word in data{to_count.push(word.to_string()) }; // convert Vec<str> to Vec<String>
		let exp = [("STR", 9),("1", 2),("writes", 1),("World!", 1),("GOTO", 1),("lp", 2),("ES", 3),("Hello World!", 2),("Hello", 1),("LBL", 1),("CALL", 2),("VAR", 2),("0", 3),
			("\n", 17), 
			("DEF", 1), 
			("CALLN", 1),
			("times", 3),
			("10", 1),
			("std.init", 1),
			("print_times", 2),
			("IF", 2),
			("str", 2),
			("2", 1),
			("OP", 3),
			("IMPORT", 1),
			("EOF", 1),
			("std", 1),
			("print", 2),("MATH", 1)].into_iter().map(|(key, value)| (key.to_owned(), value)).collect::<std::collections::HashMap<String, u32>>();
		
		let test = generate_counts(to_count);
		assert_eq!(test,exp)

    }
}