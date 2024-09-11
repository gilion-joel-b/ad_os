use crate::uefi::{system_table, TextOutputProtocol};
use spin::Lazy;

struct TextBuffer {
    chars: [u16; 256],
}

struct LinearTextWriter {
    buffer: TextBuffer,
}

impl LinearTextWriter {
    pub fn write(&mut self, string: &str) {
        string.chars().enumerate().for_each(|(i, c)| {
            self.buffer.chars[i] = c as u16;
        });
        let output = system_table().output;
        unsafe {
            ((*output).output_string)(output, self.buffer.chars.as_ptr());
        }
    }
}

pub static WRITER: Lazy<LinearTextWriter> = Lazy::new(||LinearTextWriter {
    buffer: TextBuffer { chars: [0; 256] },
});
