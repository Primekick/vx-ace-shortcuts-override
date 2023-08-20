#![windows_subsystem = "windows"]
#![allow(special_module_name)]
mod app;

use nwg::NativeUi;
use crate::app::App;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = App::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}