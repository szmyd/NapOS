use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::announce;
use crate::print;
use crate::println;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    RTC = PIC_2_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // CPU Exception Handlers
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        // PIC Handlers
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
        idt[InterruptIndex::RTC.as_usize()].set_handler_fn(rtc_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("CPU: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("CPU: DOUBLE FAULT[{:#?}]\n{:#?}", error_code, stack_frame);
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn rtc_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let hour: u8;
    let min: u8;
    let sec: u8;
    unsafe {
        let mut rtc_a = Port::new(0x70);
        let mut rtc_b = Port::new(0x71);
        rtc_a.write(0x00 as u8);
        sec = rtc_b.read() % 60;
        rtc_a.write(0x02 as u8);
        min = rtc_b.read() % 60;
        rtc_a.write(0x04 as u8);
        hour = rtc_b.read() % 12;
        rtc_a.write(0x0C as u8);
        let _prev: u8 = rtc_b.read();
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::RTC.as_u8());
    }
    announce!(
        "\r {:68}{}",
        "NapOS [v0.0.2]",
        format_args!("[{:02}:{:02}:{:02}]", hour, min, sec)
    );
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
