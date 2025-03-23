use anyhow::{Context, Ok, Result};
use id3::{Tag, TagLike, Version};
use std::fs::{copy, File};

pub fn GetTags(file_path: &str) -> Result<()> {
    let file_path = std::path::Path::new(file_path);
    let file_name = file_path
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("music.mp3"))
        .to_str()
        .unwrap_or("music.mp3");
    let temp_file = std::env::temp_dir().join(file_name);
    // let temp_file = std::path::Path::new("test/music.mp3");
    copy(file_path, &temp_file)?;

    let temp_file = temp_file.as_path();

    let mut tag = Tag::read_from_path(temp_file).context("Failed to read tags")?;

    println!("Before tag update:");
    tag.title().map(|t| println!("  Title: {}", t));
    tag.artist().map(|a| println!(" Artist: {}", a));
    tag.album().map(|a| println!("  Album: {}", a));
    tag.genre().map(|g| println!("  Genre: {}", g));

    let current_title = tag.title().unwrap_or("Unknown Title").to_string();
    tag.set_album("Tag updated Album Title-u");
    tag.set_artist("Tag updated Artist Name-u");
    tag.set_title(&current_title);
    tag.set_genre("Tag updated Genre-u");

    tag.write_to_path(temp_file, Version::Id3v24)?;

    copy(&temp_file, file_path)?;
    // delete temp file
    std::fs::remove_file(temp_file)?;

    println!("After tag Update:");
    let new_tag = Tag::read_from_path(file_path).context("Failed to read tags")?;
    new_tag.title().map(|t| println!("  Title: {}", t));
    new_tag.artist().map(|a| println!("  Artist: {}", a));
    new_tag.album().map(|a| println!("  Album: {}", a));
    new_tag.genre().map(|g| println!("  Genre: {}", g));
    Ok(())
}
