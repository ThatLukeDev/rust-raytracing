#[allow(dead_code)]

mod vector;

use vector::Vec3;

fn main() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(2.0, 4.0, 6.0);
    let scalar_c = 4.0;

    println!("A: {}", vec_a);
    println!("B: {}", vec_b);
    println!("C: {}", scalar_c);

    println!();

    println!("A+B: {}", vec_a + vec_b);
    println!("A-B: {}", vec_a - vec_b);
    println!("A*B: {}", vec_a * vec_b);
    println!("A*C: {}", vec_a * scalar_c);
    println!("A/C: {}", vec_a / scalar_c);
}
