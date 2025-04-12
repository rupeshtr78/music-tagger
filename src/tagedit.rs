use anyhow::{anyhow, Context, Ok, Result};
use id3::{Tag, TagLike, Version};
use log::debug;
use std::fs::copy;
use std::fs::read_dir;
use std::path::Path;

#[derive(Debug)]
pub struct TagEditArgs {
    pub file_type: String,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub genre: Option<String>,
}

impl TagEditArgs {
    pub fn new(file_type: &str, album: &str, artist: &str, title: &str, genre: &str) -> Self {
        TagEditArgs {
            file_type: file_type.to_string(),
            album: Some(album.to_string()),
            artist: Some(artist.to_string()),
            title: Some(title.to_string()),
            genre: Some(genre.to_string()),
        }
    }

    pub fn tag_all(&self, dir_path: &str) -> Result<()> {
        let dir = read_dir(dir_path).with_context(|| "Failed to read directory")?;
        for entry in dir {
            let entry = entry.with_context(|| "Failed to read entry")?;
            let file_path = entry.path();
            if file_path.is_file() {
                let file_extension = file_path.extension();
                // skip files check for valid file extension
                if file_extension != Some(&self.file_type.as_ref()) {
                    if let Some(file_path) = file_path.to_str() {
                        debug!(
                            "Skipping File : {:?} file type does not match {:?}",
                            file_path, &self.file_type
                        )
                    }
                    continue;
                }
                // Update tags
                if let Some(file_path) = file_path.to_str() {
                    EditTags(file_path, &self)?;
                }
            }
        }

        Ok(())
    }
}

pub fn EditTags(path: &str, args: &TagEditArgs) -> Result<()> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(anyhow!("File does not exist"));
    }
    let file_name = file_path
        .file_name()
        .context("Error getting filename")?
        .to_str()
        .context("Error converting filename to string")?;

    let temp_file = std::env::temp_dir().join(file_name);
    copy(file_path, &temp_file)?;

    let mut tag = Tag::read_from_path(&temp_file).context("Failed to read tags")?;

    if let Some(album) = &args.album {
        tag.set_album(album);
    }
    if let Some(artist) = &args.artist {
        tag.set_artist(artist);
    }
    if let Some(title) = &args.title {
        tag.set_title(title);
    }
    if let Some(genre) = &args.genre {
        tag.set_genre(genre);
    }

    tag.write_to_path(&temp_file, Version::Id3v24)?;
    copy(&temp_file, file_path)?;
    std::fs::remove_file(temp_file)?;

    Ok(())
}

pub fn print_tags(path: &str) -> Result<()> {
    // println!("After tag Update:");
    let new_tag = Tag::read_from_path(path).context("Failed to read tags")?;
    println!("Tags: {}", path);
    new_tag.title().map(|t| println!("  Title: {}", t));
    new_tag.artist().map(|a| println!("  Artist: {}", a));
    new_tag.album().map(|a| println!("  Album: {}", a));
    new_tag.genre().map(|g| println!("  Genre: {}", g));

    Ok(())
}

// // Add to cli.rs
// .arg(
//     Arg::new("album")
//         .long("album")
//         .value_name("ALBUM")
//         .help("Set album name")
// )
// .arg(
//     Arg::new("artist")
//         .long("artist")
//         .value_name("ARTIST")
//         .help("Set artist name")
// )
// .arg(
//     Arg::new("title")
//         .long("title")
//         .value_name("TITLE")
//         .help("Set song title")
// )
// .arg(
//     Arg::new("genre")
//         .long("genre")
//         .value_name("GENRE")
//         .help("Set genre")
// )
