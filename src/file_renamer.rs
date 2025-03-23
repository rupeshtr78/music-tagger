use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::fs::read_dir;
use std::fs::rename;

pub fn rename_files(path: &str, file_type: &str, current: &str, new: &str) -> Result<()> {
    let dir = read_dir(path).with_context(|| "Failed to read directory")?;
    for entry in dir {
        let entry = entry.context("Failed to read entry")?;
        let path = entry.path();
        let file_name = path.file_name();

        let file_name = file_name
            .and_then(|f| f.to_str())
            .ok_or_else(|| anyhow::anyhow!("Failed to get file name"))?;

        // Skip files that are not mp3
        let file_extension = path.extension();
        if file_extension != Some(file_type.as_ref()) {
            debug!(
                "Skipping file: {} with {}",
                file_name,
                file_extension.unwrap().to_str().unwrap()
            );
            warn!("Skipping file: {}", file_name);
            continue;
        }

        // regexp match a string and replace it with another string
        if !file_name.contains(current) {
            warn!(
                "File name does not contain current {} string: {}",
                current, file_name
            );
            continue;
        }
        let new_name = file_name.to_string().replace(current, new);

        info!("Renaming file: {} to {}", file_name, new_name);

        rename(&path, path.with_file_name(new_name))
            .with_context(|| format!("Failed to rename file: {}", file_name))?;
    }
    info!("Renaming completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    fn create_test_files(
        dir_path: &Path,
        file_names: &[&str],
    ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        let mut file_paths = Vec::new();
        for file_name in file_names {
            let file_path = dir_path.join(file_name);
            let mut file = File::create(&file_path)?;
            writeln!(file, "Hello, world!")?;
            file_paths.push(file_path);
        }
        Ok(file_paths)
    }

    #[test]
    fn test_rename_all_files() -> Result<()> {
        let dir = tempdir()?;

        // Create test files using the helper function
        let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
        let created_files = create_test_files(dir.path(), &file_names)?;

        rename_files(dir.path().to_str().unwrap(), "mp3", "file", "new")?;

        for file in created_files {
            assert!(!(file).exists());
        }

        assert!(dir.path().join("new1.mp3").exists());
        assert!(dir.path().join("new2.mp3").exists());
        assert!(dir.path().join("new3.mp3").exists());

        dir.close()?;

        Ok(())
    }
    #[test]
    fn test_rename_files_with_invalid_file_type() -> Result<()> {
        let dir = tempdir()?;

        // Create test files using the helper function
        let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
        let created_files = create_test_files(dir.path(), &file_names)?;

        rename_files(dir.path().to_str().unwrap(), "txt", "file", "new")?;

        for file in created_files {
            assert!((file).exists());
        }

        dir.close()?;

        Ok(())
    }
    #[test]
    fn test_rename_some_files() {
        let dir = tempdir().unwrap();

        let file_names = ["no_rename1.mp3", "file2.mp3", "in_file_between.mp3"];
        let _created_files = create_test_files(dir.path(), &file_names).unwrap();

        rename_files(dir.path().to_str().unwrap(), "mp3", "file", "new").unwrap();

        assert!(dir.path().join("no_rename1.mp3").exists()); // should not be renamed
        assert!(dir.path().join("new2.mp3").exists());
        assert!(dir.path().join("in_new_between.mp3").exists()); // in between string should be renamed

        dir.close().unwrap();
    }
    #[test]
    fn test_with_invalid_file_name() -> Result<()> {
        let dir = tempdir()?;

        // Create test files using the helper function
        let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
        let created_files = create_test_files(dir.path(), &file_names)?;

        rename_files(dir.path().to_str().unwrap(), "mp3", "new", "new")?;

        for file in created_files {
            assert!((file).exists());
        }

        dir.close()?;

        Ok(())
    }
    #[test]
    fn test_with_invalid_path() -> Result<()> {
        let dir = tempdir()?;

        // Create test files using the helper function
        let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
        let created_files = create_test_files(dir.path(), &file_names)?;

        let res = rename_files("invalid_path", "mp3", "file", "new");
        assert!(res.is_err());

        for file in created_files {
            assert!((file).exists());
        }

        dir.close()?;

        Ok(())
    }
}
