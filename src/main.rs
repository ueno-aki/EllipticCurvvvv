fn main() {
    println!("{:?}",plus((0.0,1.0), (0.25,-1.125)));
}

type point = (f64,f64);

fn plus(p1:point,p2:point) -> point{
    let phi = (p1.1 - p2.1) / (p1.0 - p2.0);
    let psi = (p1.0 * p2.1 - p1.1 * p2.0) / (p1.0 - p2.0);

    let p3_x = phi * phi - p1.0 -p2.0;
    let p3_y = -1.0 * (phi * p3_x + psi);

    (p3_x,p3_y)
}