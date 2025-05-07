use std::fs::File; //for opening the file
use std::io::{self, BufRead}; //for reading the file
use std::path::Path; 
use std::collections::HashMap; //for storing inputs and outputs
use crate::{Gate, Circuit};

pub fn parse_circuit(file_path: &str) -> io::Result<Circuit> {
    //open a file and wrap it with a buf reader
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file); 
    //storing circuit parts
    let mut inputs = HashMap::new();
    let mut outputs = HashMap::new();
    let mut gates = Vec::new();
    //parsing line by line
    for line in reader.lines(){
        let line = line?; //unwraps or returns an error
        if line.starts_with("inputs:"){
            let input_str = &line[7..]; //remove inputs part
            for pair in input_str.split(','){
                let mut parts = pair.split('='); //split by =
                let var = parts.next().unwrap().trim().to_string(); //remove whitespacces and convert to string
                let value = parts.next().unwrap().trim().parse::<i32>().unwrap();
                inputs.insert(var, value);
            }
        } else if line.starts_with("outputs:"){
            //same as inputs
            let output_str = &line[8..];
            for pair in output_str.split(','){
                let mut parts = pair.split('=');
                let var = parts.next().unwrap().trim().to_string();
                let value = parts.next().unwrap().trim().parse::<i32>().unwrap();
                outputs.insert(var, value);
            }
        }  else if line.starts_with("gates:") {
            continue; // skip "gates:" header
        } else if !line.trim().is_empty() && !line.contains(':') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let gate_type = parts[0].to_lowercase();
            let gate = match gate_type.as_str() {
                "add" | "mul" | "sub" => {
                    let a = parts[1].to_string();
                    let b = parts[2].to_string();
                    let c = parts[3].to_string();
                    let modulus = parts.get(4).and_then(|&s| s.parse::<i32>().ok());
                    match gate_type.as_str() {
                        "add" => Gate::Add(a, b, c, modulus),
                        "mul" => Gate::Mul(a, b, c, modulus),
                        "sub" => Gate::Sub(a, b, c, modulus),
                        _ => continue,
                    }
                }
                "xor" => Gate::Xor(parts[1].to_string(), parts[2].to_string(), parts[3].to_string()),
                "hash" => Gate::Hash(parts[1].to_string(), parts[2].to_string()),
                "eq" => Gate::Eq(parts[1].to_string(), parts[2].to_string(), parts[3].to_string()),
                _ => continue,
            };
            gates.push(gate);
        }
           
    }
    Ok(Circuit {
        name: "parsed_circuit".to_string(),
        inputs,
        outputs,
        gates,
    })
}