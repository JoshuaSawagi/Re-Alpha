#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]
#![feature(const_trait_impl)]
#![allow(warnings)] 


use skyline::nro::{self, NroInfo};
use skyline::{hook, install_hook};
use smash::app::{self, lua_bind::*, sv_system};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::lua2cpp::L2CFighterCommon;
//use crate::common::momentum_transfer;
use crate::utils::get_kind;
use crate::utils::get_category;
//pub use frame_info::*;

mod common;
mod utils;
pub mod consts;
pub mod ext;
pub mod offsets;
mod fighters;
pub mod singletons;
pub mod imports;

#[skyline::main(name = "ReAlpha")]
pub fn main() {
    common::install();
    fighters::install();
}