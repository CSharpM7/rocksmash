mod status;
mod acmd;

pub fn install() {
    status::install();
    acmd::install();
}