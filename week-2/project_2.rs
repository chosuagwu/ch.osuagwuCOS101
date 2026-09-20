fn main() {
    let toshiba = 450_000.0;
    let mac = 1_500_000.0;
    let hp = 750_000.0;
    let dell = 2_850_000.0;
    let acer = 250_000.0;

    let sum = toshiba + mac + hp + dell + acer;
    let average = sum / 5.0;
    
    println!("Sum: {}", sum);
    println!("Average: {}", average);
}