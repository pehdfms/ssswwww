use std::process::Command;

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::common::Wallpaper;

#[derive(Debug, Clone, ValueEnum)]
pub enum TransitionType {
    Simple,
    Fade,
    Left,
    Right,
    Top,
    Bottom,
    Wipe,
    Wave,
    Grow,
    Center,
    Any,
    Outer,
    Random,
}

impl ToString for TransitionType {
    fn to_string(&self) -> String {
        match self {
            TransitionType::Simple => "simple",
            TransitionType::Fade => "fade",
            TransitionType::Left => "left",
            TransitionType::Right => "right",
            TransitionType::Top => "top",
            TransitionType::Bottom => "bottom",
            TransitionType::Wipe => "wipe",
            TransitionType::Wave => "wave",
            TransitionType::Grow => "grow",
            TransitionType::Center => "center",
            TransitionType::Any => "any",
            TransitionType::Outer => "outer",
            TransitionType::Random => "random",
        }
        .to_string()
    }
}

#[derive(Default, Args)]
pub struct TransitionOptions {
    pub transition_type: Option<TransitionType>,
    pub transition_duration: Option<u32>,
}

pub struct WallpaperDaemon {}

impl WallpaperDaemon {
    pub fn new() -> Result<WallpaperDaemon> {
        println!("Starting wallpaper daemon...");
        Command::new("swww").arg("init").output()?;

        Ok(WallpaperDaemon {})
    }

    pub fn set_wallpaper(&self, wallpaper: &Wallpaper, transition_options: &TransitionOptions) {
        println!("Setting {} as wallpaper...", wallpaper.filename());
        let mut command = Command::new("swww");

        let transition_type = match &transition_options.transition_type {
            Some(transition_type) => transition_type.to_string(),
            None => "none".to_string(),
        };

        command.args(["img", &wallpaper.to_string()]);
        command.args(["--transition-type", &transition_type]);

        if let Some(transition_duration) = transition_options.transition_duration {
            command.args(["--transition-duration", &transition_duration.to_string()]);
        }

        command.output().unwrap();
    }
}
