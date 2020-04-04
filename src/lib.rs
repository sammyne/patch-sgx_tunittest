#![no_std]

extern crate sgx_tstd as std;

use sgx_tunittest::*;
//use std::panic; // panic needed by sgx_tunittest without patch

fn world() {
    panic!("aha")
}

pub fn hello() {
    should_panic!(world())
}
