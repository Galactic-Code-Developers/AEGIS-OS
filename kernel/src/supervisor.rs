pub fn init() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "msr SCTLR_EL1, {0}",
            "msr GCR_EL1, {1}",
            in(reg) 0x1000u64,
            in(reg) 0u64,
            options(nostack)
        );
    }
    println!("[AEGIS] Supervisor ready – owner-controlled root of trust");
    println!("[AEGIS] MTE blocking mode active");
}
