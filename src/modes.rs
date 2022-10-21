pub mod clock;
pub mod timer;

extern crate crossterm;

use std::time::Instant;

pub type ModeStateType<T> = T;
pub type ModeClock = Mode<Instant>;
pub type ModeTimer = Mode<[i64; 2]>;

pub struct Mode<T> {
    name: &'static str,
    state: ModeStateType<T>,
}
