#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosmo_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cosmo_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    cosmo_os::init(); // new

    #[cfg(test)]
    test_main();

    println!("lol _start didn't crash");

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
    cosmo_os::test_panic_handler(info)
}