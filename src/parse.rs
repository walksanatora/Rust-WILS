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
	use crate::data;
    #[test]
    fn test_parse() {
		let con = data::EXAMPLE_CONTENTS.to_string();
		let expected = data::stringify(data::EXAMPLE_PARSED.to_vec());
		let parsed = parse(con);
		assert_eq!(parsed,expected)
    }
	#[test]
	fn test_combine(){
		let data = data::stringify(data::EXAMPLE_PARSED.to_vec());
		let expected = data::EXAMPLE_WITHOUT_COMMENTS.to_string();
		let combined = combine(data);
		assert_eq!(combined,expected)
	}
}