#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{
    OpenFlags,
    draw,
};

use alloc::string::String;

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    println!("draw");
    for x in 100..200 {
        draw(x, x);
    }
    0
}