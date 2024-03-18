
#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::plizardon::install();
    crate::other_fighters::install();
}