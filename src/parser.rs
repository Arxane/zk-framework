use std::fs::File; //for opening the file
use std::io::{self, BufRead}; //for reading the file
use std::collections::HashMap; //for storing inputs and outputs
use crate::{Gate, Circuit};

pub fn parse_circuit(file_path: &str) -> io::Result<Circuit> {
    // Open the file and wrap it with a buf reader
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Storing circuit parts
    let mut inputs = HashMap::new();
    let mut outputs = HashMap::new();
    let mut gates = Vec::new();

    // Parsing line by line
    for line in reader.lines() {
        let line = line?; // unwraps or returns an error

        // Parse the inputs
        if line.starts_with("inputs:") {
            let input_str = &line[7..].trim(); // Remove "inputs:" and trim whitespace
            for pair in input_str.split(',') {
                let mut parts = pair.split('=');
                if let (Some(var), Some(value_str)) = (parts.next(), parts.next()) {
                    let var = var.trim().to_string();
                    match value_str.trim().parse::<i32>() {
                        Ok(value) => {
                            inputs.insert(var, value);
                        }
                        Err(_) => {
                            eprintln!("Error: Failed to parse input value for '{}'", var);
                        }
                    }
                }
            }
        } else if line.starts_with("outputs:") {
            // Parse the outputs
            let output_str = &line[8..].trim(); // Remove "outputs:" and trim whitespace
            for pair in output_str.split(',') {
                let mut parts = pair.split('=');
                if let (Some(var), Some(value_str)) = (parts.next(), parts.next()) {
                    let var = var.trim().to_string();
                    match value_str.trim().parse::<i32>() {
                        Ok(value) => {
                            outputs.insert(var, value);
                        }
                        Err(_) => {
                            eprintln!("Error: Failed to parse output value for '{}'", var);
                        }
                    }
                }
            }
        } else if line.starts_with("gates:") {
            continue; // Skip "gates:" header
        } else if !line.trim().is_empty() && !line.contains(':') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let gate_type = parts[0].to_lowercase();
            let gate = match gate_type.as_str() {
                "add" | "mul" | "sub" => {
                    let a = parts.get(1).unwrap_or(&"").to_string();
                    let b = parts.get(2).unwrap_or(&"").to_string();
                    let c = parts.get(3).unwrap_or(&"").to_string();
                    let modulus = parts.get(4).and_then(|&s| s.parse::<i32>().ok());
                    match gate_type.as_str() {
                        "add" => Gate::Add(a, b, c, modulus),
                        "mul" => Gate::Mul(a, b, c, modulus),
                        "sub" => Gate::Sub(a, b, c, modulus),
                        _ => continue,
                    }
                }
                "xor" => {
                    if parts.len() >= 4 {
                        Gate::Xor(parts[1].to_string(), parts[2].to_string(), parts[3].to_string())
                    } else {
                        eprintln!("Error: Invalid XOR gate format");
                        continue;
                    }
                }
                "hash" => {
                    if parts.len() >= 3 {
                        Gate::Hash(parts[1].to_string(), parts[2].to_string())
                    } else {
                        eprintln!("Error: Invalid Hash gate format");
                        continue;
                    }
                }
                "eq" => {
                    if parts.len() >= 4 {
                        Gate::Eq(parts[1].to_string(), parts[2].to_string(), parts[3].to_string())
                    } else {
                        eprintln!("Error: Invalid EQ gate format");
                        continue;
                    }
                }
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
