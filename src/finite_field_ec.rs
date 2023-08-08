pub fn quadratic_residue(a: i64, p: i64) -> bool {
    let num = a.pow(((p - 1) / 2).try_into().unwrap()) % p;
    num == 1
}

pub fn z_value(x: i64, p: i64, a: i64, b: i64) -> i64 {
    (x.pow(3) + a * x + b) % p
}

pub fn index(prime: i64, a: i64, b: i64) -> Vec<(i64, i64)> {
    let p: i64 = prime;
    let mut vec: Vec<(i64, i64)> = Vec::new();
    for n in 0..p {
        let z = z_value(n, p, a, b);
        if quadratic_residue(z, p) {
            vec.push((n, z.pow(((p + 1) / 4).try_into().unwrap()) % p));
            vec.push((n, (-1) * z.pow(((p + 1) / 4).try_into().unwrap()) % p + p));
        }
    }
    vec
}
