use anyhow::{anyhow, Result};

use std::{iter, mem, ptr, thread, time, path, fs, env};
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
use nwg::{CheckBoxState};

const CONFIG_FILE: &str = "pref.toml";

#[cfg(not(debug_assertions))]
const DLL_DATA: &[u8] = include_bytes!("../target/i686-pc-windows-msvc/release/deps/at_vxa_so.dll");
#[cfg(debug_assertions)]
const DLL_DATA: &[u8] = include_bytes!("../target/i686-pc-windows-msvc/debug/deps/at_vxa_so.dll");

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
    #[nwg_control(size: (480, 240), center: true, accept_files: true, title: "axer.tech | VX Ace Shortcuts Override",
    flags: "MAIN_WINDOW")]
    #[nwg_events(OnWindowClose: [App::exit], OnFileDrop: [App::get_drop_path(SELF, EVT_DATA)])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(title: "Wskaż plik edytora", action: nwg::FileDialogAction::Open, filters: "Edytor RPG Maker VX Ace (RPGVXAce.exe)")]
    dialog: nwg::FileDialog,

    #[nwg_control(text: "Ścieżka do edytora")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 1, col_span: 10)]
    editor_path_label: nwg::Label,

    #[nwg_control(flags: "VISIBLE|AUTO_SCROLL", readonly: true, placeholder_text: Some("Brak ścieżki - wskaż położenie edytora"))]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 1, col_span: 8)]
    editor_path: nwg::TextInput,

    #[nwg_control(text: "Zmień")]
    #[nwg_layout_item(layout: grid, col: 8, row: 1, row_span: 1, col_span: 2)]
    #[nwg_events(OnButtonClick: [App::pick_file])]
    editor_path_button: nwg::Button,

    #[nwg_control(text: "Pomiń to okno przy kolejnych uruchomieniach")]
    #[nwg_layout_item(layout: grid, col: 0, row: 3, row_span: 1, col_span: 10)]
    #[nwg_events(OnButtonClick: [App::set_skip_state])]
    skip_checkbox: nwg::CheckBox,

    #[nwg_control(text: "Uruchom i spatchuj edytor")]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, row_span: 2, col_span: 10)]
    #[nwg_events(OnButtonClick: [App::run_patched])]
    launch_button: nwg::Button,

    #[nwg_control(visible: false, icon: Some(& nwg::Icon::from_system
    (nwg::OemIcon::Information)))]
    tray: nwg::TrayNotification,
}

impl App {
    fn update_dir_label(&self) -> bool {
        if let Some(dir) = self.config.borrow().editor_path.clone() {
            self.editor_path.set_text(dir.as_os_str().to_str().unwrap());
            self.editor_path.set_modified(true);
            true
        } else {
            false
        }
    }

    fn update_button(&self) {
        self.launch_button.set_enabled(self.config.borrow().editor_path.is_some());
    }

    fn set_skip_state(&self) {
        let skip = match self.skip_checkbox.check_state() {
            CheckBoxState::Checked => true,
            CheckBoxState::Unchecked => false,
            CheckBoxState::Indeterminate => unreachable!()
        };
        self.config.borrow_mut().skip_to_launch = skip;
    }

    fn pick_file(&self) {
        if let Ok(dir) = env::current_dir() {
            if let Some(dir) = dir.to_str() {
                self.dialog.set_default_folder(dir).expect("Failed to set default directory");
            }
        }

        if self.dialog.run(Some(&self.window)) {
            if let Ok(path) = self.dialog.get_selected_item() {
                self.set_editor_path(PathBuf::from(path));
            }
        }
    }

    fn get_drop_path(&self, data: &nwg::EventData) {
        if let Some(path_str) = data.on_file_drop().files().first() {
            let path = PathBuf::from(path_str);
            if path.file_name().unwrap_or_default() == "RPGVXAce.exe" {
                self.set_editor_path(path);
            }
        }
    }

    fn set_editor_path(&self, path: PathBuf) {
        self.config.borrow_mut().editor_path.replace(path);
        self.update_dir_label();
        self.update_button();
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

    fn run_patched(&self) {
        let path = self.config.borrow().editor_path.clone();
        if let Some(path) = path {
            run(&path).expect("Failed to run editor");
            self.save_config();
            self.notify("VX Ace Shortcuts Override", "Skróty klawiszowe nadpisane!", nwg::OemIcon::Information);
            self.exit();
        }
    }

    fn notify(&self, title: &str, body: &str, icon: nwg::OemIcon) {
        let tray_ico = nwg::Icon::from_system(icon);
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.set_visibility(true); // older Windows versions won't show notification if the tray icon is hidden
        self.tray.show(body, Some(title), Some(flags), Some(&tray_ico));
    }

    fn init(&self) {
        self.setup_config();
        self.setup_data();
        let config = self.config.borrow();
        if config.skip_to_launch {
            self.run_patched();
        } else {
            thread::sleep(time::Duration::from_secs(1));
            self.splash_window.close();
            self.update_button();
            let has_path = self.update_dir_label();
            self.window.set_visible(true);
            if !has_path {
                nwg::modal_info_message(
                    &self.window,
                    "axer.tech | VX Ace Shortcuts Override",
                    "Automatyczne wykrywanie instalacji RPG Makera VX Ace nie powiodło się. Przeciągnij plik uruchamiający edytor na okno lub wskaż do niego ścieżkę ręcznie.",
                );
            }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}