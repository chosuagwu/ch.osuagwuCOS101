use std::io;

fn main() {
    println!("Welcome to the Restaurant");
    println!("\nWould you like to make an order? (y/n)");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if input.trim().to_lowercase() == "y" {
        // Display the menu
        println!("\n========== RESTAURANT MENU ==========");
        println!("Code\tItem\t\t\t\tPrice (₦)");
        println!("P\tPoundo Yam / Edinkaiko Soup\t3,200");
        println!("F\tFried Rice & Chicken\t\t3,000");
        println!("A\tAmala & Ewedu Soup\t\t2,500");
        println!("E\tEba & Egusi Soup\t\t\t2,000");
        println!("W\tWhite Rice & Stew\t\t2,500");
        println!("=====================================");
        println!("\nEnter 'Q' when you finish ordering.\n");

        let mut grand_total: f64 = 0.0;

        loop {
            // Get food type
            println!("Enter food type (P, F, A, E, W) or Q to finish:");
            let mut code = String::new();
            io::stdin().read_line(&mut code).expect("Failed to read input");
            let code = code.trim().to_uppercase();

            if code == "Q" {
                break;
            }

            // Get quantity
            println!("Enter quantity:");
            let mut qty_input = String::new();
            io::stdin().read_line(&mut qty_input).expect("Failed to read input");
            let quantity: f64 = qty_input.trim().parse().expect("Please enter a valid number");

            // Determine price based on code
            let price = if code == "P" {
                3_200.0
            } else if code == "F" {
                3_000.0
            } else if code == "A" {
                2_500.0
            } else if code == "E" {
                2_000.0
            } else if code == "W" {
                2_500.0
            } else {
                println!("Invalid food type! Please try again.\n");
                continue;
            };

            
            let item_total = price * quantity;
            grand_total += item_total;

            println!("Added: {} x ₦{:.2} = ₦{:.2}\n", quantity, price, item_total);
        }

        
        let final_amount = if grand_total > 10_000.0 {
            let discount = grand_total * 0.05;
            println!("\nYou got a 5% discount of ₦{:.2}", discount);
            grand_total - discount
        } else {
            grand_total
        };

        
        println!("\n        ORDER SUMMARY              ");
        println!("Total Cost     : ₦{:.2}", grand_total);
        println!("Amount Payable : ₦{:.2}", final_amount);
        println!("                             ");
        println!("Thank you for your order!");
    } else {
        println!("\nThanks for your time. Goodbye!");
    }
}