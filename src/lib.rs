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
mod installer;

mod plizardon;
mod other_fighters;

#[skyline::main(name = "smashline_rocksmash")]
pub fn main() {
    #[cfg(not(feature = "dev"))]{ 
        installer::install();
    }
    #[cfg(feature = "dev")]
    installer::smashline_install();
}