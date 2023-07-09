use lazy_static::lazy_static;
use spin;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn set_bg_color(&mut self, color: Color) {
        self.color_code = ColorCode((color as u8) << 4 | (self.color_code.0 as u8) & 0x0f)
    }

    pub fn set_fg_color(&mut self, color: Color) {
        self.color_code = ColorCode((self.color_code.0 as u8) & 0xf0 | (color as u8))
    }

    pub fn get_color_code(&mut self) -> u8 {
        self.color_code.0
    }

    pub fn write_byte(&mut self, top: usize, height: usize, col: usize, byte: u8) -> usize {
        let mut new_col = col;
        match byte {
            b'\r' => 0,
            b'\n' => {
                self.new_line(top, height);
                0
            }
            byte => {
                if new_col >= BUFFER_WIDTH {
                    self.new_line(top, height);
                    new_col = 0;
                }

                let row = top + (height - 1);
                let color_code = self.color_code;

                self.buffer.chars[row][new_col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                new_col + 1
            }
        }
    }

    pub fn clear_screen(&mut self, top: usize, height: usize) {
        let mut line_num = top;
        while line_num < (top + height) {
            self.fill_line(line_num, b' ');
            line_num += 1;
        }
    }

    fn fill_line(&mut self, line_num: usize, byte: u8) {
        let old_color = self.get_color_code();
        let character = ScreenChar {
            ascii_character: byte,
            color_code: ColorCode(old_color),
        };
        self.buffer.chars[line_num].fill(character);
    }

    fn new_line(&mut self, top: usize, height: usize) {
        let last_line = top + height;
        let mut line_num = top + 1;
        while line_num < last_line {
            let (dest, src) = self.buffer.chars.split_at_mut(line_num);
            dest.last_mut().unwrap().copy_from_slice(&src[0]);
            line_num += 1;
        }
        self.fill_line(last_line - 1, b' ');
        self.column_position = 0;
    }

    pub fn write_string(&mut self, top: usize, height: usize, col: usize, s: &str) -> usize {
        let mut new_col = col;
        for byte in s.bytes() {
            new_col = match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(top, height, new_col, byte),
                // not part of printable ASCII range
                _ => self.write_byte(top, height, new_col, 0xfe),
            }
        }
        new_col
    }
}

lazy_static! {
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
