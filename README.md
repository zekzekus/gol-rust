### gol-rust

my simple and silly implementation of [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

Purpose is just playing with [rust](http://www.rust-lang.org).

## Requirements

- libtcod -> on mac os x its just `brew install libtcod`

## Build

- cargo build --release

## Run

- usage: bedelli width height rule seeder
- rule -> like b2s23. this is standard Conway's rule.
- seeder -> 0: random, 1: a glider, 2: One alive at the center, 3: five alive
  cells in the middle.
- ./target/release/bedelli 100 100 b2s23 0

## TODO 

- use [clap](https://github.com/kbknapp/clap-rs) for cli interface.
- add interaction. e.g. start with an empty board, add alive cells with mouse
  then start processing, pause, restart etc.
