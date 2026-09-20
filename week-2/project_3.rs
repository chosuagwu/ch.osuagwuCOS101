fn main() {
    let p: f64 = 210_000.0;
    let r: f64 = 5.0;
    let n: f64 = 3.0;

    let a: f64 = p * (1.0 - (r / 100.0)).powf(n);

    println!("The value of the TV after 3 years is: {}", a);
}