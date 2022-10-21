mod modes;
mod input;
use input::*;
use modes::*;

use crossterm::{event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers}, Result};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, style::Print};
use std::io::{stdout, Write};
use std::time::Duration;

enum DClockMode {
    ModeClock,
    ModeTimer,
    ModeAlarm,
}

fn init_screen() {
    execute!(stdout(), Clear(ClearType::All)); 
}

fn main() -> Result<()> {
    let mut clock: ModeClock = modes::clock_new();
    let mut timer: ModeTimer = modes::timer_new(5);

    let mut selected: DClockMode = DClockMode::ModeClock;

    let mut stdout = stdout();

    enable_raw_mode().unwrap();
    init_screen();
    
    loop {

        let clock_update_return = clock_update(&mut clock);
        let timer_update_return = timer_update(&mut timer);       

        match selected {
            DClockMode::ModeClock => if clock_update_return {
                clock_show(&clock);
            },
            DClockMode::ModeTimer => {
                let i = match get_user_numbers() {
                    Some(i) => i,
                    None => -1,
                };
                if i < 0 {
                    println!("Invalid input, try again.\x0d");
                }
                selected = DClockMode::ModeClock;
            } 
            DClockMode::ModeAlarm => todo!("alarm"),
        } 
        if timer_update_return {
            timer_show(&timer);
        }
        
        if poll(Duration::from_millis(100))? {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::NONE,..
                }) => selected = DClockMode::ModeClock,
                
                Event::Key(KeyEvent {
                    code: KeyCode::Char('t'),
                    modifiers: KeyModifiers::NONE,..
                }) => selected = DClockMode::ModeTimer,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,..
                }) => selected = DClockMode::ModeAlarm,                

                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,..
                }) => {
                    disable_raw_mode().unwrap();
                    return Ok(());
                },
                _ => (),
            }
        }
    }
}