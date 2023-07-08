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
    loop {}
}

pub fn draw_bootscreen() {
    let mut console = console::Console {
        windows: [
            console::Window {
                top: 0,
                cur: 0,
                bg_color: vga_buffer::Color::DarkGrey,
                fg_color: vga_buffer::Color::White,
            },
            console::Window {
                top: 1,
                cur: 0,
                bg_color: vga_buffer::Color::Black,
                fg_color: vga_buffer::Color::Cyan,
            },
            console::Window {
                top: 21,
                cur: 0,
                bg_color: vga_buffer::Color::DarkGrey,
                fg_color: vga_buffer::Color::Yellow,
            },
        ],
    };

    console.clear_window(0);
    console.write_string(
        0,
        "rustOS [v0.0.1]                                                                 ",
    );

    let mut line_num = 1;
    while line_num < vga_buffer::Writer::get_height() {
        console.write_string(1, "\n");
        line_num += 1;
    }

    console.write_string(
        2,
        "                            Copyright (c) 2023 Brian Szmyd, All rights reserved.",
    );
}
