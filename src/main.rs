use num_bigint::{BigInt, ToBigInt};

use crate::{
    ec::{EllipticCurve, Point},
    finite_field_ec::{index, quadratic_residue},
};

mod ec;
mod finite_field_ec;

fn main() {
    let secp256k1 = EllipticCurve {
        a: BigInt::parse_bytes(
            b"0",
            16,
        )
        .unwrap(),
        b: BigInt::parse_bytes(
            b"7",
            16,
        )
        .unwrap(),
        gen: Point::Exact(
            BigInt::parse_bytes(
                b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
                16,
            )
            .unwrap(),
            BigInt::parse_bytes(
                b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                16,
            )
            .unwrap(),
        ),
        n: BigInt::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap(),
        p: BigInt::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap(),
        d: BigInt::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364142",
            16,
        )
        .unwrap(),
    };
    let p = secp256k1.calc_dP();
    println!("{:?},{}",p,secp256k1.check_sum(p.clone()));
}
