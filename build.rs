// SPDX-License-Identifier: 0BSD
// rustc-version-const
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

use std::{env, ffi::OsString, process::Command, str};

fn main() {
    let rustc = env::var_os("RUSTC").filter(|str| !str.is_empty()).unwrap_or_else(|| OsString::from("rustc"));
    let rustc_wrapper = env::var_os("RUSTC_WRAPPER").filter(|str| !str.is_empty());

    let result = match rustc_wrapper {
        Some(wrapper) => Command::new(wrapper).arg(rustc).arg("-V").output(),
        None => Command::new(rustc).arg("-V").output(),
    };

    let version_output = match result {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).into_owned(),
        _ => panic!("Failed to determine rustc version!"),
    };

    let version_line = version_output.lines().next().map(str::trim_ascii);
    let (prefix, version_string) = match version_line {
        Some(line) => match line.split_once(char::is_whitespace) {
            Some((prefix, version_string)) => (prefix, version_string),
            _ => panic!("Failed to parse rustc version!"),
        },
        _ => panic!("Failed to read rustc version!"),
    };

    assert_eq!(prefix, "rustc", "The rustc version string starts with an unrecognized prefix!");

    let version_string = version_string.trim_ascii_start();
    let rustc_version = match version_string.split_once(char::is_whitespace) {
        Some((rustc_version, _)) => rustc_version,
        _ => panic!("Failed to extract rustc version number!"),
    };

    println!("cargo:rustc-env=_RUSTC_VERSION_CONST={}", rustc_version);
    println!("cargo:rustc-env=_RUSTC_VERSION_CONST_FULL={}", version_string);
}
