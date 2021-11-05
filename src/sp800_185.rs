#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings_sp800_185.rs"));

use crate::utils::{bitsequence, check_output};

pub fn cshake128(input: &[u8], dk_len: usize, name: Option<&[u8]>, cust: Option<&[u8]>) -> Vec<u8> {
    let (input_ptr, input_len) = bitsequence(Some(input));
    let (name_ptr, name_len) = bitsequence(name);
    let (cust_ptr, cust_len) = bitsequence(cust);

    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "cshake128",
            cSHAKE128(
                input_ptr,
                input_len,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                name_ptr,
                name_len,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

pub fn cshake256(input: &[u8], dk_len: usize, name: Option<&[u8]>, cust: Option<&[u8]>) -> Vec<u8> {
    let (input_ptr, input_len) = bitsequence(Some(input));
    let (name_ptr, name_len) = bitsequence(name);
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];
    unsafe {
        check_output(
            "cshake256",
            cSHAKE256(
                input_ptr,
                input_len,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                name_ptr,
                name_len,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

pub fn kmac128(key: &[u8], input: &[u8], dk_len: usize, cust: Option<&[u8]>) -> Vec<u8> {
    let (key_ptr, key_len) = bitsequence(Some(key));
    let (input_ptr, input_len) = bitsequence(Some(input));
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "kmac128",
            KMAC128(
                key_ptr,
                key_len,
                input_ptr,
                input_len,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

pub fn kmac256(key: &[u8], input: &[u8], dk_len: usize, cust: Option<&[u8]>) -> Vec<u8> {
    let (key_ptr, key_len) = bitsequence(Some(key));
    let (input_ptr, input_len) = bitsequence(Some(input));
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "kmac256",
            KMAC256(
                key_ptr,
                key_len,
                input_ptr,
                input_len,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

pub fn parallel_hash128(
    input: &[u8], block_len: u64, dk_len: usize, cust: Option<&[u8]>,
) -> Vec<u8> {
    let (input_ptr, input_len) = bitsequence(Some(input));
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "ParallelHash128",
            ParallelHash128(
                input_ptr,
                input_len,
                block_len,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

pub fn parallel_hash256(
    input: &[u8], block_len: u64, dk_len: usize, cust: Option<&[u8]>,
) -> Vec<u8> {
    let (input_ptr, input_len) = bitsequence(Some(input));
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "ParallelHash256",
            ParallelHash256(
                input_ptr,
                input_len,
                block_len,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

// int TupleHash128( const TupleElement *tuple, size_t numberOfElements,
//        BitSequence *output, BitLength outputBitLen, const BitSequence
// *customization, BitLength customBitLen);

pub fn tuple_hash128(input: &[&[u8]], dk_len: usize, cust: Option<&[u8]>) -> Vec<u8> {
    let mut inp = Vec::new();
    for i in input {
        inp.push(TupleElement { input: i.as_ptr(), inputBitLen: (i.len() * 8) as u64 });
    }
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "TupleHash128",
            TupleHash128(
                inp.as_ptr(),
                inp.len() as u64,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}

pub fn tuple_hash256(input: &[&[u8]], dk_len: usize, cust: Option<&[u8]>) -> Vec<u8> {
    let mut inp = Vec::new();
    for i in input {
        inp.push(TupleElement { input: i.as_ptr(), inputBitLen: (i.len() * 8) as u64 });
    }
    let (cust_ptr, cust_len) = bitsequence(cust);
    let mut out = vec![0u8; dk_len];

    unsafe {
        check_output(
            "TupleHash256",
            TupleHash256(
                inp.as_ptr(),
                inp.len() as u64,
                out.as_mut_ptr(),
                (out.len() * 8) as u64,
                cust_ptr,
                cust_len,
            ),
        );
    }
    out
}
