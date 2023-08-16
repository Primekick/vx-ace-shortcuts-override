use embed_manifest::{embed_manifest, new_manifest};
use embed_manifest::manifest::ActiveCodePage::Legacy;
use embed_manifest::manifest::SupportedOS::{Windows10, Windows7};
use embed_resource;

fn main() {
    let manifest = new_manifest("axer.tech.vxace_shortcuts_override")
        .active_code_page(Legacy)
        .supported_os(Windows7..=Windows10);
    embed_manifest(manifest).expect("unable to embed manifest file");
    embed_resource::compile("res/splash.rc", embed_resource::NONE);
    println!("cargo:rerun-if-changed=build.rs");
}