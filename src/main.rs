use rand::*;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

const RADIUS: f32 = 1_000_f32;

fn main() {
    let mut args = std::env::args().skip(1);
    let iter = args.next().map(|i| i.parse::<f64>().unwrap()).unwrap();
    println!("Iterations: {}", iter);

    let mut was_in_circle: f64 = 0_f64;

    for _ in 0..(iter as u64) {
        let point = random_point();

        if is_in_circle(&point) { was_in_circle += 1_f64; }
    }

    let pi = 4_f64 * was_in_circle / iter;

    println!("PI = ~{}", pi);
}


fn random_point() -> Point {
    Point {
        x: random::<f32>() * RADIUS,
        y: random::<f32>() * RADIUS
    }
}

fn is_in_circle(point: &Point) -> bool {
    let distance = (point.x.powi(2) as f64 + point.y.powi(2) as f64).sqrt();

    distance < RADIUS as f64
}
