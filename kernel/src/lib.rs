#![no_std]
#![no_main]
#![feature(asm_const, naked_functions)]

use core::arch::global_asm;
global_asm!(include_str!("entry.S"));

mod supervisor;
mod realm;

#[no_mangle]
pub unsafe extern "C" fn _start_rust() -> ! {
    supervisor::init();
    realm::spawn_compiler();
    realm::spawn_shell();

    println!("[AEGIS] All Realms spawned – entering idle");
    loop {
        core::arch::asm!("wfe");
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[PANIC] {info}");
    loop {}
}
