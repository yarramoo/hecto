use std::io;
use crossterm::{cursor, event, style, terminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
}

