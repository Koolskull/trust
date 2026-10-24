//                    ☦
//         ╔════════════════════╗
//         ║  Establish the     ║
//         ║  work of our hands.║
//         ╚════════════════════╝
//
//   Psalm 90:17. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let code = match args.next().as_deref() {
        Some("bundle") => bundle(),
        Some("hear") => hear(),
        _ => {
            eprintln!("usage: cargo xtask <bundle|hear>");
            2
        }
    };
    if code != 0 {
        std::process::exit(code);
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn hear() -> i32 {
    let samples = krush::render_hear();
    let path = krush::hear_wav_path();
    if let Err(err) = krush::write_wav(&path, &samples, 48_000) {
        eprintln!("{err}");
        return 1;
    }
    let shown = path.canonicalize().unwrap_or(path);
    println!("{}", shown.display());
    0
}

fn bundle() -> i32 {
    let root = workspace_root();
    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "krush"])
        .current_dir(&root)
        .status();
    let Ok(status) = status else {
        eprintln!("cargo build failed to start");
        return 1;
    };
    if !status.success() {
        return status.code().unwrap_or(1);
    }
    let lib = match env::consts::OS {
        "windows" => "krush.dll",
        "macos" => "libkrush.dylib",
        _ => "libkrush.so",
    };
    let bundled = match env::consts::OS {
        "windows" => "KRUSH.dll",
        "macos" => "KRUSH.dylib",
        _ => "KRUSH.so",
    };
    let arch = match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => "x86_64-linux",
        ("linux", "aarch64") => "aarch64-linux",
        ("macos", _) => "MacOS",
        ("windows", "x86_64") => "x86_64-win",
        ("windows", "aarch64") => "aarch64-win",
        _ => {
            eprintln!("unsupported OS/arch");
            return 1;
        }
    };
    let from = root.join("target/release").join(lib);
    let dest = root
        .join("target/bundled/KRUSH.clap/Contents")
        .join(arch)
        .join(bundled);
    if let Some(parent) = dest.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            eprintln!("{err}");
            return 1;
        }
    }
    if let Err(err) = fs::copy(&from, &dest) {
        eprintln!("copy {} -> {}: {err}", from.display(), dest.display());
        return 1;
    }
    println!("{}", dest.display());
    0
}
