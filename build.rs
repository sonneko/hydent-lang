use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=script/");

    let script_dir = "./script";

    let status = Command::new("npm")
        .args(["start"])
        .current_dir(script_dir)
        .status()
        .expect("Failed to execute npm start");

    if !status.success() {
        panic!("script failed");
    }
}
