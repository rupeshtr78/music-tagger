use ::clap::Command;
use anyhow::{Context, Result};
use clap::Arg;

#[derive(Debug)]
pub struct CliArgs {
    pub path: String,
    pub file_type: String,
    pub current: String,
    pub new: String,
    pub verbose: String,
}

pub fn cli_args() -> Result<CliArgs> {
    let matches = Command::new("mp3-rename")
        .version("0.1.0")
        .author("Rupesh Raghavam <rupeshtr78@example.com>")
        .about("Renames mp3 files in a directory based on new string");
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
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print debug information verbosely")
                .value_parser(["true", "false"]) // Allow only "true" or "false" as values
                .default_value("false") // Default to "false" if the flag is not provided
                .action(clap::ArgAction::Set), // Allow the flag to take a value
        )
        .get_matches();

    let path = matches
        .try_get_one::<String>("path")
        .context("Failed to get path")?
        .ok_or_else(|| anyhow::anyhow!("Failed to get path"))?;
    let file_type = matches
        .try_get_one::<String>("file_type")
        .context("Failed to get file_type")?
        .ok_or_else(|| anyhow::anyhow!("Failed to get file_type"))?;
    let current = matches
        .try_get_one::<String>("current")
        .context("Failed to get current")?
        .ok_or_else(|| anyhow::anyhow!("Failed to get current"))?;
    let new = matches
        .try_get_one::<String>("new")
        .context("Failed to get new")?
        .ok_or_else(|| anyhow::anyhow!("Failed to get new"))?;
    let verbose = matches
        .try_get_one::<String>("verbose")
        .context("Failed to get verbose")?
        .ok_or_else(|| anyhow::anyhow!("Failed to get verbose"))?;

    // let verbose = verbose
    //     .parse::<bool>()
    //     .with_context(|| "Failed to parse verbose")?;
    let verbose = verbose == "false";

    let args = CliArgs {
        path: path.to_string(),
        file_type: file_type.to_string(),
        current: current.to_string(),
        new: new.to_string(),
        verbose: verbose.to_string(),
    };

    Ok(args)
}
