fn main() {
    // Create a Vec of tuples where each tuple contains a developer's name and years of experience
    let developers = vec![
        ("Alice", 8),
        ("Bob", 17),
        ("Charlie", 6),
        ("David", 15),
    ];

    // Initialize variables to track the highest experience and the corresponding developer
    let mut highest_experience = 0;
    let mut top_developer = "";

    // Iterate through the Vec of tuples to find the developer with the highest experience
    for (name, experience) in developers {
        if experience > highest_experience {
            highest_experience = experience;
            top_developer = name;
        }
    }

    // Output the result
    println!("The developer with the highest years of experience is {} with {} years of experience.", top_developer, highest_experience);
}
