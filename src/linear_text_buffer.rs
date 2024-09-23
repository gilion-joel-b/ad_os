use crate::uefi::system_table;
use core::fmt;
use spin::{Lazy, Mutex};

struct TextBuffer {
    chars: [u16; 256],
}

pub struct LinearTextWriter {
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

impl fmt::Write for LinearTextWriter {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write(string);
        Ok(())
    }
}

pub static WRITER: Lazy<Mutex<LinearTextWriter>> = Lazy::new(|| {
    Mutex::new(LinearTextWriter {
        buffer: TextBuffer { chars: [0; 256] },
    })
});

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::linear_text_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

