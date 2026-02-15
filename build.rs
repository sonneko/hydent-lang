use std::process::Command;

fn main() {
    let script_dir = "./script";

    let status = Command::new("npm")
        .args(["install"])
        .current_dir(script_dir)
        .status()
        .expect("Failed to execute npm install");

    if !status.success() {
        panic!("npm install failed");
    }

    let status = Command::new("npm")
        .args(["run", "run"])
        .current_dir(script_dir)
        .status()
        .expect("Failed to execute npm run run");

    if !status.success() {
        panic!("npm run run failed");
    }
}
