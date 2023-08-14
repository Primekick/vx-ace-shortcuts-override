// #![windows_subsystem = "windows"]
#![allow(special_module_name)]

mod lib;

use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use anyhow::{anyhow, Result};

use std::time::Duration;
use std::{iter, mem, ptr, thread};
use std::path::Path;
use crate::lib::msg_box;

use dll_syringe::process::{OwnedProcess};
use dll_syringe::Syringe;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[cfg(not(debug_assertions))]
fn get_dll_path() -> &'static str { "./at_vxa_so.dll" }

#[cfg(debug_assertions)]
fn get_dll_path() -> &'static str { "./target/debug/at_vxa_so.dll" }

fn inject() -> Result<()> {
    if let Some(target_proc) = OwnedProcess::find_first_by_name("RPGVXAce") {
        let syringe = Syringe::for_process(target_proc);
        syringe.inject(get_dll_path())?;
        Ok(())
    } else {
        Err(anyhow!("Not yet"))
    }
}

fn main() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let vxa_reg = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Enterbrain\\RPGVXAce")?;
    let vxa_dir: String = vxa_reg.get_value("ApplicationPath")?;

    let mut path = Path::new(&vxa_dir).join("RPGVXAce.exe");
    let mut path_enc = path.as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();
    unsafe {
        let mut si: STARTUPINFOW = mem::zeroed();
        si.cb = size_of::<STARTUPINFOW>() as u32;
        let mut pi: PROCESS_INFORMATION = mem::zeroed();
        let created = unsafe {
            CreateProcessW(
                ptr::null(),
                path_enc.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                false.into(),
                0,
                ptr::null_mut(),
                ptr::null(),
                &mut si,
                &mut pi,
            )
        };
        if created == 0 {
            panic!();
        }
    }

    println!("Process spawn queued.");
    loop {
        println!("Waiting 1 sec for editor to launch...");
        thread::sleep(Duration::from_secs(1));
        if inject().is_ok() {
            break;
        }
    }

    Ok(())
}