//! Go/Baduk engine written in Rust.

extern crate regex;
extern crate crossterm;

mod board;
mod game;

pub use crate::game::Game;
