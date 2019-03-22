use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use bellman::groth16::VerifyingKey;
use bellman::pairing::Engine;

pub extern fn get_verifying_key_from_file<E: Engine>(filename: String) -> Result<VerifyingKey<E>, Box<Error>> {
    let file = match File::open(&filename) {
        Err(error) => {
            return Result::Err(Box::new(error))
        },
        Ok(string) => string,
    };
    let reader = BufReader::new(file);
    let verifying_key = match VerifyingKey::read(reader) {
        Err(error) => {
            return Result::Err(Box::new(error))
        },
        Ok(vk) => vk,
    };
    Ok(verifying_key)
}

