use crossterm::event::{read, Event, KeyCode};

pub fn get_user_numbers() -> Option<i32> {
    let mut buf: Vec<char> = Vec::new();
    
    println!("seconds?>\x0d");
    
    'main: loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Enter {
                    break 'main;
                }
                if let KeyCode::Char(c) = key_event.code {       
                    buf.push(c);
                }
            },
            _ => (),
        }
    }
    
    let mut i = 0;
    let base: i32 = 10;
    for x in 0..buf.len() {
        let y = buf.pop().unwrap() as i32;
        match y {
            48..=57 => {
                i += (y - 48) * base.pow(x.try_into().unwrap()); 
            },
            _ => return None,
        }
    }

    Some(i)
}
