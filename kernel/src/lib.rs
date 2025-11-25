#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod realm;
mod supervisor;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    supervisor::init();
    realm::test_spawn();
    loop { core::arch::asm!("wfe") }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
