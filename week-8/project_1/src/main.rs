use std::collections::HashMap;
use std::io::{self, Write}; // For reading input from the user

fn main() {
    // Define the APS levels inside the main function
    let mut aps_levels: HashMap<&str, Vec<(&str, (u8, u8), &str)>> = HashMap::new();

    aps_levels.insert("Office Administrator", vec![
        ("Intern", (0, 2), "APS 1-2"),
        ("Administrator", (3, 5), "APS 3-5"),
        ("Senior Administrator", (5, 8), "APS 5-8"),
        ("Office Manager", (8, 10), "EL1 8-10"),
        ("Director", (10, 13), "EL2 10-13"),
        ("CEO", (13, 50), "SES"),
    ]);

    aps_levels.insert("Academic", vec![
        ("Research Assistant", (3, 5), "APS 3-5"),
        ("PhD Candidate", (5, 8), "APS 5-8"),
        ("Post-Doc Researcher", (8, 10), "EL1 8-10"),
        ("Senior Lecturer", (10, 13), "EL2 10-13"),
        ("Dean", (13, 50), "SES"),
    ]);

    aps_levels.insert("Lawyer", vec![
        ("Paralegal", (0, 2), "APS 1-2"),
        ("Junior Associate", (3, 5), "APS 3-5"),
        ("Associate", (5, 8), "APS 5-8"),
        ("Senior Associate 1-2", (8, 10), "EL1 8-10"),
        ("Senior Associate 3-4", (10, 13), "EL2 10-13"),
        ("Partner", (13, 50), "SES"),
    ]);

    aps_levels.insert("Teacher", vec![
        ("Placement", (0, 2), "APS 1-2"),
        ("Classroom Teacher", (3, 5), "APS 3-5"),
        ("Snr Teacher", (5, 8), "APS 5-8"),
        ("Leading Teacher", (8, 10), "EL1 8-10"),
        ("Deputy Principal", (10, 13), "EL2 10-13"),
        ("Principal", (13, 50), "SES"),
    ]);

    // Get user input for job title
    let mut job_title = String::new();
    println!("Enter your job title (e.g., 'Associate Lawyer'): ");
    //io::stdout().flush().unwrap();  // Ensure the prompt is displayed
    io::stdin().read_line(&mut job_title).unwrap();
    let job_title = job_title.trim(); // Remove any trailing newline

    // Search for the job title in the APS levels
    let mut found = false;
    for (category, roles) in &aps_levels {
        for (role, years, aps_level) in roles {
            if job_title.to_lowercase() == role.to_lowercase() {
                println!("Your APS level is {}", aps_level);
                println!("Experience: {}-{} years", years.0, years.1);
                found = true;
                break; // No need to continue once we find the role
            }
        }
        if found {
            break; // Stop once we've found the role in any category
        }
    }

    // If no role was found
    if !found {
        println!("Job title '{}' not found in APS levels.", job_title);
    }
}
