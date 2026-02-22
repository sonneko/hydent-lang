use core::panic;
use std::process::Command;

fn main() {
    if std::env::var("CARGO_CFG_TEST").is_ok() {
        // skip build.rs in test case
        return;
    }

    println!("cargo:rerun-if-changed=script/");
    let is_ci_env = std::env::var("IS_CI_ENV").unwrap_or_else(|_| "dev".to_string());

    let script_dir = "./script";

    let status = if &is_ci_env == "ci" {
        Command::new("npm")
            .args(["start", "--", "ci"])
            .current_dir(script_dir)
            .status()
            .expect("Failed to execute npm start")
    } else if &is_ci_env == "dev" {
        Command::new("npm")
            .args(["start"])
            .current_dir(script_dir)
            .status()
            .expect("Failed to execute npm start")
    } else {
        panic!("Unknown environment IS_CI_ENV: {}", is_ci_env);
    };

    if !status.success() {
        panic!("script failed");
    }
}
