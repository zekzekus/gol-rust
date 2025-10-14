### gol-rust

my simple and silly implementation of [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

Purpose is just playing with [rust](http://www.rust-lang.org).

## Requirements

- Nix with flakes enabled (for development environment)
- X11 display server (Wayland has known issues with bracket-lib)

## Build

- cargo build --release

## Run

- usage: bedelli width height rule seeder
- rule -> like b3s23. this is standard Conway's rule.
- seeder -> 0: random, 1: a glider, 2: One alive at the center, 3: five alive
  cells in the middle.
- `WINIT_UNIX_BACKEND=x11 ./target/release/bedelli 80 50 b3s23 1`

**Note:** If running on Wayland, you must set `WINIT_UNIX_BACKEND=x11` to force X11 mode.

## TODO 

- use [clap](https://github.com/kbknapp/clap-rs) for cli interface.
- add interaction. e.g. start with an empty board, add alive cells with mouse
  then start processing, pause, restart etc.
