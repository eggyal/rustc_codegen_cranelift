use std::env;
use std::process::Command;

pub(crate) fn build_backend(channel: &str) -> String {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    match channel {
        "debug" => {}
        "release" => {
            cmd.arg("--release");
        }
        _ => unreachable!(),
    }

    if cfg!(unix) {
        if cfg!(target_os = "macos") {
            cmd.env(
                "RUSTFLAGS",
                "-Csplit-debuginfo=unpacked \
                -Clink-arg=-Wl,-rpath,@loader_path/../lib \
                -Zosx-rpath-install-name"
                    .to_string()
                    + env::var("RUSTFLAGS").as_deref().unwrap_or(""),
            );
        } else {
            cmd.env(
                "RUSTFLAGS",
                "-Clink-arg=-Wl,-rpath=$ORIGIN/../lib ".to_string()
                    + env::var("RUSTFLAGS").as_deref().unwrap_or(""),
            );
        }
    }

    eprintln!("[BUILD] rustc_codegen_cranelift");
    crate::utils::spawn_and_wait(cmd);

    crate::rustc_info::get_file_name("rustc_codegen_cranelift", "dylib")
}
