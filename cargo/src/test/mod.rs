use std::str;
use std::fs::File;
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use std::error::Error;
use std::marker::PhantomData;
use rand::{thread_rng, Rng};
use bellman::groth16::{
    Proof,
    VerifyingKey,
    prepare_verifying_key,
    verify_proof,
    generate_random_parameters,
    create_random_proof,
};
use bellman::pairing::Engine;
use bellman::pairing::ff:: {
    Field,
    PrimeField,
};
use bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};
use bellman::pairing::bn256::Bn256;
use bellman::pairing::bls12_381::Bls12;

// Just for now
use crate::helpers::circuits::*;
use crate::helpers::types_helpers::*;
use crate::filesystem::get_verifying_key_from_file;

#[repr(C)]
pub enum EngineType {
    Bls12,
    Bn256
}

#[repr(C)]
pub struct VerificationResult {
    value: bool,
    error: *mut c_char,
}

fn test_verify_with_certain_engine<E: Engine>(
    file_with_vk: *const c_char, 
    inputs_array: *const u8, 
    inputs_array_size: usize, 
    proof_vec: *const u8,
    proof_vec_size: usize) -> VerificationResult {
    // VK 
    let filename = match ptr_to_string(file_with_vk) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Error: wrong file name!".to_owned()).unwrap().into_raw() 
            }
        },
        Ok(result) => result,
    };
    let verifying_key = match get_verifying_key_from_file::<E>(filename) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Error: can\'t get file!".to_owned()).unwrap().into_raw()
            }
        },
        Ok(result) => result,
    };
    let pvk = prepare_verifying_key(&verifying_key);

    // Inputs
    let inputs_bytes = utf8_bytes_to_rust(inputs_array, inputs_array_size);
    let inputs_str = match str::from_utf8(inputs_bytes) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Error: wrong input string!".to_owned()).unwrap().into_raw()  
            }
        },
        Ok(result) => result,
    };
    let inputs = E::Fr::from_str(inputs_str).unwrap();

    // Proof
    let proof_vec_bytes = utf8_bytes_to_rust(proof_vec, proof_vec_size);
    let pv: Vec<u8> = proof_vec_bytes.iter().cloned().collect();
    let proof = Proof::read(&pv[..]).unwrap();

    // Verification
    let result = match verify_proof(
        &pvk,
        &proof,
        &[inputs]
    ) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Can\'t verify proof!".to_owned()).unwrap().into_raw()  
            }
        },
        Ok(result) => result,
    };

    VerificationResult {
        value: result,
        error: CString::new("".to_owned()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern fn test_verify(
    file_with_vk: *const c_char,
    inputs_array: *const u8,
    inputs_array_size: usize,
    engine: EngineType, 
    proof_vec: *const u8,
    proof_vec_size: usize) -> VerificationResult {
    match engine {
        EngineType::Bls12 => {
            println!("Bls12 curve");
            return test_verify_with_certain_engine::<Bls12>(
                file_with_vk,
                inputs_array,
                inputs_array_size,
                proof_vec, 
                proof_vec_size)
        },
        EngineType::Bn256 => { 
            println!("Bn256 curve");
            return test_verify_with_certain_engine::<Bn256>(file_with_vk,
                inputs_array,
                inputs_array_size, 
                proof_vec, 
                proof_vec_size)
        },
    };
}

