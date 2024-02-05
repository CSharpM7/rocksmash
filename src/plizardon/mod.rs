mod acmd;
mod status;
//pub mod customstatus;

mod rock;
mod rockstone;

pub fn install() {
    acmd::install();
    status::install();

    rock::install();
    rockstone::install();

    smashline::clone_weapon("link", "boomerang", "plizardon","rock",false);
    smashline::clone_weapon("sheik", "needle", "plizardon","rockstone",false);
}