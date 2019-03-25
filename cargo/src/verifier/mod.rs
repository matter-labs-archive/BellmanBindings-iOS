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
    generate_parameters,
    create_proof,
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

// Just for now
use crate::helpers::engines::*;
use crate::helpers::circuits::*;
use crate::helpers::types_helpers::*;
use crate::filesystem::get_verifying_key_from_file;

#[repr(C)]
#[warn(non_snake_case)]
pub struct VerificationResult {
    value: bool,
    error: *mut c_char,
}

#[no_mangle]
pub extern fn verify(file_with_vk: *const c_char, inputs_array: *const u8, inputs_array_size: usize) -> VerificationResult {
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
    let verifying_key = match get_verifying_key_from_file::<DefaultEngine>(filename) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Error: can't get file!".to_owned()).unwrap().into_raw()
            }
        },
        Ok(result) => result,
    };
    let pvk = prepare_verifying_key(&verifying_key);

    // Proof
    let g1 = Fr::one();
    let g2 = Fr::one();
    let alpha = Fr::from_str("48577").unwrap();
    let beta = Fr::from_str("22580").unwrap();
    let gamma = Fr::from_str("53332").unwrap();
    let delta = Fr::from_str("5481").unwrap();
    let tau = Fr::from_str("3673").unwrap();

    let r = Fr::from_str("27134").unwrap();
    let s = Fr::from_str("17146").unwrap();

    let c = DefaultCircuit::<DefaultEngine> {
        a: None,
        b: None,
        _marker: PhantomData
    };
    let params = match generate_parameters(
        c,
        g1,
        g2,
        alpha,
        beta,
        gamma,
        delta,
        tau
    ) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Error: can't generate params!".to_owned()).unwrap().into_raw() 
            }
        },
        Ok(result) => result,
    };
    let circuit = DefaultCircuit {
        a: Some(true),
        b: Some(false),
        _marker: PhantomData
    };
    let proof = match create_proof(circuit, &params, r, s) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Error: can't create proof!".to_owned()).unwrap().into_raw() 
            }
        },
        Ok(result) => result,
    };

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
    let inputs = Fr::from_str(inputs_str).unwrap();


    // Verification
    let result = match verify_proof(
        &pvk,
        &proof,
        &[inputs]
    ) {
        Err(error) => {
            return VerificationResult {
                value: false,
                error: CString::new("Can't verify proof!".to_owned()).unwrap().into_raw()  
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
pub extern fn free_memory(verification_result: VerificationResult) {
    unsafe {
        if verification_result.error.is_null() { return }
        CString::from_raw(verification_result.error)
    };
}

