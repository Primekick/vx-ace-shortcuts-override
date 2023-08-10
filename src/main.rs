#![windows_subsystem = "windows"]
#![allow(special_module_name)]

mod lib;
use crate::lib::msg_box;

use dll_syringe::process::{OwnedProcess};
use dll_syringe::Syringe;

#[cfg(not(debug_assertions))]
fn get_dll_path() -> &'static str { "./at_vxa_so.dll" }

#[cfg(debug_assertions)]
fn get_dll_path() -> &'static str { "./target/debug/at_vxa_so.dll" }

fn main() {
    if let Some(target_proc) = OwnedProcess::find_first_by_name("RPGVXAce") {
        let syringe = Syringe::for_process(target_proc);
        syringe.inject(get_dll_path()).unwrap();
    } else {
        msg_box("VX Ace Shortcuts Override", "Najpierw uruchom edytor VX Ace!");
    }
}