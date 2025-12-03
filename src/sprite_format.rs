use std::ops::Range;
use bevy::prelude::Resource;
use egui::TextBuffer;

#[derive(Resource, Debug, Default)]
pub struct Sprite {
    pub height: IntInput,
    pub width: IntInput,
    pub data: Option<Vec<SpriteFrame>>,
    pub ind: u16,
}



impl Sprite {
    pub fn add_frame(&mut self) {
        match self.data.as_mut() {
            None => {
                self.data = Some(vec![SpriteFrame::default()]);
            }
            Some(ok) => {
                ok.push(SpriteFrame::default());
            }
        }
    }


    pub fn get_frame_count(&self) -> usize {
        self.data.as_ref().map(|v| v.len()).unwrap_or(0)
    }

    pub fn move_ind(&mut self, ind: u16) {
        if let Some(ref frames) = self.data {
            let count = frames.len() as u16;

            if ind < count {
                self.ind = ind;
            }
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

#[derive(Default, Clone, Debug)]
pub struct SpriteFrame {
    pub frame: Vec<Vec<TerminalChar>>,
}


#[derive(Default, Copy, Clone, Debug)]
pub struct TerminalChar {
    pub char: u8,
    pub foreground: RGB,
    pub background: RGB,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}