use std::io;

// Function to read a float number from standard input
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

//Trapezium
fn calculate_trapezium_area() {
    println!("\n--- Trapezium Area Calculation ---");
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base1:");
    let base2 = read_input("Enter base2:");

    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

//Rhombus
fn calculate_rhombus_area() {
    println!("\n--- Rhombus Area Calculation ---");
    let diagonal1 = read_input("Enter diagonal1:");
    let diagonal2 = read_input("Enter diagonal2:");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {:.2}", area);
}

//Parallelogram
fn calculate_parallelogram_area() {
    println!("\n--- Parallelogram Area Calculation ---");
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");

    let area = base * altitude;
    println!("The area of the parallelogram is: {:.2}", area);
}

//Cuboid
fn calculate_cube_surface_area() {
    println!("\n--- Cuboid Surface Area Calculation ---");
    let side = read_input("Enter side length:");

    let surface_area = 6.0 * side * side;
    println!("The surface area of the cube is: {:.2}", surface_area);
}

//Cylinder
fn calculate_cylinder_volume() {
    println!("\n--- Cylinder Volume Calculation ---");
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");

    let volume = std::f64::consts::PI * radius * radius * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn main() {
    loop {
        println!("\n=================================");
        println!("     PROJECT: SHAPE CALCULATOR   ");
        println!("=================================");
        println!("1-Trapezium");
        println!("2-Rhombus");
        println!("3-Parallelogram ");
        println!("4-Cuboid");
        println!("5-Cylinder");
        println!("6-Exit");
        println!("---------------------------------");
        
        let choice = read_input("Choose an option (1-6):") as i32;

        if choice == 1 {
            calculate_trapezium_area();
        } else if choice == 2 {
            calculate_rhombus_area();
        } else if choice == 3 {
            calculate_parallelogram_area();
        } else if choice == 4 {
            calculate_cube_surface_area();
        } else if choice == 5 {
            calculate_cylinder_volume();
        } else if choice == 6 {
            println!("Exiting Shape Calculator. Goodbye!");
            break;
        } else {
            println!("Invalid choice! Please select a number between 1 and 6.");
        }
    }
}