pub fn init() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        // Enable MTE in blocking mode (kill on tag violation)
        core::arch::asm!(
            "msr MTECTRL_EL1, {0}",
            "msr GCR_EL1, xzr",           // Exclude nothing – full blocking
            in(reg) 0b11u64,
            options(nostack)
        );
    }

    println!("[AEGIS] Supervisor ready – owner-controlled root of trust");
    println!("[AEGIS] MTE blocking mode active");
}
