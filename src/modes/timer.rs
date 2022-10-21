use crate::ModeTimer;

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
