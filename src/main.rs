extern crate getopts;
#[macro_use]
extern crate strum_macros;

mod qrcode;

use crate::qrcode::{ECLevel, QRCode, QRCodeOptions};
use getopts::Options;
use std::str::FromStr;
use std::{env, path::Path, path::PathBuf, process::exit};

pub struct CLIOptions {
    opts: QRCodeOptions,
    data: Vec<u8>,
    output_path: PathBuf,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} DATA [options]\nVersion: {}", program, VERSION);
    println!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} v{}", program, VERSION);
}

fn get_options() -> Options {
    let default_options = QRCode::default_options();
    let mut opts = Options::new();
    opts.optopt(
        "e",
        "eclevel",
        &format!(
            "error correction level ({}, {}, {}, {}) (default: {})",
            ECLevel::L,
            ECLevel::M,
            ECLevel::Q,
            ECLevel::H,
            default_options.ec_level
        ),
        "EC_LEVEL",
    );
    opts.optopt("o", "output", "output path for the image QR code", "OUTPUT");
    opts.optflag("t", "text", "embed the original data on the image QR code");

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version information");

    opts
}

fn parse_options() -> CLIOptions {
    let opts = get_options();
    let args: Vec<String> = env::args().collect();
    let program = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("v") {
        print_version(&program);
        exit(0);
    }

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        exit(0);
    }

    let default_options = QRCode::default_options();
    let ec_level = matches
        .opt_str("e")
        .and_then(|o| ECLevel::from_str(&o).ok())
        .unwrap_or(default_options.ec_level);

    let data = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        println!("Error: data is missing!");
        exit(0);
    };

    let path = match matches.opt_str("o") {
        Some(o) => o,
        None => {
            print_usage(&program, &opts);
            println!("Error: output path is missing!");
            exit(0);
        }
    };

    let embed = matches.opt_present("t");

    CLIOptions {
        opts: QRCodeOptions { ec_level, embed },
        data: data.as_bytes().to_vec(),
        output_path: PathBuf::from(path),
    }
}

fn main() {
    let opts = parse_options();

    let qr = QRCode::new(&opts.opts);
    qr.encode(&opts.data, &opts.output_path).unwrap();
}
