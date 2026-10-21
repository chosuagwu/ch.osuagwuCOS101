use std::fs;

fn main() {
let _file = std::fs::File::create("data.txt").expect("create failed");
fs::remove_file("data.txt").expect("could not remove file");
println!("file is removed");

}