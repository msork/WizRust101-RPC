fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rerun-if-changed=packaging/windows/wizrust101-rpc.rc");
        println!("cargo:rerun-if-changed=packaging/windows/wizrust101-rpc.ico");
        embed_resource::compile("packaging/windows/wizrust101-rpc.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("embed the Windows application icon resource");
        embed_manifest::embed_manifest(embed_manifest::new_manifest("WizRust101-RPC"))
            .expect("embed the Windows Common Controls v6 activation manifest");
    }
}
