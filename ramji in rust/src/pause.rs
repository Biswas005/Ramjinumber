// command promt pause.rs

use std::process::Command;

pub fn pause_cmd() {
    // Run the 'pause' command as a subprocess to keep the CMD window open
    let status = Command::new("cmd")
        .args(&["/C", "pause"])
        .status()
        .expect("Failed to execute pause command");

    if !status.success() {
        eprintln!("Failed to run pause command");
    }
}
