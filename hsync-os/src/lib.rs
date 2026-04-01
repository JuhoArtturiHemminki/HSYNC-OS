#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

pub mod vga_buffer;
pub mod allocator;
pub mod interrupts;

pub const PHI_FIXED: u128 = 117010293101834720255; 
pub const PHI_INV_FIXED: u128 = 72314541575087521792; 

#[inline(always)]
pub fn hsync_transform(input: u8) -> u128 {
    let mask = (input as u128).wrapping_neg();
    (mask & PHI_FIXED) | (!mask & PHI_INV_FIXED)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("HSYNC-OS: ALLOCATION ERROR {:?}", layout)
}

