use std::io;

fn main() {
    println!("=== Annual Incentive Calculator ===");

    println!("Please enter your years of work experience(3 yrs>=):");

    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).unwrap();
    let experience: i32 = experience_input.trim().parse().unwrap();

    if experience >= 3 {
        println!("Please enter your age:");

        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).unwrap();
        let age: i32 = age_input.trim().parse().unwrap();

        if age < 18 {
            println!("Too young for this position.");
        }
        else if age >= 40 {
            println!("Your annual incentive is ₦1,560,000.");
        }
        else if age >= 30 {
            println!("Your annual incentive is ₦1,480,000.");
        }
        else {
            println!("Your annual incentive is ₦1,300,000.");
        }
    }
    else {
        println!("Your annual incentive is ₦100,000.");
    }
}