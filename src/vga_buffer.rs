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

#[allow(dead_code)]
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

    pub fn reset_color_code(&mut self, code: u8) {
        self.color_code.0 = code;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        let mut line_num = 0;
        while line_num < BUFFER_HEIGHT {
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

    fn new_line(&mut self) {
        let mut line_num = 1;
        while line_num < BUFFER_HEIGHT {
            let (dest, src) = self.buffer.chars.split_at_mut(line_num);
            dest.last_mut().unwrap().copy_from_slice(&src[0]);
            line_num += 1;
        }
        self.fill_line(BUFFER_HEIGHT - 1, b' ');
        self.column_position = 0;
    }
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

pub fn draw_bootscreen() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.clear_screen();
    writer.set_bg_color(Color::DarkGrey);
    writer.write_string("rustOS [v0.0.1]                                                                ");
    writer.set_bg_color(Color::Black);
    let mut line_num = 1;
    while line_num < BUFFER_HEIGHT {
        writer.write_string("\n");
        line_num += 1;
    }
    writer.set_bg_color(Color::DarkGrey);
    writer.set_fg_color(Color::Yellow);
    writer.write_string("                            Copyright (c) 2023 Brian Szmyd, All rights reserved.");
}
