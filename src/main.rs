use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn generate_tex(qubits: &Vec<String>, gates: &Vec<(String, String)>) {
    println!("generate tex");

    // The tex HashMap uses the qubit names as keys and all of the tex
    // on each line as the values. The qubit_order HashMap maintains
    // the order of the qubits in the circuit. This qubit_order structure
    // is needed to ensure multi-qubit gates are formatted properly.
    // The default HashMap structure does not track order, so we must
    // do this, implement our own hash structure, or add a dependency.
    
    //let mut tex: Vec<String> = Vec::new();
    let mut tex: HashMap<String, String> = HashMap::new();
    let mut qubit_order = HashMap::new();

    let mut order = 0;
    for qubit in qubits.iter() {
	let s = format!("{}{}{}", "\\lstick{\\ket{", qubit, "}}");
	tex.insert(qubit.to_string(), s);
	qubit_order.insert(qubit, order);
	order += 1;
    }

    for gate in gates.iter() {
	let g = &gate.0;
	let qtemp = &gate.1;
	let qs: Vec<&str> = qtemp.split(",").collect();

	if g == "h" {
	    let curr_str = tex.get(qtemp).unwrap();
	    *tex.get_mut(qtemp).unwrap() =  format!("{}{}", curr_str, " & \\gate{H}");

	    let mut keys: Vec<String> = Vec::new();
	    for (key, value) in &tex {
		if key != qtemp {
		    keys.push(key.to_string());
		}
	    }

	    for key in keys {
		*tex.get_mut(&key).unwrap() = format!("{}{}", tex.get(&key).unwrap(), " & \\qw");
	    }
	}

	// If g is not a single qubit gate, iterate through qs
	//for q in qs.iter() {
	//    
	//}
	
	//println!("{}", g);
	//println!("{:#?}", qs);
    }

    println!("{:#?}", tex);
    
    /*
    for qubit in qubits.iter() {
	let s = format!("{}{}{}", "\\lstick{\\ket{", qubit, "}} \\\\");
	tex.push(s)
    }
    
    println!("{}", "\\Qcircuit {");

    for line in tex.iter() {
	println!("{}", line);
    }
    
    println!("{}", '}');
     */
}

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

    generate_tex(&qubits, &gates);

    //let mut tex: HashMap<String, String> = HashMap::new();
    //tex.insert("q1".to_string(), "test".to_string());
    //let curr_str = tex.get("q1").unwrap();
}
