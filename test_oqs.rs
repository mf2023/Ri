#[cfg(feature = "oqs")]
fn main() {
    println!("oqs feature is enabled");
    use oqs::kem::Kem;
    let kem = Kem::new(oqs::kem::Algorithm::Kyber512).unwrap();
    let Ok((pk, sk)) = kem.keypair() else {
        eprintln!("Failed to generate keypair");
        return;
    };
    println!("Successfully generated keypair");
}

#[cfg(not(feature = "oqs"))]
fn main() {
    println!("oqs feature is NOT enabled");
}
