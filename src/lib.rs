use std::{collections::HashMap, i32}; //hashmap for key-values pairs

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


#[derive(Debug)]
pub enum Gate {
    Add(String, String, String, Option<i32>),
    Mul(String, String, String, Option<i32>),
    Sub(String, String, String, Option<i32>),    
    Xor(String, String, String),     
    Const(String, i32),
}

#[derive(Debug)] //for execution
pub struct Circuit { //public struct accessible by main 
    pub name: String, //name of circuit
    pub inputs: HashMap<String, i32>, //inputs to circuit
    pub outputs: HashMap<String, i32>, //outputs of circuit
    pub gates: Vec<Gate>, //logic
}

#[derive(Debug)]
pub struct Proof {
    pub circuit_name: String, //name of circuit of proof
    pub data: String, //proof data
}

pub struct Verifier;

impl Verifier {
    //logging the circuit being verified
    pub fn verify(circuit: &Circuit, proof: &Proof) -> bool {
        println!("Verifying proof for circuit: {}", circuit.name);
        circuit.name == proof.circuit_name && proof.data == "valid" //proof is valid if circuit name matches and data is valid
    }
}

impl Circuit {
    pub fn simulate(&self) -> HashMap<String, i32> {
        let mut values = self.inputs.clone();

        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, c, modulus) => {
                    if let (Some(x), Some(y)) = (values.get(a), values.get(b)) {
                        let result = mod_add(*x, *y, modulus.unwrap_or(1000));
                        values.insert(c.clone(), result);
                    }
                }
                Gate::Mul(a, b, c, modulus) => {
                    if let (Some(x), Some(y)) = (values.get(a), values.get(b)) {
                        let result = mod_mul(*x, *y,modulus.unwrap_or(1000));
                        values.insert(c.clone(), result);
                    }
                }
                Gate::Const(a, val) => {
                    values.insert(a.clone(), *val);
                }
                Gate::Sub(a,b ,c, modulus) => {
                    if let (Some(x), Some(y)) = (values.get(a), values.get(b)) {
                        let result = mod_add(*x, -(*y),modulus.unwrap_or(1000));
                        values.insert(c.clone(), result);
                    }
                }
                Gate::Xor(a,b ,c )=>{
                    if let (Some(x), Some(y)) = (values.get(a), values.get(b)){
                        values.insert(c.clone(), x^y);
                    }
                }
            }
        }

        values
    }
}

pub fn prove(circuit: &Circuit) -> Proof {
    println!("Proving circuit: {}", circuit.name);

    let simulated = circuit.simulate();

    for (key, expected) in &circuit.outputs {
        if simulated.get(key) != Some(expected) {
            return Proof {
                circuit_name: circuit.name.clone(),
                data: "invalid".to_string(),
            };
        }
    }
    Proof {
        circuit_name: circuit.name.clone(),
        data: "valid".to_string(),
    }
}

