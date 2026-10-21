use std::io::Write;
//Creating the bytes//
fn main() {
let lager = "Lager:\n33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n";
let stout = "Stout:\nLegend\nTurbo King\nWilliams\n\n";
let non_alcoholic = "Non-Alcoholic:\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n";


//Creating and Writing into the File//
let mut file = std::fs::File::create("Nigerian_Breweries_PLC.txt").expect("create failed");
file.write_all("Project I: Nigerian Breweries\n\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}