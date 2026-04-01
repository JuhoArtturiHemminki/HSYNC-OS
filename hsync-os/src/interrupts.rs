use x86_64::structures::idt::{InterruptDescriptorTable, 
InterruptStackFrame};
use crate::vga_buffer;
use crate::hsync_transform;

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[33].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: 
InterruptStackFrame) {
    vga_buffer::print_error("EXCEPTION: BREAKPOINT");
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: 
InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: 
InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if scancode == 11 {
        let state = hsync_transform(0);
        vga_buffer::print_hsync_info(0, state);
    } else if scancode == 2 {
        let state = hsync_transform(1);
        vga_buffer::print_hsync_info(1, state);
    }

    unsafe {
        let mut pic_port = Port::new(0x20);
        pic_port.write(0x20u8);
    }
}

