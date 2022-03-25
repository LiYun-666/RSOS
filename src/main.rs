/*
 * @file    main.rs
 * @author  黎酝
 * @brief   main 程序
 */

#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(RSOS::test_runner)]

use core::panic::PanicInfo;
use RSOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    RSOS::init();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    loop {}
}
/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    RSOS::test_panic_handler(info)
}

