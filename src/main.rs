use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "/Users/spencerking/Documents/qasm2tex/examples/test1.qasm";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
	let line = line.unwrap();

	if line.is_empty() {
	    continue;
	}
	
	if line.chars().next().unwrap() == '#' {
	    continue;
	}
	
	println!("{}", line);
    }
}
