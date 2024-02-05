#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![allow(warnings)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

pub mod imports;
pub mod vars;

mod plizardon;

#[skyline::main(name = "smashline_rocksmash")]
pub fn main() {
    plizardon::install();
}