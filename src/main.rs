#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosmo_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::Page;

use cosmo_os::println;
use cosmo_os::task::{simple_executor::SimpleExecutor, Task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use cosmo_os::allocator;
    use cosmo_os::memory;
    use cosmo_os::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;
    //
    println!("Hello World{}", "!");
    // INIT EVERYTHING FROM LIB.RS
    cosmo_os::init();
    // MEMORY MAPPING
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    // ALLOCATOR
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // ASYNC TEST
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
    // TEST MAIN
    #[cfg(test)]
    test_main();
    // HLT LOOP
    println!("didn't crash!");
    cosmo_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cosmo_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cosmo_os::test_panic_handler(info)
}
