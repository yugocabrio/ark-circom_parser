use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::path::PathBuf;


mod r1cs_reader;

use ark_ec::pairing::Pairing;

use ark_bn254::Bn254;

pub type Constraints<E> = (ConstraintVec<E>, ConstraintVec<E>, ConstraintVec<E>);
pub type ConstraintVec<E> = Vec<(usize, <E as Pairing>::ScalarField)>;

fn extract_constraints_from_r1cs<E>(filename: &str) -> Result<Vec<Constraints<E>>, Box<dyn Error>>
where E: ark_ec::pairing::Pairing
{
    // カレントディレクトリを取得
    let current_dir = std::env::current_dir()?;
    
    // パスを結合
    let filepath: PathBuf = [current_dir.to_str().unwrap(), filename].iter().collect();

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    
    let r1cs_file = r1cs_reader::R1CSFile::<E>::new(reader)?;

    let r1cs = r1cs_reader::R1CS::<E>::from(r1cs_file);


    Ok(r1cs.constraints)
}

fn main() {
    let filename = "src/toy_vitalik_example.r1cs";

    match extract_constraints_from_r1cs::<Bn254>(filename) {
        Ok(constraints) => {
            println!("Constraints extracted:");
            for constraint in constraints {
                println!("{:?}", constraint);
            }
        }
        Err(e) => {
            eprintln!("Error while extracting constraints: {:?}", e);
        }
    }
}
