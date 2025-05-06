mod lib;
mod parser;

use lib::{prove, Verifier};
use parser::parse_circuit;

fn main() {
    let circuit = parse_circuit("circuit.zk").expect("Failed to parse circuit");
    
    println!("Parsed Circuit: {:?}", circuit);

    let proof = prove(&circuit);
    println!("Generated Proof: {:?}", proof);

    let is_valid = Verifier::verify(&circuit, &proof);
    println!("Verification Result: {}", is_valid);
}
