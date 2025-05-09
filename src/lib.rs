use std::collections::{HashMap, HashSet}; //hashmap for key-values pairs
pub mod parser; // expose the file `parser.rs`
pub use parser::parse_circuit; // expose function directly

#[derive(Debug, Clone)]
pub struct R1CSConstraint {
    pub a: HashMap<usize, i32>,
    pub b: HashMap<usize, i32>,
    pub c: HashMap<usize, i32>,
}

fn get_index(var: &str, var_index: &mut HashMap<String, usize>, next_index: &mut usize) -> usize {
    if let Some(&idx) = var_index.get(var) {
        idx
    } else {
        let idx = *next_index;
        var_index.insert(var.to_string(), idx);
        *next_index += 1;
        idx
    }
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
    pub fn to_r1cs_constraints(&self, var_index:&mut HashMap<String, usize>, next_index: &mut usize) -> R1CSSystem {
        let mut constraints = Vec::new(); // Initialize a vector to store constraints

        var_index.insert("1".to_string(),0);

        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, c, modulus) => {
                    let a_idx = get_index(a, var_index, next_index);
                    let b_idx = get_index(b, var_index, next_index);
                    let c_idx = get_index(c, var_index, next_index);

                    constraints.push(R1CSConstraint {
                        a: vec![(a_idx, 1), (b_idx, 1)].into_iter().collect(),
                        b: vec![(var_index["1"], 1)].into_iter().collect(),
                        c: vec![(c_idx, 1)].into_iter().collect(),
                    }) // Add the constraint to the vector
                }
                Gate::Mul(a, b, c, modulus) => {
                    let a_idx = get_index(a, var_index, next_index);
                    let b_idx = get_index(b, var_index, next_index);
                    let c_idx = get_index(c, var_index, next_index);

                    constraints.push(R1CSConstraint {
                        a: vec![(a_idx, 1)].into_iter().collect(),
                        b: vec![(b_idx, 1)].into_iter().collect(),
                        c: vec![(c_idx, 1)].into_iter().collect(),
                    }); 
                }
                Gate::Sub(a, b, c, modulus) => {
                    let a_idx = get_index(a, var_index, next_index);
                    let b_idx = get_index(b, var_index, next_index);
                    let c_idx = get_index(c, var_index, next_index);

                    constraints.push(R1CSConstraint {
                        a: vec![(a_idx, 1), (b_idx, -1)].into_iter().collect(),
                        b: vec![(var_index["1"], 1)].into_iter().collect(),
                        c: vec![(c_idx, 1)].into_iter().collect(),
                    }); 
                }
                Gate::Eq(a, b, out) => {
                    let a_idx = get_index(a, var_index, next_index);
                    let b_idx = get_index(b, var_index, next_index);
                    let out_idx = get_index(out, var_index, next_index);

                    constraints.push(R1CSConstraint {
                        a: vec![(a_idx, 1), (b_idx, -1)].into_iter().collect(),
                        b: vec![(var_index["1"], 1)].into_iter().collect(),
                        c: vec![(out_idx, 1)].into_iter().collect(),
                    }); 
                }
                Gate::Hash(input, output) => {
                    let input_idx = get_index(input, var_index, next_index);
                    let output_idx = get_index(output, var_index, next_index);
                    constraints.push(R1CSConstraint {
                        a: vec![(input_idx, 1)].into_iter().collect(),
                        b: vec![(var_index["1"], 7)].into_iter().collect(), // Fake hash with multiplier
                        c: vec![(output_idx, 1)].into_iter().collect(),
                    }); 
                }
                Gate::Const(name,val ) => {
                    let idx = get_index(name, var_index, next_index);
                    constraints.push(R1CSConstraint {
                        a: vec![(var_index["1"],1)].into_iter().collect(),
                        b: vec![(var_index["1"],1)].into_iter().collect(),
                        c: vec![(idx, *val)].into_iter().collect(),
                    });
                }
                Gate::Xor(a,b ,c ) => {
                    let a_idx = get_index(a, var_index, next_index);
                    let b_idx = get_index(b, var_index, next_index);
                    let ab_idx = get_index(&format!("{}_{}",a,b), var_index, next_index);
                    let c_idx = get_index(c, var_index, next_index);

                    constraints.push(R1CSConstraint {
                        a: vec![(a_idx,1)].into_iter().collect(),
                        b: vec![(b_idx,1)].into_iter().collect(),
                        c: vec![(ab_idx, 1)].into_iter().collect(),
                    });
                    constraints.push(R1CSConstraint {
                        a: vec![(a_idx,1),(b_idx,1),(ab_idx,-2)].into_iter().collect(),
                        b: vec![(var_index["1"],1)].into_iter().collect(),
                        c: vec![(c_idx,1)].into_iter().collect(),
                    });
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
    pub fn to_matrices(
        &self,
        _var_index: &mut HashMap<String, usize>,
        _next_index: &mut usize,
    )-> (
        Vec<HashMap<usize, i32>>,
        Vec<HashMap<usize, i32>>,
        Vec<HashMap<usize, i32>>,
    ) {
        let mut a_matrix = Vec::new();
        let mut b_matrix = Vec::new();
        let mut c_matrix = Vec::new();

        for constraint in &self.constraints {
            a_matrix.push(constraint.a.clone());
            b_matrix.push(constraint.b.clone());
            c_matrix.push(constraint.c.clone());
        }
        (a_matrix, b_matrix, c_matrix)
    }
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

