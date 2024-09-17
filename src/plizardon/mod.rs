mod acmd;
mod status;
//pub mod customstatus;

mod rock;
mod rockstone;

use crate::vars::*;
use smash::lib::lua_const::*;

pub fn install() {
    acmd::install();
    status::install();

    rock::install();
    rockstone::install();

    unsafe {
        smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOOMERANG, "plizardon", "rock", false);
        smashline::clone_weapon("sheik", *WEAPON_KIND_SHEIK_NEEDLE, "plizardon", "rockstone", false);
    }
}