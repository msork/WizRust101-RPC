fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_manifest::embed_manifest(embed_manifest::new_manifest("WizRust101-RPC"))
            .expect("embed the Windows Common Controls v6 activation manifest");
    }
}
