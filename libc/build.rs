extern crate avr_mcu;
extern crate bindgen;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate zip_extract;

use avr_mcu::Mcu;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const BINDINGS_DEST: &'static str = "src/bindings.rs";

const LIBC_OWNER_REPO: &'static str = "avrdudes/avr-libc";
const LIBC_LOCAL_PATH: &'static str = "avr-libc";

/// Headers which can't be used from Rust.
const HEADER_BLACKLIST: &'static [&'static str] = &[
    "avr/crc16.h",
    "avr/parity.h",
    "avr/delay.h",  // Deprecated, moved to 'util'
    "avr/signal.h", // Deprecated, moved to `avr/interrupt.h`
    "avr/wdt.h",    // Uses inline assembly constraint I, gives out of range errors because
    // bindgen does not use an AVR assembler..
    "stdfix-avrlibc.h", // Deprecated, use 'stdfix.h' instead.
    "util/delay.h",
    "util/delay_basic.h", // relies on AVR-GCC specific optimisations
    "util/setbaud.h",     // mostly made of preprocessor magic
];

const DEVICE_SPECIFIC_HEADERS: &'static [&'static str] =
    &["avr/boot.h", "avr/sleep.h", "util/crc16.h"];

lazy_static! {
    static ref MCU: Option<Mcu> = avr_mcu::current::mcu();
}

fn architecture() -> avr_mcu::Architecture {
    match *MCU {
        Some(ref mcu) => mcu.architecture,
        None => avr_mcu::Architecture::Avr2, // The bare minimum.
    }
}

fn download_libc() -> Result<(), reqwest::Error> {
    let mut default_headers = HeaderMap::new();
    default_headers.append(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );
    default_headers.append("User-Agent", HeaderValue::from_static("avr-rust-libc"));

    let api = reqwest::blocking::Client::builder()
        .default_headers(default_headers)
        .build()
        .unwrap();
    let latest_release = api
        .get(format!(
            "https://api.github.com/repos/{LIBC_OWNER_REPO}/releases/latest"
        ))
        .send()?;
    let body = latest_release.text()?;
    let zipball_url = serde_json::from_str::<Value>(&body)
        .unwrap()
        .get("zipball_url")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    eprintln!("cargo:warning={zipball_url}");
    let zipball_response = api.get(zipball_url).send()?;
    eprintln!("cargo:warning={:?}", zipball_response.content_length());

    let zip_content = zipball_response.bytes()?;
    zip_extract::extract(Cursor::new(zip_content), Path::new(LIBC_LOCAL_PATH), true);

    Ok(())
}

fn main() {
    if !std::fs::exists(LIBC_LOCAL_PATH).unwrap() {
        match download_libc() {
            Ok(_) => (),
            Err(err) => return eprintln!("Something gone wrong: {err}"),
        };
    }

    if MCU.is_none() {
        println!("cargo:warning=not targeting a specific microcontroller, create a custom target specification to enable mcu-specific functionality");
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let libc_dir = manifest_dir.join("avr-libc");
    let include_dir = libc_dir.join("include");
    let arch_dir = libc_dir.join("avr").join("lib").join(architecture().name());
    let static_lib_path = arch_dir.join("libc.a");

    println!("static lib path is {}", static_lib_path.as_path().display());

    if !static_lib_path.exists() {
        println!(
            "avr-libc not yet built for '{}', building now",
            architecture().name()
        );
        bootstrap(&libc_dir);
        configure(&libc_dir);

        make(&include_dir);
        make(&arch_dir);
    }

    generate_bindings(&libc_dir);

    println!("cargo:rustc-link-search={}", arch_dir.display());
    println!("cargo:rustc-link-lib=static=c");
}

fn bootstrap(libc_dir: &Path) {
    println!("Bootstrapping avr-libc");

    let mut cmd = Command::new("sh");
    cmd.arg("bootstrap");

    cmd.current_dir(libc_dir);
    println!("{:?}", cmd);

    if !cmd
        .status()
        .expect("failed to bootstrap avr-libc")
        .success()
    {
        panic!("failed to bootstrap");
    }
}

fn configure(libc_dir: &Path) {
    println!("Configuring avr-libc");

    let host = env::var("HOST").unwrap();

    let mut cmd = Command::new("sh");
    cmd.arg("configure");
    cmd.arg(&format!("--build={}", host));
    cmd.arg("--host=avr");

    cmd.env("CC", "avr-gcc");
    if let Some(mcu) = MCU.as_ref() {
        cmd.env(
            "CFLAGS",
            format!("-mmcu={}", mcu.device.name.to_lowercase()),
        );
    }

    cmd.current_dir(libc_dir);
    println!("{:?}", cmd);

    if !cmd
        .status()
        .expect("failed to configure avr-libc")
        .success()
    {
        panic!("failed to configure");
    }
}

fn make(dir: &Path) {
    println!("Making avr-libc");

    let mut cmd = Command::new("make");
    cmd.current_dir(&dir);
    println!("{:?}", cmd);

    if !cmd.status().expect("failed to compile avr-libc").success() {
        panic!("failed to make");
    }
}

fn headers_inside(dir: &Path, libc_path: &Path) -> Vec<PathBuf> {
    let mut headers = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            match path.extension().clone() {
                Some(ext) if ext == "h" => {
                    if !is_header_blacklisted(&path, libc_path) {
                        headers.push(path.clone());
                    }
                }
                _ => (),
            }
        }
    }

    headers
}

fn is_header_blacklisted(path: &Path, libc_path: &Path) -> bool {
    if let Some(stem) = path.file_stem() {
        if stem.to_str().unwrap().starts_with("io") {
            return true;
        }
    }

    is_header_in_list(path, libc_path, HEADER_BLACKLIST)
        || (MCU.is_none() && is_header_device_specific(path, libc_path))
}

fn is_header_device_specific(path: &Path, libc_path: &Path) -> bool {
    is_header_in_list(path, libc_path, DEVICE_SPECIFIC_HEADERS)
}

fn is_header_in_list(path: &Path, libc_path: &Path, list: &[&str]) -> bool {
    let include_path = libc_path.join("include");

    list.iter().any(|header| include_path.join(header) == path)
}

fn base_headers(libc_dir: &Path) -> Vec<PathBuf> {
    let include_dir = libc_dir.join("include");
    let mut headers = Vec::new();

    headers.extend(headers_inside(&include_dir, libc_dir));
    headers.extend(headers_inside(&include_dir.join("util"), libc_dir));
    headers.extend(headers_inside(&include_dir.join("sys"), libc_dir));
    headers.extend(headers_inside(&include_dir.join("avr"), libc_dir));
    headers
}

fn mcu_define_name() -> Option<&'static str> {
    MCU.as_ref().map(|mcu| &mcu.c_preprocessor_name[..])
}

fn generate_bindings(libc_dir: &Path) {
    // Configure and generate bindings.
    let avr_arch_name = format!("{:?}", architecture()).to_lowercase();
    let mut builder = bindgen::builder()
        .use_core()
        .ctypes_prefix("::rust_ctypes")
        .clang_arg("-Iavr-libc/include")
        .clang_arg("-ffreestanding")
        .clang_arg(format!("-mmcu={}", avr_arch_name))
        .clang_arg("-lm");

    if let Some(define_name) = mcu_define_name() {
        builder = builder.clang_arg(format!("-D{}", define_name));
    }

    for header_path in base_headers(libc_dir) {
        builder = builder.header(header_path.display().to_string());
    }

    let bindings = builder.generate().expect("failed to create bindings");

    // Write the generated bindings to an output file.
    bindings
        .write_to_file(BINDINGS_DEST)
        .expect("could not write bindings to file");
}
