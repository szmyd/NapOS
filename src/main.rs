#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(napos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use napos::console;
use napos::println;
use napos::subtext;
use napos::vga_buffer::Color;

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
    init_console();
    napos::init();

    #[cfg(test)]
    test_main();

    napos::hlt_loop();
}

pub fn init_console() {
    let mut console = console::CONSOLE.lock();
    console.set_bg_color(0, Color::DarkGrey);
    console.set_fg_color(0, Color::White);
    console.clear_window(0);
    console.set_bg_color(1, Color::Black);
    console.set_fg_color(1, Color::Cyan);
    console.clear_window(1);
    console.set_bg_color(2, Color::DarkGrey);
    console.set_fg_color(2, Color::Yellow);
    console.clear_window(2);
}
