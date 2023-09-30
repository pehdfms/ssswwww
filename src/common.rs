use std::{fs, path::PathBuf};

use anyhow::{bail, Result};

const VALID_EXTENSIONS: [&str; 10] = [
    "jpg", "jpeg", "png", "gif", "pnm", "tga", "tiff", "webp", "bmp", "farbfeld",
];

pub struct Wallpaper {
    path: PathBuf,
}

impl Wallpaper {
    pub fn new(path: PathBuf) -> Result<Wallpaper> {
        let error_start = format!("Tried to create wallpaper from path {}, ", path.display());

        if !path.is_file() {
            bail!(error_start + "but it was not a file.")
        }

        if let Some(extension) = path.extension() {
            if !VALID_EXTENSIONS.contains(&extension.to_str().unwrap()) {
                bail!(error_start + "but it did not seem to have a file type supported by swww.")
            }
        } else {
            bail!(error_start + "but was unable to find out wallpaper file type.")
        }

        Ok(Wallpaper { path })
    }

    pub fn filename(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }
}

impl ToString for Wallpaper {
    fn to_string(&self) -> String {
        self.path
            .to_str()
            .expect("should have been checked by constructor")
            .to_string()
    }
}

pub struct WallpaperDirectory {
    path: PathBuf,
}

impl WallpaperDirectory {
    pub fn new(path: PathBuf) -> Result<WallpaperDirectory> {
        if !path.is_dir() {
            bail!(
                "Error: Tried to read path {} as directory, but was file instead.",
                path.display()
            )
        }

        Ok(WallpaperDirectory { path })
    }

    pub fn get_wallpapers(&self) -> Vec<Wallpaper> {
        fs::read_dir(&self.path)
            .unwrap()
            .filter_map(|entry| {
                entry
                    .ok()
                    .map(|file| Wallpaper::new(file.path()).ok())
                    .flatten()
            })
            .collect()
    }
}

impl ToString for WallpaperDirectory {
    fn to_string(&self) -> String {
        self.path
            .to_str()
            .expect("should have been checked by constructor")
            .to_string()
    }
}
