use env_logger::{Builder, Target};
use log::{debug, error, info, log_enabled, Level, LevelFilter};
use std::env;

use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use configparser::ini::Ini;

#[derive(Parser)]
#[clap(about, author, version, long_about = None)]
struct Args {
    // Location of credentials file
    #[clap(short, long, default_value_t = String::from(
        format!("{}/.aws/credentials", env::var("HOME").expect("$HOME is not set")))
    )]
    file_name: String,

    #[clap(short, long, default_value = "default")]
    profile: String,
}

fn main() {
    //env_logger::init();
    let mut builder = Builder::from_default_env();
    builder
        .target(Target::Stdout)
        .init();

    let args = Args::parse();

    let mut config = Ini::new();
    info!("Opening {}", &args.file_name);
    config.load(&args.file_name).unwrap();

    let clippy: String = read_clipboard();
    let splits = clippy.trim().split_ascii_whitespace();

    for s in splits {
        if s.find("AWS") != None {
            let e = s.split_once("=").unwrap();
            let lower = e.0.to_lowercase();
            let key = lower.as_str();
            let value: Option<String> = Some(String::from(e.1));
            config.set(&args.profile, key, value);
        }
    }
    config.write(&args.file_name).unwrap();
    info!("profile: {} added to: {}", &args.profile, &args.file_name);
}

fn read_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    String::from(ctx.get_contents().unwrap())
}

//fn first_name(x: &'static str) -> &'static str {
//    x
//}
