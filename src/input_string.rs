use std::ops::Range;
use bevy::reflect::List;
use egui::TextBuffer;

#[derive(Default, Debug)]
pub struct InputString{
    pub width: u16,
    pub height: u16,
    pub value: String,
    formatted: Vec<String>,
}

impl InputString {
    pub fn format_string(&self) -> Vec<String> {
        self.value.chars().collect::<Vec<char>>().chunks(self.width as usize)
            .map(|c| {
                c.iter().collect::<String>()
            })
            .collect::<Vec<String>>()
    }
}

impl TextBuffer for InputString {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        //self.formatted.iter().collect()
        self.value.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let s = self.value.insert_text(text, char_index);
        self.formatted = self.format_string();
        s
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        self.value.delete_char_range(char_range)
    }
}