use ::clap::Command;
use anyhow::{Context, Result};
use clap::Arg;

#[derive(Debug)]
pub struct CliArgs {
    pub path: String,
    pub file_type: String,
    pub current: String,
    pub new: String,
    pub loglevel: String,
}

pub fn cli() -> Result<CliArgs> {
    let matches = Command::new("mp3-rename")
        .version("0.1.0")
        .author("Rupesh Raghavam <rupeshtr78@example.com>")
        .about("Renames files in a directory matching string");
    let matches = matches
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to the directory containing mp3 files")
                .required(true),
        )
        .arg(
            Arg::new("file_type")
                .short('t')
                .long("file_type")
                .value_name("FILE_TYPE")
                .help("File type to rename")
                .required(true),
        )
        .arg(
            Arg::new("current")
                .short('c')
                .long("current")
                .value_name("CURRENT")
                .help("Current string to replace")
                .required(true),
        )
        .arg(
            Arg::new("new")
                .short('n')
                .long("new")
                .value_name("NEW")
                .help("New string to replace with")
                .default_value(""),
        )
        .arg(
            Arg::new("loglevel")
                .short('l')
                .long("loglevel")
                .help("Set log level")
                .default_value("info") // Default to "false" if the flag is not provided
                .action(clap::ArgAction::Set), // Allow the flag to take a value
        )
        .get_matches();

    // Get the values of the arguments
    let path = matches
        .get_one::<String>("path")
        .context("Failed to get path")?
        .to_string();
    let file_type = matches
        .get_one::<String>("file_type")
        .context("Failed to get file_type")?
        .to_string();
    let current = matches
        .get_one::<String>("current")
        .context("Failed to get current string")?
        .to_string();
    let new = matches
        .get_one::<String>("new")
        .context("Failed to get new")?
        .to_string();
    let loglevel = matches
        .get_one::<String>("loglevel")
        .context("Failed to get loglevel")?
        .to_string();

    Ok(CliArgs {
        path,
        file_type,
        current,
        new,
        loglevel,
    })
}

pub fn SetLogLevel() {
    let args = cli().unwrap();
    let level = args.loglevel.as_str();
    match level {
        "info" => simple_logger::init_with_level(log::Level::Info).unwrap(),
        "debug" => simple_logger::init_with_level(log::Level::Debug).unwrap(),
        "error" => simple_logger::init_with_level(log::Level::Error).unwrap(),
        _ => simple_logger::init_with_level(log::Level::Warn).unwrap(),
    }
}
