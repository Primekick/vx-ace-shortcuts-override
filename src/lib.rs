#![allow(improper_ctypes_definitions)]

use std::ffi::CString;
use std::{mem, ptr};
use winapi::shared::minwindef::HINSTANCE;
use winapi::um::winuser::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi_easy::keyboard::{GlobalHotkeySet, Key, Modifier};

#[derive(Copy, Clone)]
enum Action {
    SmallOgonek,
    BigOgonek,
}

#[no_mangle]
fn send_key(key_code: u16, up: bool) {
    let flags = if up { KEYEVENTF_UNICODE | KEYEVENTF_KEYUP } else { KEYEVENTF_UNICODE };
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { mem::zeroed() },
    };

    unsafe {
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: 0,
            dwFlags: flags,
            dwExtraInfo: 1,
            wScan: key_code,
            time: 0,
        };

        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

#[no_mangle]
fn send_a_with_ogonek(uppercase: bool) {
    let key = if uppercase { 'Ą' as u16 } else { 'ą' as u16 };
    send_key(key, false);
    send_key(key, true);
}

#[no_mangle]
fn key_hook() -> u32 {
    let hotkeys = GlobalHotkeySet::new()
        .add_global_hotkey(Action::BigOgonek, Modifier::Shift + Modifier::Ctrl + Modifier::Alt + Key::A)
        .add_global_hotkey(Action::SmallOgonek, Modifier::Ctrl + Modifier::Alt + Key::A);

    for action in hotkeys.listen_for_hotkeys().unwrap() {
        match action.unwrap() {
            Action::SmallOgonek => {
                send_a_with_ogonek(false);
            }
            Action::BigOgonek => {
                send_a_with_ogonek(true);
            }
        }
    }

    0
}

#[no_mangle]
pub extern fn msg_box(title: &str, content: &str) {
    let lp_title = CString::new(title).unwrap();
    let lp_content = CString::new(content).unwrap();
    unsafe {
        MessageBoxA(ptr::null_mut(),
                    lp_content.as_ptr(),
                    lp_title.as_ptr(),
                    MB_OK | MB_ICONINFORMATION | MB_SYSTEMMODAL);
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(move || key_hook());
    }

    true
}
