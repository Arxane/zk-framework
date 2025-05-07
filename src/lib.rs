use std::{collections::HashMap, i32}; //hashmap for key-values pairs
pub mod parser; // expose the file `parser.rs`
pub use parser::parse_circuit; // expose function directly

#[derive(Debug, Clone)]
pub struct R1CSConstraint{
    pub a: Vec<String>,
    pub b: Vec<String>,
    pub c: Vec<String>,

}

pub struct R1CSSystem {
    pub constraints: Vec<R1CSConstraint>, //list of R1CS constraints
}

pub fn mod_add(a: i32, b: i32, modulus: i32) -> i32 {
    (a + b) % modulus
}

pub fn mod_mul(a: i32, b: i32, modulus: i32) -> i32 {
    (a * b) % modulus
}

pub fn mod_inv(a: i32, modulus: i32) -> Option<i32> {
    for x in 1..modulus {
        if (a * x) % modulus == 1 {
            return Some(x);
        }
    }
    None
}

pub fn generate_keys(circuit: &Circuit) -> (ProvingKey, VerifyingKey) {
    (ProvingKey {}, VerifyingKey {})
}



#[derive(Debug)]
pub enum Gate {
    Add(String, String, String, Option<i32>),
    Mul(String, String, String, Option<i32>),
    Sub(String, String, String, Option<i32>),    
    Xor(String, String, String),     
    Const(String, i32),
    Hash(String, String), //Hash(input, output)
    Eq(String, String, String), //Eq(a,b,output_bool)
}

#[derive(Debug)] //for execution
pub struct Circuit { //public struct accessible by main 
    pub name: String, //name of circuit
    pub inputs: HashMap<String, i32>, //inputs to circuit
    pub outputs: HashMap<String, i32>, //outputs of circuit
    pub gates: Vec<Gate>, //logic
}



pub struct Verifier;


impl Circuit {
    pub fn to_r1cs_constraints(&self) -> R1CSSystem {
        let mut constraints = Vec::new(); // Initialize a vector to store constraints

        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, c, modulus) => {
                    let constraint = R1CSConstraint {
                        a: vec![a.clone(), b.clone()], // Variables for the addition (a + b)
                        b: vec![String::from("1"), String::from("1")], // Coefficients for addition (no scaling)
                        c: vec![c.clone()], // Result of the addition (c)
                    };
                    constraints.push(constraint); // Add the constraint to the vector
                }
                Gate::Mul(a, b, c, modulus) => {
                    let constraint = R1CSConstraint {
                        a: vec![a.clone(), b.clone()], // Variables for multiplication (a * b)
                        b: vec![String::from("1"), String::from("1")], // Coefficients for multiplication
                        c: vec![c.clone()], // Result of the multiplication (c)
                    };
                    constraints.push(constraint); // Add the constraint to the vector
                }
                Gate::Sub(a, b, c, modulus) => {
                    let constraint = R1CSConstraint {
                        a: vec![a.clone(), b.clone()], // Variables for subtraction (a - b)
                        b: vec![String::from("1"), String::from("-1")], // Coefficients for subtraction (a - b)
                        c: vec![c.clone()], // Result of the subtraction (c)
                    };
                    constraints.push(constraint); // Add the constraint to the vector
                }
                Gate::Eq(a, b, out) => {
                    let constraint = R1CSConstraint {
                        a: vec![a.clone(), b.clone()], // Variables for equality (a == b)
                        b: vec![String::from("1"), String::from("-1")], // Coefficients for equality (a == b)
                        c: vec![out.clone()], // Output of the equality check (0 or 1)
                    };
                    constraints.push(constraint); // Add the constraint to the vector
                }
                Gate::Hash(input, output) => {
                    let constraint = R1CSConstraint {
                        a: vec![input.clone()], // Input to the hash
                        b: vec![String::from("7")], // A fake hash multiplier (this is just a placeholder)
                        c: vec![output.clone()], // Output of the hash
                    };
                    constraints.push(constraint); // Add the constraint to the vector
                }
                _ => continue, // Skip unsupported gate types
            }
        }

        R1CSSystem { constraints } // Return the R1CS system with all constraints
    }
}

pub struct ProvingKey {

}

pub struct VerifyingKey {

}

impl R1CSSystem {
    pub fn generate_keys(&self) -> (ProvingKey, VerifyingKey) {
        let proving_key = ProvingKey{};
        let verifying_key = VerifyingKey{};
        (proving_key, verifying_key)
    }
}


#[derive(Debug)]
pub struct Proof {
    pub data: String,
}

impl Circuit {
    pub fn prove(&self, proving_key: &ProvingKey) -> Proof {
        Proof {
            data: "valid".to_string(),
        }
    }
}

impl Verifier {
    pub fn verify(&self, proof: &Proof, verifying_key: &VerifyingKey, circuit: &Circuit) -> bool {
        proof.data == "valid"
    }
}