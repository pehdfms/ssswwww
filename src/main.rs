use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Args, Parser};
use common::{ImageOrder, WallpaperDirectory};
use daemon::{TransitionOptions, TransitionType};
use humantime::Duration;
use manager::{TimerOptions, WallpaperManager};

use crate::common::Wallpaper;

mod common;
mod daemon;
mod manager;

#[derive(Args)]
struct TransitionArgs {
    #[arg(short, long)]
    transition_type: Option<TransitionType>,

    #[arg(long)]
    transition_duration: Option<u32>,
}

#[derive(Args)]
struct TimerArgs {
    #[arg(long)]
    timer: Option<Duration>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: PathBuf,

    #[command(flatten)]
    transition_args: TransitionArgs,

    #[command(flatten)]
    timer_args: TimerArgs,

    #[arg(long, short)]
    order: Option<ImageOrder>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = cli.path;

    if !path.exists() {
        bail!(
            "Could not find anything at path {}, are you sure it exists?",
            path.display()
        );
    }

    let manager = WallpaperManager::new()?;

    if path.is_file() {
        if cli.order.is_some() {
            println!("Warn: Order argument is only available when watching wallpaper folders");
        }

        let wallpaper = Wallpaper::new(path)?;
        manager.set_wallpaper(&wallpaper);
    } else {
        let directory = WallpaperDirectory::new(path)?;
        let transition_args = cli.transition_args;
        let timer_args = cli.timer_args;
        let order = cli.order.unwrap_or_default();

        manager.watch_wallpaper_folder(
            &directory,
            TransitionOptions {
                transition_type: transition_args.transition_type,
                transition_duration: transition_args.transition_duration,
            },
            TimerOptions {
                timer: timer_args.timer.map(|duration| duration.into()),
            },
            order,
        );
    }

    loop {}
}
