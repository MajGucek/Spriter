#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::Formatter;
use std::ops::Range;
use bevy::prelude::Resource;
use egui::TextBuffer;

#[derive(Resource, Debug, Default)]
pub struct Sprite {
    pub height: IntInput,
    pub width: IntInput,
    pub data: SpriteFrames,
    pub ind: Option<u16>,
}

#[derive(Debug, Default)]
pub struct SpriteFrames {
    pub frames: Vec<SpriteFrame>
}


impl fmt::Display for SpriteFrames {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               format!("[{}]",
                       self.frames.iter().map(|frame| {
                           format!("{}", frame)
                       }).collect::<Vec<_>>()
                           .join(", ")
               )
        )
    }
}


impl Sprite {
    pub fn add_frame(&mut self) {
        self.data.frames.push(SpriteFrame::default());
        match self.ind {
            None => { self.ind = Some(0); }
            Some(ok) => { self.ind = Some(ok + 1); }
        }
    }


    pub fn get_frame_count(&self) -> usize {
        self.data.frames.len()
    }

    pub fn move_ind(&mut self, ind: u16) -> Result<(), IndexMoveError> {
        if ind < self.get_frame_count() as u16 {
            self.ind = Some(ind);
            Ok(())
        } else {
            Err(IndexMoveError::IndexOutOfBounds(ind))
        }
    }
}

#[derive(Debug)]
pub enum IndexMoveError {
    IndexOutOfBounds(u16),
}


#[derive(Default, Clone, Debug)]
pub struct SpriteFrame {
    pub frame: Vec<Vec<TerminalChar>>,
}

impl fmt::Display for SpriteFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               self.frame.iter().map(|row | {
                   format!("[{}]",
                           row.iter().map(|term_c| {
                               term_c.char.to_string()
                           }).collect::<Vec<String>>()
                               .join(", ")
                   )
               }).collect::<Vec<String>>()
                   .join(", ")
        )
    }
}


#[derive(Debug, Default, Copy, Clone)]
pub struct TerminalChar {
    pub char: u8,
}


impl TerminalChar {
    fn convert_char(c: char) -> u8 {
        if c.is_ascii() {
            c as u8
        } else {
            b' '
        }
    }

    pub fn from_char(ch: char) -> Self {
        TerminalChar {
            char: Self::convert_char(ch),
            //foreground: RGB::white(),
            //background: RGB::black(),
        }
    }
}




#[derive(Default, Copy, Clone, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn white() -> Self {
        RGB {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn black() -> Self {
        RGB {
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn grayscale(scale: u8) -> Self {
        RGB {
            r: scale,
            g: scale,
            b: scale,
        }
    }
}





#[derive(Debug, Default)]
pub struct IntInput {
    pub value: u16,
    formatted: String,
}
impl IntInput {
    pub fn assign(&mut self, b: u16) {
        self.value = b;
        self.formatted = b.to_string();
    }
}

impl TextBuffer for IntInput {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.formatted
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let digits: String = text.chars().filter(|c| c.is_numeric()).collect();
        let count = digits.len();

        self.formatted.insert_str(char_index, &digits);

        self.value = self.formatted.parse::<u16>().unwrap_or(0);

        count
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        let start_byte = self.formatted.char_indices()
            .nth(char_range.start)
            .map(|(i, _)| i)
            .unwrap_or(self.formatted.len());

        let end_byte = self.formatted.char_indices()
            .nth(char_range.end)
            .map(|(i, _)| i)
            .unwrap_or(self.formatted.len());

        self.formatted.replace_range(start_byte..end_byte, "");

        self.value = self.formatted.parse::<u16>().unwrap_or(0);
    }
}