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
    pub frames: Vec<Vec<Vec<u8>>>
}


impl Sprite {
    pub fn add_frame(&mut self) {
        self.data.frames.push(Vec::new());
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
            Err(IndexMoveError::IndexOutOfBounds)
        }
    }
}

#[derive(Debug)]
pub enum IndexMoveError {
    IndexOutOfBounds,
}


#[derive(Debug, Resource, Default)]
pub struct MultilineString {
    pub width: u16,
    pub height: u16,
    pub text: String,
}

impl TextBuffer for MultilineString {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.text.insert_text(text, char_index)
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        self.text.delete_char_range(char_range);
    }
}



#[derive(Debug, Default)]
pub struct IntInput {
    pub value: u16,
    formatted: String,
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