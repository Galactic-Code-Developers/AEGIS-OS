//! Aegis OS v0.1 – Userspace shell (runs inside a sealed Realm)

#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

static mut STDOUT: Option<ShellWriter> = None;

struct ShellWriter;
impl Write for ShellWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            unsafe { core::ptr::write_volatile(0x1000_0000 as *mut u8, b); }
        }
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => {{
        unsafe {
            if let Some(w) = STDOUT.as_mut() {
                let _ = write!(w, $($arg)*);
            }
        }
    }};
}

macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => { print!($($arg)*); print!("\n"); }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { STDOUT = Some(ShellWriter); }

    println!(
        r"
    █████╗ ███████╗ ██████╗ ██╗███████╗
    ██╔══██╗██╔════╝██╔════╝ ██║██╔════╝
    ███████║█████╗  ██║  ███╗██║███████╗
    ██╔══██║██╔══╝  ██║   ██║██║╚════██║
    ██║  ██║███████╗╚██████╔╝██║███████║
    ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝╚══════╝
    "
    );

    println!("Aegis v0.1 – Realm shell ready");
    println!("Type 'help' for commands\n");

    loop {
        print!("aegis> ");
        let cmd = read_line();
        execute(cmd.trim());
    }
}

fn read_line() -> &'static str {
    // In real Realm we would read from UART via hypercall
    // For bootstrap we just simulate a few commands
    static COMMANDS: [&str; 5] = [
        "help",
        "realm ls",
        "compile hello.rs",
        "sign update.img",
        "reboot",
    ];
    static mut INDEX: usize = 0;
    unsafe {
        let cmd = COMMANDS[INDEX % COMMANDS.len()];
        INDEX += 1;
        cmd
    }
}

fn execute(cmd: &str) {
    match cmd {
        "help" => {
            println!("Available commands:");
            println!("  help        – show this message");
            println!("  realm ls    – list active Realms");
            println!("  compile <f> – compile in sealed compiler Realm");
            println!("  sign <f>    – sign image with owner key");
            println!("  reboot      – reboot device");
        }
        "realm ls" => {
            println!("Active Realms:");
            println!("  0xAEG15_0001  compiler   (encrypted, attested)");
            println!("  0xAEG15_0002  shell      (you are here)");
        }
        "compile hello.rs" => {
            println!("[compiler-realm] Compiling hello.rs → success");
            println!("[compiler-realm] Output sealed – kernel cannot read");
        }
        "sign update.img" => {
            println!("[bootchain] Signing update.img with owner key... done");
        }
        "reboot" => {
            println!("Rebooting – owner signature verified");
            println!("Aegis never sleeps.");
        }
        _ => println!("Unknown command – type 'help'"),
    }
    println!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Shell panic – this Realm is sealed and cannot be inspected");
    loop {}
}
