//defining the details of a laptop
struct Laptop {
    brand: String,
    quantity:u32,
    price:u32,
}

//logic to calculate the cost of buying some units of a laptop
impl Laptop {
    fn cost(&self, units: u32) -> u32 {
        self.price * units
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        quantity:10,
        price:650000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        quantity: 6,
        price:755000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        quantity:10,
        price: 550000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        quantity:4,
        price:850000,
    };

    //total cost of 3 laptops from each brand
    let total = hp.cost(3) + ibm.cost(3) + toshiba.cost(3) + dell.cost(3);

    println!("");
    println!("-------------------------------------");
    println!("The cost of 3 {} Laptops => {}", hp.brand, hp.cost(3));
    println!("");
    println!("The cost of 3 {} Laptops => {}", ibm.brand, ibm.cost(3));
    println!("");
    println!("TThe cost of 3 {} Laptops => {}", toshiba.brand, toshiba.cost(3));
    println!("");
    println!("The cost of 3 {} Laptops => {}", dell.brand, dell.cost(3));
    println!("-------------------------------------");
    println!("Your total cost => {}", total);
}