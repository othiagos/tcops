use std::process::Command;

const PYTHON_SCRIPT: &str = include_str!("../../script/visualizer.py");

pub fn show(path: &str, show: bool, save: bool) {
    let mut cmd = Command::new("python3");

    cmd.arg("-c").arg(PYTHON_SCRIPT).arg(path);

    if show {
        cmd.arg("show");
    }

    if save {
        cmd.arg("save");
    }

    if (show || save)
        && let Err(e) = cmd.status()
    {
        eprintln!("Failed to execute script: {}", e);
    }
}
