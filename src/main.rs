#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(napos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use napos::console;
use napos::vga_buffer::Color;
use napos::println;
use napos::print;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    {
        let mut console = console::CONSOLE.lock();
        console.set_fg_color(1, Color::Red);
    }
    println!("\n{}", info);
    napos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    napos::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    napos::init();
    draw_bootscreen();

    #[cfg(test)]
    test_main();

    print!("Starting NapOS...");
    napos::hlt_loop();
}

pub fn draw_bootscreen() {
    let mut console = console::CONSOLE.lock();
    console.set_bg_color(0, Color::DarkGrey);
    console.set_fg_color(0, Color::White);
    console.clear_window(0);
    console.write_string(0, "NapOS [v0.0.2]");
    console.set_bg_color(1, Color::Black);
    console.set_fg_color(1, Color::Cyan);
    console.clear_window(1);
    console.set_bg_color(2, Color::DarkGrey);
    console.set_fg_color(2, Color::Yellow);
    console.write_string(
        2,
        "                            Copyright (c) 2023 Brian Szmyd, All rights reserved.",
    );
}
