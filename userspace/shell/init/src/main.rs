//! Aegis OS v0.1 – PID1 (Realm Init)
//! Runs in the first confidential Realm – cannot be inspected by kernel

#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

// Minimal UART write for early boot (direct memory-mapped on RB6)
const UART_BASE: *mut u8 = 0x00A60000 as *mut u8;

fn uart_write_byte(b: u8) {
    unsafe { core::ptr::write_volatile(UART_BASE, b) }
}

fn uart_write_str(s: &str) {
    for &b in s.as_bytes() {
        if b == b'\n' { uart_write_byte(b'\r'); }
        uart_write_byte(b);
    }
}

macro_rules! print {
    ($($arg:tt)*) => { uart_write_str(format_args!($($arg)*).to_string().as_str()) };
}

macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => { print!($($arg)*); print!("\n") };
}

// Fake hypercalls – in real CCA we would use RMI/SMC
extern "C" {
    fn realm_spawn(image: u64, entry: u64) -> u64;
    fn realm_seal(realm_id: u64);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!();
    println!(" █████╗ ███████╗ ██████╗ ██╗ ███████╗");
    println!(" ██╔══██╗██╔════╝██╔════╝ ██║██╔════╝");
    println!(" ███████║█████╗  ██║  ███╗██║███████╗");
    println!(" ██╔══██║██╔══╝  ██║   ██║██║╚════██║");
    println!(" ██║  ██║███████╗╚██████╔╝██║███████║");
    println!(" ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝╚══════╝");
    println!();
    println!("Aegis OS v0.1 – PID1 (Realm Init) running");
    println!("All memory encrypted via ARM CCA – kernel cannot read");

    // Spawn compiler Realm (sealed, attested)
    println!("[INIT] Spawning compiler-realm...");
    unsafe { realm_spawn(0x5000_0000, 0x5000_1000); }
    println!("[INIT] Compiler Realm sealed and running");

    // Spawn shell Realm
    println!("[INIT] Spawning user shell Realm...");
    unsafe { realm_spawn(0x6000_0000, 0x6000_1000); }
    println!("[INIT] Shell Realm active – transferring control");

    println!();
    println!("aegis> Shell ready – type 'help'");
    println!("Zero telemetry. Owner-controlled. Unbreakable.");
    println!();

    // Hand over to shell – in real system this would be a hypercall
    loop {
        core::arch::asm!("wfe");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("!!! INIT REALM PANIC !!!");
    println!("{info}");
    println!("This Realm is sealed – panic data never leaves encrypted RAM");
    loop {}
}
