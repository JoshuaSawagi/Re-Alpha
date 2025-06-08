#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]
#![feature(const_trait_impl)]

mod common;
mod utils;
pub mod consts;
pub mod ext;
pub mod offsets;
mod fighters;
pub mod singletons;

#[skyline::main(name = "L-Cancels")]
pub fn main() {
    common::install();
    fighters::install();
}