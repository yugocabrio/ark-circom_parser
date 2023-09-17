use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::path::PathBuf;
use ark_bn254::{Bn254, Fr};
use ark_ff::PrimeField;
use ark_ff::biginteger::BigInt;
use ark_ec::pairing::Pairing;
use ark_circom::circom::r1cs_reader;

// mod r1cs_reader;

pub type Constraints<E> = (ConstraintVec<E>, ConstraintVec<E>, ConstraintVec<E>);
pub type ConstraintVec<E> = Vec<(usize, <E as Pairing>::ScalarField)>;

fn bigint_to_bn254_scalar(bigint: BigInt<4>) -> Option<Fr> {
    Fr::from_bigint(bigint)
}

fn convert_constraints_bigint_to_scalar(constraints: Constraints<Bn254>) -> Constraints<Bn254> {
    let convert_vec = |vec: ConstraintVec<Bn254>| -> ConstraintVec<Bn254> {
        vec.into_iter()
            .filter_map(|(index, bigint)| {
                match bigint_to_bn254_scalar(bigint.into()) {
                    Some(scalar) => {
                        println!("Converted BigInt to Bn254 Scalar for index {}: {}", index, scalar);
                        Some((index, scalar))
                    },
                    None => {
                        println!("Failed to convert BigInt for index {}: {:?}", index, bigint);
                        None
                    }
                }
            })
            .collect()
    };

    (convert_vec(constraints.0), convert_vec(constraints.1), convert_vec(constraints.2))
}


fn extract_constraints_from_r1cs(filename: &str) -> Result<Vec<Constraints<Bn254>>, Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let filepath: PathBuf = [current_dir.to_str().unwrap(), filename].iter().collect();
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let r1cs_file = r1cs_reader::R1CSFile::<Bn254>::new(reader)?;
    let r1cs = r1cs_reader::R1CS::<Bn254>::from(r1cs_file);
    Ok(r1cs.constraints)
}

fn main() {
    let filename = "src/toy.r1cs";

    match extract_constraints_from_r1cs(filename) {
        Ok(constraints) => {
            println!("Original Constraints:");
            for constraint in &constraints {
                println!("{:?}", constraint);
            }

            println!("Converted Constraints:");
            for constraint in &constraints {
                let converted_constraint = convert_constraints_bigint_to_scalar(constraint.clone());
                println!("{:?}", converted_constraint);
            }
        },
        Err(e) => {
            eprintln!("Error while extracting constraints: {:?}", e);
        }
    }
}