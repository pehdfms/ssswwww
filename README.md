# Stupid Simple SWWW Wrapper
### SWWW abstracted for common usecases

## Dependencies

 - [swww](https://github.com/Horus645/swww)
 - whatever swww depends on

## Build

### Dependencies:

  - rustc compiler and cargo

To build, clone this repository and run:
```
cargo build --release
```
Then put the resulting binary in your path.

## Features

 - Set specific wallpaper
 - Set random wallpaper from folder
 - Cycle through wallpapers in a folder
    - Random
    - Shuffle (pseudo-random, no repeats, default)
    - Date (most recent first)

## Why

swww is a great wallpaper daemon, but it can feel a little too simple when it
doesn't abstract away common user stories, like being able to "set and forget"
a wallpaper folder and have the daemon toil away in the background changing your
wallpaper. If you don't want to write your own shell script to handle your wallpapers,
this might be the program for you!

## Usage

```
# Set file as wallpaper
ssswwww file.png

# Set random wallpaper from folder
ssswwww folder

# Cycle wallpapers in folder (no default transition, 60s default time between wallpaper)
ssswwww folder --order random # Randomly selects permanent wallpaper in folder
ssswwww folder --order shuffle --timer 15s # Pseudorandom, no default transition, change wallpaper every 15s
ssswwww folder --order descdate --timer 15s --transition center # Date (descending), 15s timer, apply center transition

# Transitions
ssswwww folder --order shuffle -e center --transition-step 30 // Transition Step (smaller = smoother), default = 20
ssswwww folder --order shuffle -e wipe --transition-fps 60 // Transition Frame Rate, default = 30

```

## TODO

  - Add all options from swww

## Acknowledgments

All props should go to [Horus645](https://github.com/Horus645) for creating the original
[swww](https://github.com/Horus645/swww) utility, ssswwww is in no way dissing his work.
