#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos::console;
use rustos::vga_buffer;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    {
        let mut console = console::CONSOLE.lock();
        console.set_fg_color(1, vga_buffer::Color::Red);
    }
    rustos::println!("\n{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    draw_bootscreen();

    #[cfg(test)]
    test_main();

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
