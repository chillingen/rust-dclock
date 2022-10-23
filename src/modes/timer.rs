use crate::ModeTimer;
use crate::modes::clock::print_digits;

use crossterm::{execute, style::Print, cursor::MoveTo};
use std::io::stdout;

use chrono::NaiveTime;

const TIMER_BOUND: [u16; 2] = [0, 7];

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
    
    if timer.state[INDEX_EVERY_SECONDS] < 0 {
        return false;
    }

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
    
    execute!(stdout(),
        MoveTo(TIMER_BOUND[0], TIMER_BOUND[1]),
    ).unwrap();
    
    if timer.state[INDEX_EVERY_SECONDS] < 0 {
        print_digits(&"00:00:00".to_string());
        return;
    }

    let beep_on = timer.state[INDEX_BEEPON];
    let now: i64 = chrono::Local::now().timestamp();
    
    let seconds_left = beep_on - now;

    let t: NaiveTime = NaiveTime::from_num_seconds_from_midnight(
        seconds_left.try_into().unwrap(),
        0
    );
    
    let t_formatted = t.format("%H:%M:%S").to_string();
    print_digits(&t_formatted);
}
