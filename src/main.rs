use std::collections::HashMap;

#[cfg(test)]
mod tests {
	#[test]
	fn open_and_decode_table_file() {
		let map = crate::create_decode_map("src/morse.txt");
		assert_eq!(*map.get("...").unwrap(), 's');
	}

	#[test]
	fn decode_morse() {
		let map = crate::create_decode_map("src/morse.txt");
		assert_eq!(crate::decode_morse(String::from("... --- .../... --- ..."), map), "sossos");
	}
}

fn create_decode_map(filepath: &str) -> HashMap<String, char>{
	let mut map: HashMap<String, char> = HashMap::new();
	let file = std::fs::read_to_string(filepath).expect("Failed to open file");
	
	for i in file.lines() {
		let mut j = i.split_whitespace();
		map.insert(j.next().expect("Failed to parse file").to_string(), j.next().expect("Failed to parse file").to_string().chars().next().unwrap());
	}
	map

}

fn decode_morse(code: String, hash_map: HashMap<String, char>) -> String {
	let mut out = String::from("");
	for i in code.split("/") {
		for j in i.split_whitespace() {
			out.push(*hash_map.get(j).expect("Failed to decode morse"));
		}
	}
	out
}

fn main() {
	let decoder: HashMap<String, char> = create_decode_map("morse.txt");
	println!("{}", decode_morse(String::from("... --- .../... --- ..."), decoder));

    println!("Hello, world!");
}
