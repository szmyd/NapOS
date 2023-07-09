// No Standard Library and no c-runtime
#![no_std]
#![no_main]

mod console;
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    draw_bootscreen();
    print!("Hello World{}", "!");
    loop {}
}

pub fn draw_bootscreen() {
    let mut console = console::CONSOLE.lock();
    console.set_bg_color(0, vga_buffer::Color::DarkGrey);
    console.set_fg_color(0, vga_buffer::Color::White);
    console.clear_window(0);
    console.write_string(0, "rustOS [v0.0.2]");
    console.set_bg_color(1, vga_buffer::Color::Black);
    console.set_fg_color(1, vga_buffer::Color::Cyan);
    console.clear_window(1);
    console.set_bg_color(2, vga_buffer::Color::DarkGrey);
    console.set_fg_color(2, vga_buffer::Color::Yellow);
    console.write_string(
        2,
        "                            Copyright (c) 2023 Brian Szmyd, All rights reserved.",
    );
}
