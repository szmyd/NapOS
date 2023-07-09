use crate::vga_buffer;
use core::fmt;
use lazy_static::lazy_static;
use spin;

#[derive(Debug)]
struct Window {
    pub top: usize,
    pub height: usize,
    pub cur: usize,
    pub bg_color: vga_buffer::Color,
    pub fg_color: vga_buffer::Color,
}

#[derive(Debug)]
pub struct Console {
    windows: [Window; 3],
}

#[allow(dead_code)]
impl Console {
    pub fn set_bg_color(&mut self, window_idx: usize, color: vga_buffer::Color) {
        let window = &mut self.windows[window_idx];
        window.bg_color = color;
    }

    pub fn set_fg_color(&mut self, window_idx: usize, color: vga_buffer::Color) {
        let window = &mut self.windows[window_idx];
        window.fg_color = color;
    }

    pub fn clear_window(&mut self, window_idx: usize) {
        let window = &mut self.windows[window_idx];
        let mut writer = vga_buffer::WRITER.lock();
        writer.set_fg_color(window.fg_color);
        writer.set_bg_color(window.bg_color);
        writer.clear_screen(window.top, window.height);
        window.cur = 0;
    }

    pub fn write_string(&mut self, window_idx: usize, s: &str) {
        let window = &mut self.windows[window_idx];
        let mut writer = vga_buffer::WRITER.lock();
        writer.set_fg_color(window.fg_color);
        writer.set_bg_color(window.bg_color);
        window.cur = writer.write_string(window.top, window.height, window.cur, s);
    }
}

lazy_static! {
    pub static ref CONSOLE: spin::Mutex<Console> = spin::Mutex::new(Console {
        windows: [
            Window {
                top: 0,
                height: 1,
                cur: 0,
                bg_color: vga_buffer::Color::Black,
                fg_color: vga_buffer::Color::White,
            },
            Window {
                top: 1,
                height: 23,
                cur: 0,
                bg_color: vga_buffer::Color::Black,
                fg_color: vga_buffer::Color::White,
            },
            Window {
                top: 24,
                height: 1,
                cur: 0,
                bg_color: vga_buffer::Color::Black,
                fg_color: vga_buffer::Color::White,
            },
        ],
    });
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(1, s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args).unwrap();
}
