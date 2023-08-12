#![feature(test)]

use elliptic_curve::ecdh::diffie_hellman;
use test::bench;

extern crate test;

#[bench]
fn ecdh(b:&mut test::Bencher) {
    b.iter(diffie_hellman)
}