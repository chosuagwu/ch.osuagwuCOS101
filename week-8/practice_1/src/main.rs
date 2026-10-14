fn main() {

// Using Vec :: new()
let a : Vec<i64> = Vec::new();

// printing the size of vector
println!("\nThe length of Vec::new is: {}",a.len());

// Using macro
let v = vec!["Giannis","Steph", "Paul George","Lebron","Kobe","Michael Jordan"];

// printing the size of vector
println!("\nThe length of vec macro is: { }",v.len());
}