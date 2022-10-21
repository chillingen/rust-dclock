use crate::ModeClock;
use std::time::Instant;

pub const DIGITS: &[[&str; 12]; 5] =
&[
    ["   #", "    ", " ## ", " ## ", " ## ", "### ", "  # ", "####", " ###", "####", " ## ", " ## "],
    ["  ##", " ## ", "#  #", "  # ", "#  #", "   #", " #  ", "#   ", "#   ", "   #", "#  #", "#  #"],
    [" ## ", "    ", "#  #", "  # ", "  # ", " ## ", "####", "### ", "### ", "  # ", " ## ", " ###"],
    ["##  ", " ## ", "#  #", "  # ", " #  ", "   #", "  # ", "   #", "#  #", " #  ", "#  #", "   #"],
    ["#   ", "    ", " ## ", " ###", "####", "### ", "  # ", "### ", " ## ", "#   ", " ## ", "### "],
];

pub fn print_digits(s: &str) {
    for y in 0..5 {
        for x in s.chars() {
            let mut buf = String::new();
            if x == ':' {
                buf.push_str(DIGITS[y][1]);
            }
            if x == '/' {
                buf.push_str(DIGITS[y][0]);
            }
            match x.to_digit(10) {
                Some(n) => buf.push_str(DIGITS[y][(n + 2) as usize]),
                None => {},
            }
            execute!(stdout(),
                Print(format!("{}", buf))
            ).unwrap();
        }
        execute!(stdout(),
            Print("\n".to_string())
        ).unwrap();
    } 
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
pub fn clock_update(clock: &mut ModeClock) -> bool {
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
pub fn clock_show(clock: &ModeClock) {
    print!("{esc}[3;1H", esc = 27 as char);

    let now = chrono::Local::now();
    let f = now.format("%H:%M:%S").to_string();

    print_digits(&f);
}
