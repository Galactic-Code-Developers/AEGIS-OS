# AEGIS OS v0.1 – Masters Edition

```text
 █████╗ ███████╗ ██████╗ ██╗ ███████╗
 ██╔══██╗ ██╔════╝ ██╔════╝ ██║ ██╔════╝
 ███████║ █████╗  ██║  ███╗ ██║ ███████╗
 ██╔══██║ ██╔══╝  ██║   ██║ ██║ ╚════██║
 ██║  ██║ ███████╗ ╚██████╔╝ ██║ ███████║
 ╚═╝  ╚═╝ ╚══════╝  ╚═════╝  ╚═╝ ╚══════╝
```

The world’s first mobile OS with hardware-enforced confidential Realms, owner-controlled verified boot, and an on-device compiler that cannot leak source code.

## Threat Model – Nothing Else Comes Close

| Threat                         | iOS / Android / GrapheneOS     | **AEGIS v0.1**                                                     |
|--------------------------------|---------------------------------|---------------------------------------------------------------------|
| Kernel compromise              | Monolithic Linux kernel         | **seL4 microkernel + 100% Rust**                                    |
| App memory exfiltration        | Shared kernel RAM               | **ARM CCA Realms – encrypted RAM per app**                          |
| Supply-chain compiler backdoor | Trust Apple/Google LLVM         | **On-device compiler Realm**                                        |
| Forced corporate updates       | Yes                             | **Owner-controlled Ed25519 key – you sign everything**              |
| Modem/baseband attacks         | Shared memory with modem        | Modem isolated in its own Realm (v0.2)                              |
| Memory corruption exploits     | ASLR + PAC only                 | **MTE blocking mode + zero unsafe Rust**                            |

## Current Status – v0.1 (November 2025)

- Boots on real ARMv9-CCA hardware (Qualcomm RB6)
- Verified boot with **your** owner key
- MTE in **blocking mode**
- Full `realm::spawn_realm()` primitive functional
- Compiler Realm operational (encrypted RAM)
- `aegis>` shell ready
- Zero telemetry

## Supported Hardware

| Device                      | CCA/RME | MTE | Price         | Status                        |
|----------------------------|---------|-----|---------------|-------------------------------|
| Qualcomm RB6 (QCS6490)     | Yes     | Yes | ~$1,200       | **Primary – fully working**   |
| Google Pixel 9 / 9 Pro XL  | Yes     | Yes | $799–$1,099   | Experimental boot             |
| Future RISC‑V SoC          | Yes     | Yes | 2027+         | Phase 3                       |

## Quick Start

```bash
git clone https://github.com/aegis-os/aegis.git
cd aegis
rustup toolchain install nightly-2025-11-25
rustup target add aarch64-unknown-none
./scripts/build_all.sh --target rb6
sudo ./scripts/flash_rb6.sh aegis-boot.img
```

UART output:

```
[AEGIS] Supervisor ready – owner-controlled root of trust
[AEGIS] MTE blocking mode active
[AEGIS] Spawning confidential Realm: compiler
aegis>
```

## Owner Key Ceremony

```bash
cd bootchain/keygen
cargo run --release
# → aegis-owner.privkey.der
# → aegis-owner.pubkey.der
```

## Project Structure

```
aegis/
├── kernel/
├── compiler-realm/
├── bootchain/
├── userspace/
│   ├── init/
│   └── shell/
├── hardware/
│   ├── qrb5165-rb6/
│   └── pixel9-tensor-g4/
├── scripts/
│   ├── build_all.sh
│   └── flash_rb6.sh
├── rust-toolchain.toml
└── README.md
```

## 60-Day Masters Roadmap

| Week | Milestone                         | Deliverable                         |
|------|-----------------------------------|--------------------------------------|
| 1    | Boot + Realm spawner              | `aegis>` shell                       |
| 2    | Compiler Realm functional         | compile `hello.rs`                   |
| 3    | Android apps in Realm             | Waydroid + CCA patches               |
| 4    | Owner-signed A/B updates          | rollback-protected updates           |
| 5–8  | Desktop mode, camera, 120 Hz UI   | iPhone-level polish                  |
| 9–12 | First 100 Alpha devices shipped   | **New standard**                     |

## Contributing (Masters Only)

1. Fork https://github.com/aegis-os/aegis  
2. **No C/C++** in kernel or privileged Realms  
3. All new code = **Rust + memory-safe**  
4. New Realms must be **sealed + attested**  
5. PR naming:

```
[realm/compiler/bootchain] Description
```

## License & Governance

- **Core OS:** GPLv3 + Apache 2.0  
- **Hardware:** CERN-OHL-S v2  
- **Governance:** Independent Aegis Foundation  

## Raise the Shield

github.com/aegis-os/aegis • #aegis-os on Matrix  
Flash it today → change everything tomorrow.
