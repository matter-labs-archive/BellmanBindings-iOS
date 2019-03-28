use std::fs::File;
use std::io::{BufReader, Read};
use std::error::Error;
use std::env;
use std::io::prelude::*;
use std::ffi::{CString, CStr};

use bellman::groth16::VerifyingKey;
use bellman::pairing::Engine;

pub extern fn get_verifying_key_from_file<E: Engine>(filename: String) -> Result<VerifyingKey<E>, Box<Error>> {
    println!("Start opening file");
    let file = match File::open(&filename) {
        Err(error) => {
            return Result::Err(Box::new(error))
        },
        Ok(string) => string,
    };
    println!("Opened file");
    let reader = BufReader::new(file);
    println!("Start reading");
    let verifying_key = match VerifyingKey::read(reader) {
        Err(error) => {
            return Result::Err(Box::new(error))
        },
        Ok(vk) => vk,
    };
    println!("VK is ready");
    Ok(verifying_key)
}

