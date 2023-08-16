use embed_manifest::{embed_manifest, new_manifest};
use embed_manifest::manifest::ActiveCodePage::Legacy;
use embed_manifest::manifest::SupportedOS::{Windows10, Windows7};

fn main() {
    let manifest = new_manifest("axer.tech.vxace_shortcuts_override")
        .active_code_page(Legacy)
        .supported_os(Windows7..=Windows10);
    embed_manifest(manifest).expect("unable to embed manifest file");
    println!("cargo:rerun-if-changed=build.rs");
}