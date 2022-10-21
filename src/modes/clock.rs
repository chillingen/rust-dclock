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
            print!("{} ", buf);
        }
        println!("\x0d");
    } 
}