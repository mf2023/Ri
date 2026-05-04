fn main() {
    use oqs::kem::Kem;
    let kem = Kem::new(oqs::kem::Algorithm::Kyber512).unwrap();
    let (pk, sk) = kem.keypair();
    let (ct, ss1) = kem.encapsulate(&pk);
    let ss2 = kem.decapsulate(&ct, &sk);
    assert_eq!(ss1, ss2);
    println!("Kyber test passed!");
}
