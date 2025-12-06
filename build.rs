use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend");

    let status = Command::new("wasm-pack")
        .args(&["build", "--target", "web", "frontend"])
        .status()
        .expect("Failed to run wasm-pack");

    if !status.success() {
        panic!("wasm-pack build failed");
    }
}
