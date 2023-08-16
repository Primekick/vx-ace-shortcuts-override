use anyhow::{anyhow, Result};

use std::{iter, mem, ptr, thread, time, path, fs};
use std::cell::RefCell;
use std::ffi::OsString;
use std::ops::Deref;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use directories::ProjectDirs;

use dll_syringe::process::{OwnedProcess};
use dll_syringe::Syringe;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;
use serde::{Serialize, Deserialize};

const CONFIG_FILE: &str = "pref.toml";

#[cfg(not(debug_assertions))]
fn get_dll_path() -> &'static str { "./at_vxa_so.dll" }

#[cfg(debug_assertions)]
fn get_dll_path() -> &'static str { "./target/i686-pc-windows-msvc/debug/at_vxa_so.dll" }

#[derive(Serialize, Deserialize)]
struct AppConfig {
    pref_lang: String,
    editor_path: Option<PathBuf>,
}

fn get_dir_from_registry() -> Result<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let vxa_reg = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Enterbrain\\RPGVXAce")?;
    let vxa_dir = vxa_reg.get_value("ApplicationPath")?;

    Ok(vxa_dir)
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut def = Self {
            pref_lang: String::from("pl-PL"),
            editor_path: None,
        };

        if let Ok(vxa_dir) = get_dir_from_registry() {
            def.editor_path = Some(path::Path::new(&vxa_dir).join("RPGVXAce.exe"));
        };

        def
    }
}

fn inject() -> Result<()> {
    if let Some(target_proc) = OwnedProcess::find_first_by_name("RPGVXAce") {
        let syringe = Syringe::for_process(target_proc);
        syringe.inject(get_dll_path())?;
        Ok(())
    } else {
        Err(anyhow!("Not yet"))
    }
}

fn run(path: &PathBuf) -> Result<()> {
    let mut path_enc = path.as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();
    unsafe {
        let mut si: STARTUPINFOW = mem::zeroed();
        si.cb = mem::size_of::<STARTUPINFOW>() as u32;
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
        thread::sleep(time::Duration::from_secs(1));
        if inject().is_ok() {
            break;
        }
    }

    Ok(())
}

#[derive(Default, NwgUi)]
pub struct App {
    // state
    config: RefCell<AppConfig>,

    // splashscreen
    #[nwg_control(size: (320, 240), center: true, flags: "POPUP|VISIBLE", topmost: true)]
    #[nwg_events(OnInit: [App::init])]
    splash: nwg::Window,

    // proper app
    #[nwg_control(size: (320, 240), center: true, title: "axer.tech | VX Ace Shortcuts Override",
    flags: "MAIN_WINDOW")]
    #[nwg_events(OnWindowClose: [App::exit])]
    window: nwg::Window,

    #[nwg_control(text: "Run and inject")]
    #[nwg_events(OnButtonClick: [App::run_patched])]
    launch_button: nwg::Button,
}

impl App {
    fn load_config(&self) {
        let Some(proj_dirs) = ProjectDirs::from("pl", "axer-tech", "vx-ace-shortcuts-override") else {
            panic!()
        };

        let cfg_path = proj_dirs.config_dir().join(CONFIG_FILE);

        if !cfg_path.exists() {
            fs::create_dir_all(proj_dirs.config_dir())
                .expect("Failed to create app config directory");
            let serialized = toml::to_string_pretty(self.config.borrow().deref())
                .expect("Failed to serialize config");
            fs::write(cfg_path, serialized)
                .expect("Failed to save config");
        } else {
            let cfg_content = fs::read_to_string(cfg_path)
                .expect("Failed to read config");
            *self.config.borrow_mut() = toml::from_str(&cfg_content)
                .expect("Failed to parse config");
        }
    }

    fn init(&self) {
        self.load_config();
        thread::sleep(time::Duration::from_secs(1));
        self.splash.close();
        self.window.set_visible(true);
    }

    fn run_patched(&self) {
        if let Some(path) = &self.config.borrow().editor_path {
            run(path).unwrap()
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
