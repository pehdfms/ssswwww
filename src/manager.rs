use rand::seq::SliceRandom;
use std::{thread, time::Duration};

use anyhow::Result;

use crate::{
    common::{ImageOrder, Wallpaper, WallpaperDirectory},
    daemon::{TransitionOptions, WallpaperDaemon},
};

pub struct TimerOptions {
    pub timer: Option<Duration>,
}

pub struct WallpaperManager {
    daemon: WallpaperDaemon,
}

impl WallpaperManager {
    pub fn new() -> Result<WallpaperManager> {
        let daemon = WallpaperDaemon::new()?;
        Ok(WallpaperManager { daemon })
    }

    pub fn set_wallpaper(&self, wallpaper: &Wallpaper) {
        self.daemon
            .set_wallpaper(wallpaper, &TransitionOptions::default())
    }

    pub fn watch_wallpaper_folder(
        &self,
        directory: &WallpaperDirectory,
        transition_options: TransitionOptions,
        timer_options: TimerOptions,
        order: ImageOrder,
    ) {
        loop {
            let mut wallpapers = directory.get_wallpapers();
            match order {
                ImageOrder::Alphabetical => (),
                ImageOrder::Random => wallpapers.shuffle(&mut rand::thread_rng()),
            };

            if let Some(timer) = timer_options.timer {
                wallpapers.iter().for_each(|wallpaper| {
                    self.daemon.set_wallpaper(wallpaper, &transition_options);

                    if let Some(timer) = timer_options.timer {
                        thread::sleep(timer)
                    }
                });
            } else {
                self.daemon
                    .set_wallpaper(&wallpapers[0], &transition_options);
            }
        }
    }
}
