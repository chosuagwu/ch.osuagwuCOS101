
use std::io;

fn main() {
    println!("=== EY Public Service Level Checker ===");

    println!("\nSelect your profession:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let choice = read_input("Enter your choice:");

    let positions = if choice == "1" {
        office_admin()
    } else if choice == "2" {
        academic()
    } else if choice == "3" {
        lawyer()
    } else if choice == "4" {
        teacher()
    } else {
        println!("Invalid profession choice.");
        return;
    };

    println!("\nAvailable positions:");

    for (level, title, _) in &positions {
        println!("{} - {}", level, title);
    }

    let title = read_input("\nEnter your current position:");
    let years_input = read_input("Enter years of programming experience:");

    let years: u32 = years_input.trim().parse().unwrap_or(0);

    if years_input.trim().parse::<u32>().is_err() {
        println!("Please enter a valid number.");
        return;
    }

    let mut current_index = 0;
    let mut found = false;

    for (index, position) in positions.iter().enumerate() {
        if position.1.to_lowercase() == title.to_lowercase() {
            current_index = index;
            found = true;
            break;}
    }

    if found {
        let recommended_index = if years <= 2 {
            0
        } else if years <= 5
         {
            1
        } else if years <= 8 
        {
            2
        } else if years <= 10 
        {
            3
        } else if years <= 13 
        {
            4
        } else 
        {
            5
        };

        let current = positions.get(current_index);
        let recommended = positions.get(recommended_index);

        if current.is_some() && recommended.is_some() {
            let current = current.unwrap();
            let recommended = recommended.unwrap();

            println!("\n=== Results ===");
            println!("Current position: {}", current.1);
            println!("Current level: {}", current.0);
            println!("Years of experience: {}", years);

            if recommended_index > current_index {
                println!("\nYou may qualify for a higher position.");
            } else if recommended_index < current_index {
                println!("\nYour current position is above the level suggested by your experience.");
            } else {
                println!("\nYour position matches your experience level.");
            }

            println!("Suggested position: {}", recommended.1);
            println!("Suggested level: {}", recommended.0);
        }
    } 
    else 
    {
        println!("Position not found. Please choose a title from the list.");
    }
}

fn read_input(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn office_admin() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("APS 1-2", "Intern", 2),
        ("APS 3-5", "Administrator", 5),
        ("APS 5-8", "Senior Administrator", 8),
        ("EL1 8-10", "Office Manager", 10),
        ("EL2 10-13", "Director", 13),
        ("SES", "CEO", 14),
    ]
}

fn academic() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("APS 1-2", "-", 2),
        ("APS 3-5", "Research Assistant", 5),
        ("APS 5-8", "PhD Candidate", 8),
        ("EL1 8-10", "Post-Doc Researcher", 10),
        ("EL2 10-13", "Senior Lecturer", 13),
        ("SES", "Dean", 14),
    ]
}

fn lawyer() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("APS 1-2", "Paralegal", 2),
        ("APS 3-5", "Junior Associate", 5),
        ("APS 5-8", "Associate", 8),
        ("EL1 8-10", "Senior Associate 1-2", 10),
        ("EL2 10-13", "Senior Associate 3-4", 13),
        ("SES", "Partner", 14),
    ]
}

fn teacher() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("APS 1-2", "Placement", 2),
        ("APS 3-5", "Classroom Teacher", 5),
        ("APS 5-8", "Senior Teacher", 8),
        ("EL1 8-10", "Leading Teacher", 10),
        ("EL2 10-13", "Deputy Principal", 13),
        ("SES", "Principal", 14),
    ]
}
