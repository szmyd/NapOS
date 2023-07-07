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
    LightGray = 7,
    DarkGray = 8,
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

    fn new_line(&mut self) {
        let old_color = self.get_color_code();
        let mut line_num = 0;
        while line_num < (BUFFER_HEIGHT - 1) {
            let old_line = self.buffer.chars[line_num+1];
            self.buffer.chars[line_num].copy_from_slice(&old_line);
            line_num += 1;
        }
        let blank = ScreenChar { ascii_character: b' ', color_code: ColorCode(old_color) };
        self.buffer.chars[BUFFER_HEIGHT - 1].fill(blank);
        self.column_position = 0;
        self.reset_color_code(old_color);
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

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.set_fg_color(Color::Yellow);
    writer.write_string("1: Hello World!\n");
    writer.set_bg_color(Color::Yellow);
    writer.set_fg_color(Color::Green);
    writer.write_string("2: New line\n");
    writer.set_bg_color(Color::Blue);
    writer.write_string("3: Really Really Long Line that Would Extend past the end of the screen boundary of 80 columns");
}
