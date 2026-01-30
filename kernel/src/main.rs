#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;
    let msg = b"Welcome to Hello World OS";

    for (i, &b) in msg.iter().enumerate() {
        unsafe {
            *vga.offset(i as isize * 2) = b;
            *vga.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
