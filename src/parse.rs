use shlex;
pub fn parse(contents: String) -> Vec<String>{
	let mut output = Vec::<String>::new();

	for line in contents.lines() {
		let possible_words = shlex::split(line);
		match possible_words {
			None => { 
				// println!("line is empty or failed to parse")
			 }, 
			Some(words) => {
				for word in words{
					if !word.starts_with('#'){
						output.push(word);
					} else {
						break
					}
				}
				output.push("\n".to_string());
			}
		}
	}

	return output
}

pub fn combine(words: Vec<String>) -> String{
	let mut lines = Vec::<Vec<String>>::new();
	let mut line = Vec::<String>::new();
	for word in words {
		if word == "\n"{
			lines.push(line);
			line = Vec::<String>::new();
		}else{
			line.push(word);
		}
	}
	let mut output = Vec::<String>::new();
	for line in lines{
		output.push(line.join(" "))
	}

	return output.join("\n")
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_parse() {
		let con = "IF OP 0 VAR std.init #only load if std.init is unset\nIMPORT STR std\nES\nCALL STR print STR \"Hello World!\" writes Hello World!\n\n#this is a function which takes a string to output and a number of times to output it\nDEF STR print_times STR str times\nLBL STR lp\nCALLN print VAR str\nMATH 1 times 1\nIF OP 0 OP 2 times 0\nGOTO STR lp\nES\nES\n#print the string 10 times\nCALL STR print_times STR \"Hello World!\" 10\nEOF".to_string();
		let expected = vec!["IF", "OP", "0", "VAR", "std.init", "\n", "IMPORT", "STR", "std", "\n", "ES", "\n", "CALL", "STR", "print", "STR", "Hello World!", "writes", "Hello", "World!", "\n", "\n", "\n", "DEF", "STR", "print_times", "STR", "str", "times", "\n", "LBL", "STR", "lp", "\n", "CALLN", "print", "VAR", "str", "\n", "MATH", "1", "times", "1", "\n", "IF", "OP", "0", "OP", "2", "times", "0", "\n", "GOTO", "STR", "lp", "\n", "ES", "\n", "ES", "\n", "\n", "CALL", "STR", "print_times", "STR", "Hello World!", "10", "\n", "EOF", "\n"];
		let mut exp: Vec<String> = Vec::<String>::new();
		for word in expected{exp.push(word.to_string()) }
		let parsed = parse(con);
		assert_eq!(parsed,exp)
    }
	#[test]
	fn test_combine(){
		let data = vec!["IF", "OP", "0", "VAR", "std.init", "\n", "IMPORT", "STR", "std", "\n", "ES", "\n", "CALL", "STR", "print", "STR", "Hello World!", "writes", "Hello", "World!", "\n", "\n", "\n", "DEF", "STR", "print_times", "STR", "str", "times", "\n", "LBL", "STR", "lp", "\n", "CALLN", "print", "VAR", "str", "\n", "MATH", "1", "times", "1", "\n", "IF", "OP", "0", "OP", "2", "times", "0", "\n", "GOTO", "STR", "lp", "\n", "ES", "\n", "ES", "\n", "\n", "CALL", "STR", "print_times", "STR", "Hello World!", "10", "\n", "EOF", "\n"];
		let mut to_combine: Vec<String> = Vec::<String>::new();
		for word in data{to_combine.push(word.to_string()) };
		let expected = "IF OP 0 VAR std.init\nIMPORT STR std\nES\nCALL STR print STR Hello World! writes Hello World!\n\n\nDEF STR print_times STR str times\nLBL STR lp\nCALLN print VAR str\nMATH 1 times 1\nIF OP 0 OP 2 times 0\nGOTO STR lp\nES\nES\n\nCALL STR print_times STR Hello World! 10\nEOF".to_string();
		let combined = combine(to_combine);
		assert_eq!(combined,expected)
	}
}