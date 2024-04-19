use std::io::{self, Write};
use crossterm::{style, terminal};
use crossterm::event::{read, KeyCode, KeyEvent, Event, KeyModifiers};
use crossterm::cursor;

use crate::Terminal;


pub struct Editor {
    should_quit: bool,
    terminal: Terminal
}

impl Editor {
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            if let Err(err) = self.refresh_screen() {
                die(err);
            }
            if self.should_quit {
                break;
            }
            if let Err(err) = self.process_keypress() {
                die(err);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        crossterm::queue!(
            io::stdout(), 
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        if self.should_quit {
            return crossterm::execute!(io::stdout(), style::Print("Goodbye.\r\n"));
        }
        self.draw_rows();
        crossterm::queue!(
            io::stdout(),
            cursor::MoveTo(0, 0)
        );
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            crossterm::queue!(
                io::stdout(),
                style::Print("~\r\n")
            );
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        if pressed_key.modifiers == KeyModifiers::CONTROL && 
            pressed_key.code == KeyCode::Char('q')
        {
            self.should_quit = true;
        }
        Ok(())
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}

fn read_key() -> Result<KeyEvent,std::io::Error> {
    loop {
        match read()? {
            Event::Key(key) => return Ok(key),
            _ => continue,
        }
    };
}

fn die(e: io::Error) {
    crossterm::execute!(
        io::stdout(), 
        terminal::Clear(terminal::ClearType::All)
    );
    panic!("{}", e);
}