use crate::vga_buffer;

#[derive(Debug)]
pub struct Window {
    pub top: usize,
    pub cur: usize,
    pub bg_color: vga_buffer::Color,
    pub fg_color: vga_buffer::Color,
}

#[derive(Debug)]
pub struct Console {
    pub windows: [Window; 3],
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
        let window = &self.windows[window_idx];
        let mut writer = vga_buffer::WRITER.lock();
        writer.set_fg_color(window.fg_color);
        writer.set_bg_color(window.bg_color);
        writer.clear_screen();
    }

    pub fn write_string(&mut self, window_idx: usize, s: &str) {
        let window = &mut self.windows[window_idx];
        let mut writer = vga_buffer::WRITER.lock();
        writer.set_fg_color(window.fg_color);
        writer.set_bg_color(window.bg_color);
        writer.write_string(s);
    }
}
