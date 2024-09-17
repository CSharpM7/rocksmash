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
mod other_fighters;

#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::plizardon::install();
    crate::other_fighters::install();
}

#[skyline::main(name = "smashline_rocksmash")]
pub fn main() {
    #[cfg(not(feature = "dev"))]
    install();
    #[cfg(feature = "dev")]
    smashline_install();
}