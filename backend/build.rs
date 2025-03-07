
fn main() {
    #[cfg(feature = "compression")]
    {
        use std::process::Command;
        // FYI: this seems always to be the parent of src, even if one runs cargo build at workspace root
        let current_dir = std::env::current_dir().unwrap();

        let workspace_root = current_dir.parent().unwrap();

        let frontend_dir = workspace_root.join("frontend/dist/identity");

        Command::new("trunk-compress").output().expect("failed to compress");

        println!("cargo:rerun-if-changed={}", frontend_dir.to_str().unwrap());
    }
}
