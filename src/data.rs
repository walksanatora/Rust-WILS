use std::collections::HashMap;

pub static EXAMPLE_CONTENTS: &str = "IF OP 0 VAR std.init #only load if std.init is unset\nIMPORT STR std\nES\nCALL STR print STR \"Hello World!\" writes Hello World!\n\n#this is a function which takes a string to output and a number of times to output it\nDEF STR print_times STR str times\nLBL STR lp\nCALLN print VAR str\nMATH 1 times 1\nIF OP 0 OP 2 times 0\nGOTO STR lp\nES\nES\n#print the string 10 times\nCALL STR print_times STR \"Hello World!\" 10\nEOF";
pub static EXAMPLE_WITHOUT_COMMENTS: &str = "IF OP 0 VAR std.init\nIMPORT STR std\nES\nCALL STR print STR Hello World! writes Hello World!\n\n\nDEF STR print_times STR str times\nLBL STR lp\nCALLN print VAR str\nMATH 1 times 1\nIF OP 0 OP 2 times 0\nGOTO STR lp\nES\nES\n\nCALL STR print_times STR Hello World! 10\nEOF";
pub static EXAMPLE_PARSED: [&str; 70] = ["IF", "OP", "0", "VAR", "std.init", "\n", "IMPORT", "STR", "std", "\n", "ES", "\n", "CALL", "STR", "print", "STR", "Hello World!", "writes", "Hello", "World!", "\n", "\n", "\n", "DEF", "STR", "print_times", "STR", "str", "times", "\n", "LBL", "STR", "lp", "\n", "CALLN", "print", "VAR", "str", "\n", "MATH", "1", "times", "1", "\n", "IF", "OP", "0", "OP", "2", "times", "0", "\n", "GOTO", "STR", "lp", "\n", "ES", "\n", "ES", "\n", "\n", "CALL", "STR", "print_times", "STR", "Hello World!", "10", "\n", "EOF", "\n"];
pub fn example_freq() -> HashMap<&'static str,u32>{
	HashMap::from(
		[
			("IMPORT", 1),
			("GOTO", 1),
			("std", 1),
 			("OP", 3),
 			("VAR", 2),
 			("DEF", 1),
 			("Hello", 1),
 			("print", 2),
 			("CALL", 2),
 			("0", 3),
 			("LBL", 1),
 			("\n", 17),
 			("print_times", 2),
 			("CALLN", 1),
 			("STR", 9),
 			("2", 1),
 			("std.init", 1),
 			("World!", 1),
 			("str", 2),
 			("times", 3),
 			("Hello World!", 2),
 			("lp", 2),
 			("1", 2),
 			("10", 1),
 			("IF", 2),
 			("ES", 3),
 			("MATH", 1),
 			("writes", 1),
 			("EOF", 1)
		]
	)
}

pub fn stringify(strs: Vec<&str>) -> Vec<String> {
	let mut output: Vec<String> = Vec::<String>::new();
	for word in strs{output.push(word.to_string()) }
	output
}

pub fn stringify_hashmap(map: HashMap<&str,u32>) -> HashMap<String,u32> {
	let mut new_map = HashMap::new();
	for (key, value) in map {
		new_map.insert(key.to_string(), value);
	}
	return new_map
}