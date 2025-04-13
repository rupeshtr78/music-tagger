use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command};
use dialoguer::{Input, Select};

#[derive(Debug)]
pub struct RenameCommandArgs {
    pub path: String,
    pub file_type: String,
    pub current: String,
    pub new: String,
    pub loglevel: String,
}

pub fn cli() -> Result<RenameCommandArgs> {
    let matches = Command::new("file-rename")
        .version("0.1.0")
        .author("Rupesh Raghavam <rupeshtr78@example.com>")
        .about("Renames files in a directory matching string")
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .value_name("INTERACTIVE")
                .help("Use interactive mode")
                .default_value("false")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to the directory containing files")
                .required_unless_present("interactive"),
        )
        .arg(
            Arg::new("file_type")
                .short('t')
                .long("file_type")
                .value_name("FILE_TYPE")
                .help("File type to rename")
                .required_unless_present("interactive"),
        )
        .arg(
            Arg::new("current")
                .short('c')
                .long("current")
                .value_name("CURRENT")
                .help("Current string to replace")
                .required_unless_present("interactive"),
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
                .default_value("info")
                .action(clap::ArgAction::Set),
        )
        .get_matches();

    let interactive = matches
        .get_one::<String>("interactive")
        .context("Failed to get interactive flag")?
        .parse::<bool>()
        .context("Failed to parse interactive flag")?;

    match interactive {
        true => cli_rename(),
        false => cli_commands(matches),
    }
}

pub fn cli_rename() -> Result<RenameCommandArgs> {
    let path = Input::new()
        .with_prompt("Enter the path to the directory containing files")
        .interact_text()?;

    let file_type = Input::new()
        .with_prompt("Enter the file type to rename")
        .interact_text()?;

    let current = Input::new()
        .with_prompt("Enter the current string to replace")
        .interact_text()?;

    let new = Input::new()
        .with_prompt("Enter the new string to replace with")
        .allow_empty(true)
        .interact_text()?;

    let loglevel_options = vec!["info", "debug", "error", "warn"];
    let loglevel_index = Select::new()
        .with_prompt("Choose a log level")
        .items(&loglevel_options)
        .default(loglevel_options.iter().position(|&x| x == "info").unwrap())
        .interact()?;
    let loglevel = loglevel_options[loglevel_index].to_string();

    Ok(RenameCommandArgs {
        path,
        file_type,
        current,
        new,
        loglevel,
    })
}

fn cli_commands(matches: ArgMatches) -> Result<RenameCommandArgs> {
    Ok(RenameCommandArgs {
        path: matches
            .get_one::<String>("path")
            .context("Missing path")?
            .to_string(),
        file_type: matches
            .get_one::<String>("file_type")
            .context("Missing file type")?
            .to_string(),
        current: matches
            .get_one::<String>("current")
            .context("Missing current string")?
            .to_string(),
        new: matches
            .get_one::<String>("new")
            .context("Missing new string")?
            .to_string(),
        loglevel: matches
            .get_one::<String>("loglevel")
            .context("Missing log level")?
            .to_string(),
    })
}

pub fn SetLogLevel(level: &str) -> Result<()> {
    match level {
        "info" => simple_logger::init_with_level(log::Level::Info),
        "debug" => simple_logger::init_with_level(log::Level::Debug),
        "error" => simple_logger::init_with_level(log::Level::Error),
        _ => simple_logger::init_with_level(log::Level::Warn),
    }
    .context("Failed to initialize logger")?;

    Ok(())
}
