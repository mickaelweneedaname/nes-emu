use js_sys::Intl::Collator;

use crate::traits::Device;

const MEMORY_RANGE: std::ops::Range<usize> = 0x0200..0x0600;
const NUMBER_OF_PIXELS: usize = 0x0600 - 0x0200;
const NUMBER_OF_PIXELS_PER_LINE: usize = 32;

pub enum Color {
    Black,
    White,
    Grey,
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
}

impl std::convert::From<u8> for Color {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Color::Black,
            1 => Color::White,
            2 | 9 => Color::Grey,
            3 | 10 => Color::Red,
            4 | 11 => Color::Green,
            5 | 12 => Color::Blue,
            6 | 13 => Color::Magenta,
            7 | 14 => Color::Yellow,
            _ => Color::Cyan,
        }
    }
}

impl std::convert::From<Color> for String {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => "#000000".to_owned(),
            Color::White => "#FFFFFF".to_owned(),
            Color::Grey => "#7F7F7F".to_owned(),
            Color::Red => "#FF0000".to_owned(),
            Color::Green => "#00FF00".to_owned(),
            Color::Blue => "#0000FF".to_owned(),
            Color::Magenta => "#FF00FF".to_owned(),
            Color::Yellow => "#FFFF00".to_owned(),
            Color::Cyan => "#00FFFF".to_owned(),
        }
    }
}

pub struct Screen {
    memory: [*mut u8; NUMBER_OF_PIXELS],
}

impl Screen {
    pub fn default() -> Self {
        Self {
            memory: [std::ptr::null_mut(); NUMBER_OF_PIXELS],
        }
    }

    pub fn get_screen_data(&self) -> Vec<Vec<u8>> {
        let mut data = Vec::new();
        let mut line = Vec::new();
        let mut col_count = 0;
        for byte in &self.memory {
            unsafe {
                line.push(**byte);
            }
            col_count += 1;
            if col_count == NUMBER_OF_PIXELS_PER_LINE {
                col_count = 0;
                data.push(line);
                line = Vec::new();
            }
        }
        data
    }
}

impl Device for Screen {
    fn mapping_def(&self) -> std::ops::Range<usize> {
        MEMORY_RANGE
    }

    fn map(&mut self, memory: &mut [u8]) {
        for (count, pointer) in self.memory.iter_mut().enumerate() {
            *pointer = &mut memory[count];
        }
    }

    /// # Safety
    /// Make sure that `memory` ptr is valid
    unsafe fn mem_read(&mut self) {}

    /// # Safety
    /// Make sure that `memory` ptr is valid
    unsafe fn mem_write(&mut self) {}
}
