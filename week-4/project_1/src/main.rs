use std::io;

fn main() {
    let mut input_a = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input_a).unwrap();
    let a: f64 = input_a.trim().parse().unwrap();

    let mut input_b = String::new();

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input_b).unwrap();
    let b: f64 = input_b.trim().parse().unwrap();

    let mut input_c = String::new();

    println!("Enter the value of c:");
    io::stdin().read_line(&mut input_c).unwrap();
    let c: f64 = input_c.trim().parse().unwrap();

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("The two roots are: {} and {}", root1, root2);
    } 
    else if d == 0.0 {
        let root = -b / (2.0 * a);

        println!("The equation has one real root: {}", root);
    } 
    else {
        println!("The equation has no real roots.");
    }
}