# AEGIS OS v0.1 – Masters Edition

```ascii
 █████╗  ███████╗  ██████╗  ██╗ ███████╗
 ██╔══██╗ ██╔════╝ ██╔════╝  ██║ ██╔════╝
 ███████║ █████╗   ██║  ███╗ ██║ ███████╗
 ██╔══██║ ██╔══╝   ██║   ██║ ██║ ╚════██║
 ██║  ██║ ███████╗ ╚██████╔╝ ██║ ███████║
 ╚═╝  ╚═╝ ╚══════╝  ╚═════╝  ╚═╝ ╚══════╝
```
| Threat                          | iOS / Android / GrapheneOS          | **AEGIS v0.1**                                          |
|---------------------------------|-------------------------------------|----------------------------------------------------------|
| Kernel compromise               | Monolithic Linux kernel             | seL4 microkernel + 100 % Rust (mathematically isolated) |
| App memory exfiltration         | Shared kernel RAM                   | **ARM CCA Realms** – encrypted RAM per app             |
| Supply-chain compiler backdoor  | Trust Apple/Google LLVM             | On-device compiler runs in sealed Realm                |
| Forced corporate updates        | Yes                                 | **Owner-controlled Ed25519 key** – you sign everything |
| Modem baseband attacks          | Shared memory with modem            | Modem isolated in its own Realm (v0.2)                  |
| Memory corruption exploits      | ASLR + PAC only                     | **MTE in blocking mode** + zero unsafe Rust           |

## Current Status – v0.1 (November 2025)

- Boots on real ARMv9-CCA hardware (Qualcomm RB6 primary target)
- Verified boot with **your** owner key
- Memory Tagging Extension in **blocking** mode
- Full `realm::spawn_realm()` CCA primitive working
- Built-in compiler Realm (source never leaves encrypted RAM)
- `aegis>` shell with `compile`, `realm`, `sign`, `flash` commands
- Zero telemetry, zero cloud services

## Supported Hardware

| Device                        | CCA/RME | MTE | Price           | Status                  |
|-------------------------------|---------|-----|-----------------|-------------------------|
| Qualcomm RB6 (QCS6490)        | Yes     | Yes | ~$1,200         | Primary – fully working |
| Google Pixel 9 / 9 Pro XL     | Yes     | Yes | $799–$1,099     | Experimental boot       |
| Future custom RISC-V SoC      | Yes     | Yes | 2027+           | Phase 3                 |
