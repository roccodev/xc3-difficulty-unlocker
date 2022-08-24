use std::{ffi::CStr, io::Cursor};

use skyline::{
    hook,
    hooks::{InlineCtx, Region},
};

mod config;

#[skyline::main(name = "xc3_difficulty_unlocker")]
pub fn main() {
    println!("[XC3-DU] Loading...");

    let text_ptr = unsafe { skyline::hooks::getRegionAddress(Region::Text) } as *const u8;

    println!("[XC3-DU] Installing hooks");
    unsafe {}

    println!("[XC3-DU] Loaded!");
}
