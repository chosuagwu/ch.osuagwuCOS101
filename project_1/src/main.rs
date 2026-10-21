use std::io::{self, Write};

fn main() {
    // Data table using a vector of tuples[cite: 1]
    // Structure: (Level, Office Admin, Academic, Lawyer, Teacher)
    let aps_table: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("APS 1-2", "Intern", "-", "Paralegal", "Placement"),
        ("APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"),
        ("EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", "CEO", "Dean", "Partner", "Principal"),
    ];

    println!("--- Public Service APS Level Checker ---\n");

    // Input collection
    let profession = get_user_input("Enter Profession (Office Admin, Academic, Lawyer, Teacher): ");
    let title = get_user_input("Enter Current Position/Title: ");
    let experience_str = get_user_input("Enter Years of Experience: ");
    let years_of_exp: u32 = experience_str.parse().unwrap_or(0);

    // Determine target row index based on years of experience
    let recommended_index = if years_of_exp <= 2 {
        0 // APS 1-2
    } else if years_of_exp <= 5 {
        1 // APS 3-5
    } else if years_of_exp <= 8 {
        2 // APS 5-8
    } else if years_of_exp <= 10 {
        3 // EL1 8-10
    } else if years_of_exp <= 13 {
        4 // EL2 10-13
    } else {
        5 // SES
    };

    // Find the row index of the user's entered job title
    let entered_index = find_title_index(&aps_table, &profession, &title);

    if entered_index.is_some() {
        let current_row_idx = entered_index.unwrap();
        let current_level = aps_table[current_row_idx].0;

        // Fetch recommended position and level based on experience
        let rec_row = aps_table[recommended_index];
        let rec_level = rec_row.0;
        let rec_title = get_profession_title(&rec_row, &profession);

        println!("\n==============================================");
        println!("Profession:          {}", profession);
        println!("Entered Position:    {} ({})", title, current_level);
        println!("Years of Experience: {} year(s)", years_of_exp);

        if recommended_index > current_row_idx {
            println!("\n[RECOMMENDATION]");
            println!("Based on your experience, you qualify for a higher role!");
            println!("Recommended Position: {}", rec_title);
            println!("Recommended Level:    {}", rec_level);
        } else if recommended_index < current_row_idx {
            println!("\n[NOTICE]");
            println!("Your entered role typically requires more experience.");
            println!("Role matching your experience: {} ({})", rec_title, rec_level);
        } else {
            println!("\n[MATCH CONFIRMED]");
            println!("Your position aligns perfectly with your years of experience!");
            println!("Assigned Level: {}", rec_level);
        }
        println!("==============================================");
    } else {
        println!("\n[Error] Position or profession not found in the APS reference table.[cite: 1]");
    }
}

/// Helper function to prompt and read user input
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

/// Helper to get job title string based on profession column index
fn get_profession_title(row: &(&str, &str, &str, &str, &str), profession: &str) -> String {
    let prof_lower = profession.to_lowercase();
    if prof_lower == "office admin" || prof_lower == "office_admin" {
        row.1.to_string()
    } else if prof_lower == "academic" {
        row.2.to_string()
    } else if prof_lower == "lawyer" {
        row.3.to_string()
    } else if prof_lower == "teacher" {
        row.4.to_string()
    } else {
        "Unknown Position".to_string()
    }
}

/// Finds the row index in the table matching the user's input position
fn find_title_index(table: &[(&str, &str, &str, &str, &str)], profession: &str, title: &str) -> Option<usize> {
    let title_lower = title.to_lowercase();

    for (index, row) in table.iter().enumerate() {
        let actual_title = get_profession_title(row, profession);
        if actual_title.to_lowercase() == title_lower {
            return Some(index);
        }
    }

    None
}