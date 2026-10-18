fn main() {
    // Vector of tuples storing candidate names and years of experience: (name, years)
    let candidates: Vec<(&str, u32)> = vec![
        ("Amina", 6),
        ("Chinedu", 11),
        ("Folake", 9),
        ("Tunde", 1),
        ("Zainab", 4),
    ];

    // Ensure the list is not empty before processing
    if candidates.is_empty() 
    {
        println!("No candidates available for the interview.");
        return;
    }

    // Keep track of the highest seen so far, starting with the first candidate
    let mut top_candidate = candidates[0];

    // Walk through the vector with a loop
    for &candidate in &candidates {
        if candidate.1 > top_candidate.1 
        {
            top_candidate = candidate;
        }
    }

    // Output the result
    println!("The candidate with the highest years of experience is {} with {} years of experience.", top_candidate.0, top_candidate.1);
}