use std::process::Command;

const PYTHON_SCRIPT: &str = include_str!("../../script/visualizer.py");

pub fn show(json_filename: &str, save: bool) {
    let mut cmd = Command::new("python3");

    let status = cmd.arg("-c").arg(PYTHON_SCRIPT).arg(json_filename).arg(save.to_string()).status();

    if let Err(e) = status {
        eprintln!("Failed to execute script: {}", e);
    }
}
