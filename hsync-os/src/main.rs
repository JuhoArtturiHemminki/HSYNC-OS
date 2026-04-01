#![no_std]
#![no_main]

use core::panic::PanicInfo;
use hsync::vga_buffer;
use hsync::allocator::{ALLOCATOR, HEAP_START, HEAP_SIZE};
use hsync::interrupts;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga_buffer::print_error("HSYNC-OS PANIC!");
    loop {
        x86_64::instructions::hlt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let allocator = &ALLOCATOR as *const _ as *mut 
hsync::allocator::LockedHeap;
        (*allocator).init(HEAP_START, HEAP_SIZE);
    }

    interrupts::init_idt();
    
    x86_64::instructions::interrupts::enable();

    vga_buffer::print_hsync_header();
    vga_buffer::print_error("WAITING FOR BIT INPUT (0 or 1)...");

    loop {
        x86_64::instructions::hlt();
    }
}

