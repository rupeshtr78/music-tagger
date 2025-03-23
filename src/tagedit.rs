use anyhow::{Context, Ok, Result};
use id3::{Tag, TagLike, Version};
use std::fs::{copy, File};

pub fn read_tags(file_path: &str) -> Result<()> {
    let temp_file = std::env::temp_dir().join("music.mp3");
    copy(file_path, &temp_file)?;

    let mut tag = Tag::read_from_path(&temp_file).context("Failed to read tags")?;

    tag.title().map(|t| println!("Title: {}", t));
    tag.artist().map(|a| println!("Artist: {}", a));
    tag.album().map(|a| println!("Album: {}", a));
    tag.genre().map(|g| println!("Genre: {}", g));

    tag.set_album("Fancy Album Title");

    tag.write_to_path(temp_file, Version::Id3v24)?;

    tag.album().map(|a| println!("Album: {}", a));

    // copy(&temp_file, file_path)?;

    Ok(())
}
