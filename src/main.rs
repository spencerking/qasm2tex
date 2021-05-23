use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "/Users/spencerking/Documents/qasm2tex/examples/test1.qasm";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut qubits: Vec<String> = Vec::new();
    let mut gates: Vec<(String, String)> = Vec::new();
    
    for (index, line) in reader.lines().enumerate() {
	let mut line = line.unwrap();
	
	// Ignore empty lines
	if line.is_empty() {
	    continue;
	}

	// Remove leading and trailing whitespace
	line = line.trim().to_string();
	
	// Ignore comment lines
	if line.chars().next().unwrap() == '#' {
	    continue;
	}

	// Strip comments in line
	let comment_pos = line.chars().position(|c| c == '#');
	if comment_pos != None {
	    line.replace_range(comment_pos.unwrap().., "");
	}

	let s: Vec<&str> = line.split_whitespace().collect(); //split('\t').collect();
	//s = s.iter().map(|x| x.trim()).collect();

	// qubits
	if s[0].to_string() == "qubit" {
	    //println!("qubit");
	    qubits.push(s[1].to_string());
	    //continue;
	} else {
	    gates.push((s[0].to_string(), s[1].to_string()));
	    //continue;
	}
	
	//println!("{}", line);
	//println!("\n");
    }
    
    for qubit in qubits.iter() {
	println!("{}", qubit);
    }

    for gate in gates.iter() {
	println!("{:#?}", gate);
    }
}
