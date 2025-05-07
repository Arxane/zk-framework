use std::env;
use zk_framework::{parse_circuit, prove, Verifier}; // your lib.rs module

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: cargo run -- <path_to_circuit_file>");
    }

    let path = &args[1];
    let circuit = parse_circuit(path).expect("Failed to parse circuit");
    println!("Parsed Circuit: {:?}", circuit);

    let proof = prove(&circuit);
    println!("Generated Proof: {:?}", proof);

    let is_valid = Verifier::verify(&circuit, &proof);
    println!("Verification Result: {}", is_valid);
}
