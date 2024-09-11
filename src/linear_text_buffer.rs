use crate::uefi::TextOutputProtocol;
use spin::Lazy;

struct TextBuffer {
    chars: [u16; 256],
}

struct LinearTextWriter {
    buffer: TextBuffer,
    output: *const TextOutputProtocol,
}

impl LinearTextWriter {
    pub fn write(&mut self, string: &str) {
        string.chars().enumerate().for_each(|(i, c)| {
            self.buffer.chars[i] = c as u16;
        });
        unsafe {
            ((*self.output).output_string)(self.output, self.buffer.chars.as_ptr());
        }
    }
}

static WRITER: Lazy<LinearTextWriter> = Lazy::new(||LinearTextWriter {
    buffer: TextBuffer { chars: [0; 256] },
    output: 0 as *const TextOutputProtocol,
});
