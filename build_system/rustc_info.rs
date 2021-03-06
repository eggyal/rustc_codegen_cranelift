use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub(crate) fn get_host_triple() -> String {
    let version_info =
        Command::new("rustc").stderr(Stdio::inherit()).args(&["-vV"]).output().unwrap().stdout;
    String::from_utf8(version_info)
        .unwrap()
        .lines()
        .to_owned()
        .find(|line| line.starts_with("host"))
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .to_owned()
}

pub(crate) fn get_rustc_path() -> PathBuf {
    let rustc_path = Command::new("rustup")
        .stderr(Stdio::inherit())
        .args(&["which", "rustc"])
        .output()
        .unwrap()
        .stdout;
    Path::new(String::from_utf8(rustc_path).unwrap().trim()).to_owned()
}

pub(crate) fn get_default_sysroot() -> PathBuf {
    let default_sysroot = Command::new("rustc")
        .stderr(Stdio::inherit())
        .args(&["--print", "sysroot"])
        .output()
        .unwrap()
        .stdout;
    Path::new(String::from_utf8(default_sysroot).unwrap().trim()).to_owned()
}

pub(crate) fn get_file_name(crate_name: &str, crate_type: &str) -> String {
    let file_name = Command::new("rustc")
        .stderr(Stdio::inherit())
        .args(&[
            "--crate-name",
            crate_name,
            "--crate-type",
            crate_type,
            "--print",
            "file-names",
            "-",
        ])
        .output()
        .unwrap()
        .stdout;
    let file_name = String::from_utf8(file_name).unwrap().trim().to_owned();
    assert!(!file_name.contains('\n'));
    assert!(file_name.contains(crate_name));
    file_name
}
