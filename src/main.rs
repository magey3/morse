use std::collections::HashMap;
use structopt::StructOpt;

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

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str), short = "i", long = "input_file")]
	path: Option<std::path::PathBuf>,

	#[structopt(parse(from_os_str), short = "o", long = "output_file")]
	output_path: Option<std::path::PathBuf>,
}

fn read_from_stdin() -> String{
	let mut stdin = String::new();
	use std::io::BufRead;
	std::io::stdin().lock().read_line(&mut stdin).unwrap();
	stdin
}

fn main() {
	let args = Cli::from_args();
	let decoder: HashMap<String, char> = create_decode_map("morse.txt");
	
	let out = match args.path {
		Some(path) => decode_morse(std::fs::read_to_string(path).expect("Could not read input file"), decoder),
		None => decode_morse(read_from_stdin(), decoder),
	};
	
	match args.output_path {
		Some(path) => std::fs::write(path, out).expect("Could not write to output file"),
		None => println!("{}", out)
	}
}
