use std::f64::consts::PI;

struct Circle {
    radius: u32,
}

impl Circle {
    fn area(&self) -> f64 {
        2.0 * PI * self.radius as f64
    }

    fn create(radius: u32) -> Circle {
        Circle { radius }
    }
}

fn main() {
    let c = Circle::create(3);
    let a1 = Circle::area(&c);
    let a2 = c.area();
    println!("Areas. a1: {}, a2: {}", a1, a2);
}
