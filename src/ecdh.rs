use num_bigint::BigInt;

use crate::ec::{EllipticCurve, Point};

pub fn diffie_hellman() {
    let secp256k1 = EllipticCurve {
        a: BigInt::parse_bytes(b"0", 16).unwrap(),
        b: BigInt::parse_bytes(b"7", 16).unwrap(),
        gen: Point::Exact(
            BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16).unwrap(),
            BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",16).unwrap(),
        ),
        n: BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16).unwrap(),
        p: BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap(),
    };
    let alice = BigInt::parse_bytes(b"f8f41dbd28979b9859f741e082542a385502f25dbf55296c3a545e3872760ab7",16).unwrap();
    let alice_pubkey = secp256k1.create_public(alice.clone());

    let bob = BigInt::parse_bytes(b"6e1d3b628ba79b9859fffffffffffffffffffeffffffff0000000000000000ff",16).unwrap();
    let bob_pubkey = secp256k1.create_public(bob.clone());

    println!(
        "alice_check{},bob_check{}",
        bob_pubkey.calc_dP(alice,secp256k1.p.clone(),secp256k1.a.clone()),
        alice_pubkey.calc_dP(bob, secp256k1.p.clone(),secp256k1.a.clone())
    )
}
