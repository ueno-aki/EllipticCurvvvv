use num_bigint::{BigInt, ToBigInt};

pub struct EllipticCurve {
    pub a: BigInt,
    pub b: BigInt,
    pub gen: Point,
    pub n: BigInt,
    pub p: BigInt,
    pub d: BigInt,
}
impl EllipticCurve {
    pub fn calc_dP(&self) -> Point {
        let mut gen = Point::Infinity;
        let bits = self.d.to_str_radix(2);
        for b in bits.split("").into_iter() {
            match b {
                "" => continue,
                "1" => {
                    gen = gen.double(self.p.clone(), self.a.clone());
                    gen = gen.plus(&self.gen, self.p.clone(),self.a.clone());
                }
                "0" => {
                    gen = gen.double(self.p.clone(), self.a.clone());
                }
                _ => panic!(""),
            }
        }
        gen
    }
    pub fn check_sum(&self,point:Point) -> bool{
        match point {
            Point::Exact(x, y) => {
                let r = (x.pow(3) + (&self.a * &x) + &self.b) % &self.p;
                let l = (y.pow(2)) % &self.p;
                r == l
            },
            _ => true
        }
    }
}

#[derive(Debug, Clone)]
pub enum Point {
    Exact(BigInt, BigInt),
    Infinity,
}
impl Point {
    pub fn plus(&self, other: &Self, prime: BigInt, a: BigInt) -> Self {
        match (self, other) {
            (Point::Exact(sx, sy), Point::Exact(ox, oy)) => {
                if sx == ox && sy != oy{
                    Point::Infinity
                } else if sx == ox {
                    self.double(prime, a)
                }else {
                    let phi = Point::div_modulo(abs_modulo(sy - oy, &prime), abs_modulo(sx - ox, &prime), prime.clone());
                    let x = abs_modulo(phi.pow(2) - sx - ox, &prime);
                    let y = abs_modulo(phi * (sx - &x) - sy, &prime);
                    Point::Exact(x, y)
                }
            }
            (Point::Exact(x, y), Point::Infinity) => Point::Exact(x.clone(), y.clone()),
            (Point::Infinity, Point::Exact(x, y)) => Point::Exact(x.clone(), y.clone()),
            (Point::Infinity, Point::Infinity) => Point::Infinity,
        }
    }
    pub fn double(&self, prime: BigInt, a: BigInt) -> Self {
        match self {
            Point::Exact(sx, sy) => {
                let phi = Point::div_modulo(abs_modulo(3 * sx.pow(2) + a, &prime), abs_modulo(2 * sy, &prime), prime.clone());
                let x = abs_modulo(phi.pow(2) - sx - sx, &prime);
                let y = abs_modulo(phi * (sx - &x) - sy, &prime);
                Point::Exact(x, y)
            }
            Point::Infinity => Point::Infinity,
        }
    }
    pub fn div_modulo(num: BigInt, denom: BigInt, prime: BigInt) -> BigInt {
        // p = q * denom + a
        let mut p = prime.clone();
        let mut denom = denom.clone();
        let mut q = &p / &denom;
        let mut a = &p - (&q * &denom);

        let mut k = BigInt::from(1);
        let mut x = BigInt::from(0);
        let mut u = BigInt::from(0);
        let mut v = BigInt::from(1);

        while a != BigInt::from(0) {
            let k_cln = k.clone();
            let x_cln = x.clone();
            k = u.clone();
            x = v.clone();
            u = k_cln - (&q * u);
            v = x_cln - (&q * v);
            p = denom.clone();
            denom = a.clone();
            q = &p / &denom;
            a = &p - (&q * &denom);
        }
        x = v.clone();
        (num * x) % prime
    }
}

fn abs_modulo(num: BigInt, prime: &BigInt) -> BigInt{
    if &num % prime < BigInt::from(0) {
        (num % prime) + prime
    }else {
        num % prime
    }
}