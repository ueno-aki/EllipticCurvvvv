use num_bigint::BigInt;

use crate::{ec::{EllipticCurve, Point}, ecdh::diffie_hellman};

#[test]
fn ecdh() {
    diffie_hellman();
}

#[test]
fn secp256k1() {
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
    let d = BigInt::parse_bytes(b"bb7bff1091c3622dab0c27ad438703b56a3be7ea763c8e6ffd1ebba7edf84427",16).unwrap();
    let p = secp256k1.create_public(d);
    println!("{},{}", p, secp256k1.check_sum(p.clone()));
    println!("{}", secp256k1.gen);
}

#[test]
fn secp384r1() {
    let secp384r1 = EllipticCurve{
        a: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000fffffffc", 16).unwrap(),
        b: BigInt::parse_bytes(b"b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef", 16).unwrap(),
        gen: Point::Exact(
            BigInt::parse_bytes(b"aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7",16).unwrap(),
            BigInt::parse_bytes(b"3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f", 16).unwrap(),
        ),
        n: BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973",16).unwrap(),
        p: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff",16).unwrap(),
    };
    let d = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52974",16,).unwrap();
    let p = secp384r1.create_public(d);
    println!("{},{}", p, secp384r1.check_sum(p.clone()));
    println!("{}", secp384r1.gen);
}
