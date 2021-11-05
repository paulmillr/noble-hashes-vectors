#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings_k12.rs"));

use crate::utils::check_output;

pub fn k12(input: &[u8], dk_len: usize, cust: Option<&[u8]>) -> Vec<u8> {
    let c = if let Some(c) = cust { c } else { &[] };
    let mut out = vec![0u8; dk_len];
    unsafe {
        check_output(
            "k12",
            KangarooTwelve(
                input.as_ptr(),
                input.len() as u64,
                out.as_mut_ptr(),
                (out.len()) as u64,
                c.as_ptr(),
                c.len() as u64,
            ),
        );
    }
    out
}
