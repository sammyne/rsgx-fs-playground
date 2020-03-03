#![no_std]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;

use std::prelude::v1::*;

use std::ffi::CStr;
use std::io::{BufRead, BufReader};
use std::os::raw::{c_char, c_int};
use std::slice;
use std::untrusted::fs::{self, File};

#[no_mangle]
pub extern "C" fn bufio_read(path: *const c_char) {
    let path = must_to_str(path);

    let lines_count = {
        let f = File::open(path).expect(format!("failed to open {}", path).as_str());
        BufReader::new(f).lines().count()
    };

    let mut data: Vec<String> = Vec::with_capacity(lines_count);

    let f = File::open(path).expect(format!("failed to open {}", path).as_str());
    let mut r = BufReader::new(f);

    for line in r.lines() {
        let v = line.expect("invalid line");
        println!("{}", v);
        data.push(v);
    }
}

#[no_mangle]
pub extern "C" fn read_file(path: *const c_char) {
    let path = must_to_str(path);
    match fs::read_to_string(path) {
        Ok(v) => println!("content: {}", v),
        Err(err) => println!("err: {:?}", err),
    }
}

#[no_mangle]
pub extern "C" fn read_to_vec(path: *const c_char) {
    let path = must_to_str(path);
    match fs::read(path) {
        Ok(v) => println!("content: {:?}", v),
        Err(err) => println!("err: {:?}", err),
    }
}

fn must_to_str<'a>(s: *const c_char) -> &'a str {
    unsafe { CStr::from_ptr(s).to_str().expect("invalid c-string") }
}
