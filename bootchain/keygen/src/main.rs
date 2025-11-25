use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;

fn main() {
    println!("AEGIS OWNER KEY CEREMONY – AIRGAP THIS MACHINE");
    let keypair = Keypair::generate(&mut OsRng);
    std::fs::write("aegis-owner.privkey.der", keypair.to_bytes()).unwrap();
    std::fs::write("aegis-owner.pubkey.der", keypair.public.to_bytes()).unwrap();
    println!("Keys generated – private key NEVER leaves this machine");
}
