#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut kernel_vga = vga::VGA::new();
    kernel_vga.write("Hello there!");
    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
