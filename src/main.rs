#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), "Welcome to RSOS!").unwrap();
    write!(vga_buffer::WRITER.lock(), "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n").unwrap();
    
    // panic!("Some panic message");
    loop {}
}

mod vga_buffer;
