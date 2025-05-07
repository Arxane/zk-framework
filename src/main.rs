use std::env;
use zk_framework::{parse_circuit, Verifier, generate_keys}; // Make sure generate_keys is also imported

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: cargo run -- <path_to_circuit_file>");
    }

    let path = &args[1];
    let circuit = parse_circuit(path).expect("Failed to parse circuit");
    println!("Parsed Circuit: {:?}", circuit);

    // Generate proving and verifying keys
    let (pk, vk) = generate_keys(&circuit);

    let proof = circuit.prove(&pk);
    println!("Generated Proof: {:?}", proof);

    // Create Verifier instance
    let verifier = Verifier {};
    let is_valid = verifier.verify(&proof, &vk, &circuit);

    println!("Verification Result: {}", is_valid);
}
