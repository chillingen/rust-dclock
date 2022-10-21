mod clock;

extern crate crossterm;

use std::time::Instant;
use std::{thread, time::Duration};

pub type ModeStateType<T> = T;
pub type ModeClock = Mode<Instant>;
pub type ModeTimer = Mode<[i64; 2]>;

pub struct Mode<T> {
    name: &'static str,
    state: ModeStateType<T>,
}

/// Constructs a new clock mode, which can be updated and shown.
/// The update function will return true if 1 second had passed,
/// which can be used to only print time every single second.
///
/// * Example
///
/// ```
/// let clock: ModeClock = modes::clock_new();
/// loop {
///     if clock_update(&mut clock) {
///         clock_show(&clock);
///     }
/// }
pub fn clock_new() -> ModeClock {
    ModeClock {
        name: "Clock",
        state: Instant::now(),
    }
}

/// Updates a clock struct.
///
/// * Arguments
///
/// `clock` - A mutable reference to your clock struct.
///
/// * Return
///
/// A boolean, which is true if a second had passed, otherwise false.
pub fn clock_update(clock: &mut Mode<Instant>) -> bool {
    if clock.state.elapsed().as_millis() >= 1000 {
        clock.state = Instant::now();
        return true;
    }
    false
}

/// Displays time.
/// Combine with `clock_update()` to display the time every
/// 1 second.
///
/// * Arguments
///
/// `clock` - An immutable refernce to your clock struct.
pub fn clock_show(clock: &Mode<Instant>) {
    print!("{esc}[3;1H", esc = 27 as char);

    let now = chrono::Local::now();
    let f = now.format("%H:%M:%S").to_string();

    clock::print_digits(&f);
}


/// Constructs a new timer mode. Just like the clock mode,
/// it can be updated, as well as shown.
///
/// * Arguments
///
/// `n` - The `timer_update` function will return true
/// for every `n` seconds that have passed.
///
/// * Example
///
/// ```
/// let timer: ModeTimer = modes::timer_new(5);
/// loop {
///     if timer_update(&mut timer) {
///         timer_show(&timer);
///     }
/// }
/// ```
pub fn timer_new(n: i64) -> ModeTimer {
    let now: i64 = chrono::Local::now().timestamp();
    ModeTimer {
        name: "Timer",
        state: [
            n,
            now + n,
        ], 
    }
}

/// Updates a timer struct.
/// 
/// * Arguments
///
/// `timer` - A mutable reference to your timer struct.
///
/// * Return
///
/// Returns a boolean which is true if `n` amount of
/// seconds have passed, otherwise false.
const INDEX_EVERY_SECONDS: usize = 0;
const INDEX_BEEPON: usize = 1;
pub fn timer_update(timer: &mut ModeTimer) -> bool {
    
    let now: i64 = chrono::Local::now().timestamp(); 

    if now >= timer.state[INDEX_BEEPON] {
        timer.state[INDEX_BEEPON] = now + timer.state[INDEX_EVERY_SECONDS];
        return true;
    }
    
    false
}

/// BEEP! Should be shown when `timer_update` returns
/// true.
///
/// * Arguments
///
/// `timer` - An immutable reference to your timer struct.
pub fn timer_show(timer: &ModeTimer) {
    println!(">> {}: BEEP! Time is: {} <<", timer.name, chrono::Local::now().to_rfc2822());
}

