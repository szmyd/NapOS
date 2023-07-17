#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod console;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_print!("failed!");
    serial_println!("\n{}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    use x86_64::instructions::port::Port;
    println!("Starting NapOS...");

    print!("Initializing Interrupts.");
    interrupts::init_idt();
    print!(".");
    unsafe { interrupts::PICS.lock().initialize() };
    print!(".");
    x86_64::instructions::interrupts::enable();
    println!("done.");

    print!("Initializing RTC.");
    x86_64::instructions::interrupts::without_interrupts(|| unsafe {
        print!(".");
        let mut rtc_a = Port::new(0x70);
        let mut rtc_b = Port::new(0x71);
        rtc_a.write(0x8A as u8);
        let mut prev: u8 = rtc_b.read();
        print!(".");
        rtc_a.write(0x8A as u8);
        rtc_b.write((prev & 0xF0) | 0x0B);
        rtc_a.write(0x8B as u8);
        prev = rtc_b.read();
        print!(".");
        rtc_a.write(0x8B as u8);
        rtc_b.write(prev | (0x40 as u8));
    });
    println!("done.");
    subtext!(
        "{:29}{}",
        "\r",
        "Copyright (c) 2023 Brian Szmyd, All rights reserved."
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
