use anyhow::{anyhow, Result};

use std::{iter, mem, ptr, thread, time, path, fs};
use std::cell::RefCell;
use std::sync::OnceLock;
use std::ops::Deref;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use directories::ProjectDirs;
use dll_syringe::process::{OwnedProcess};
use dll_syringe::Syringe;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use serde::{Serialize, Deserialize};

use native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;

const CONFIG_FILE: &str = "pref.toml";

#[cfg(not(debug_assertions))]
const DLL_DATA: &[u8] = include_bytes!("../target/i686-pc-windows-msvc/release/at_vxa_so.dll");
#[cfg(debug_assertions)]
const DLL_DATA: &[u8] = include_bytes!("../target/i686-pc-windows-msvc/debug/at_vxa_so.dll");

static CFG_PATH: OnceLock<PathBuf> = OnceLock::new();
static DLL_PATH: OnceLock<PathBuf> = OnceLock::new();

fn cfg_path() -> &'static PathBuf {
    CFG_PATH.get_or_init(|| {
        let Some(proj_dirs) = ProjectDirs::from("pl", "axer-tech", "vx-ace-shortcuts-override") else {
            panic!()
        };

        proj_dirs.config_dir().join(CONFIG_FILE)
    })
}

fn dll_path() -> &'static PathBuf {
    DLL_PATH.get_or_init(|| {
        let Some(proj_dirs) = ProjectDirs::from("pl", "axer-tech", "vx-ace-shortcuts-override") else {
            panic!()
        };

        proj_dirs.data_dir().join(format!("at_vxa_so_{}.dll", env!("CARGO_PKG_VERSION")))
    })
}

#[derive(Serialize, Deserialize)]
struct AppConfig {
    pref_lang: String,
    editor_path: Option<PathBuf>,
    skip_to_launch: bool,
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
            skip_to_launch: false,
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
        syringe.inject(dll_path())?;
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
        let mut pi: PROCESS_INFORMATION = mem::zeroed();
        let mut si: STARTUPINFOW = mem::zeroed();
        si.cb = mem::size_of::<STARTUPINFOW>() as u32;
        let created =
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
            );
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
    splash_window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(& data.embed), source_embed_str: Some("SPLASH"))]
    splash: nwg::Bitmap,

    #[nwg_control(size: (320, 240), bitmap: Some(& data.splash))]
    image_frame: nwg::ImageFrame,

    // actual app
    #[nwg_control(size: (480, 240), center: true, accept_files: true, title: "axer.tech | VX Ace\
     Shortcuts Override",
    flags: "MAIN_WINDOW")]
    #[nwg_events(OnWindowClose: [App::exit], OnFileDrop: [App::get_drop_path(SELF, EVT_DATA)])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 2)]
    show_dir: nwg::Label,

    #[nwg_control(text: "Run and inject")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 1)]
    #[nwg_events(OnButtonClick: [App::run_patched])]
    launch_button: nwg::Button,

    #[nwg_control(realtime: true, visible: true, icon: Some(& nwg::Icon::from_system
    (nwg::OemIcon::Information)))]
    tray: nwg::TrayNotification,
}

impl App {
    fn update_dir_label(&self) {
        if let Some(dir) = self.config.borrow().editor_path.clone() {
            self.show_dir.set_text(dir.as_os_str().to_str().unwrap());
        }
    }

    fn get_drop_path(&self, data: &nwg::EventData) {
        if let Some(path_str) = data.on_file_drop().files().first() {
            let path = PathBuf::from(path_str);
            if path.extension().unwrap_or_default() == "exe" {
                self.set_editor_path(path);
            }
        }
    }

    fn set_editor_path(&self, path: PathBuf) {
        self.config.borrow_mut().editor_path.replace(path);
        self.update_dir_label();
        self.save_config();
    }

    fn load_config(&self) {
        let cfg_content = fs::read_to_string(cfg_path())
            .expect("Failed to read config");
        *self.config.borrow_mut() = toml::from_str(&cfg_content)
            .expect("Failed to parse config");
    }

    fn save_config(&self) {
        let serialized = toml::to_string_pretty(self.config.borrow().deref())
            .expect("Failed to serialize config");
        fs::write(cfg_path(), serialized)
            .expect("Failed to save config");
    }

    fn setup_config(&self) {
        if !cfg_path().exists() {
            fs::create_dir_all(cfg_path().parent().unwrap())
                .expect("Failed to create app config directory");
            self.save_config();
        } else {
            self.load_config();
        }
    }

    fn setup_data(&self) {
        let dll_dir = dll_path().parent().unwrap();
        if !dll_dir.exists() {
            fs::create_dir_all(&dll_dir)
                .expect("Failed to create app data directory");
        }
        if !dll_path().exists() {
            fs::write(dll_path(), DLL_DATA)
                .expect("Failed to extract dll");
        }
    }

    fn init(&self) {
        self.setup_config();
        self.setup_data();
        self.update_dir_label();
        thread::sleep(time::Duration::from_secs(1));
        self.splash_window.close();
        self.window.set_visible(true);
    }

    fn run_patched(&self) {
        if let Some(path) = &self.config.borrow().editor_path {
            run(path).unwrap();
            self.notify("VX Ace Shortcuts Override", "Skróty klawiszowe nadpisane!", nwg::OemIcon::Information);
            self.exit();
        }
    }

    fn notify(&self, title: &str, body: &str, icon: nwg::OemIcon) {
        let tray_ico = nwg::Icon::from_system(icon);
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show(body, Some(title), Some(flags), Some(&tray_ico));
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
