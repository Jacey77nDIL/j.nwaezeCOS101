use std::io;
use std::process;

fn main() {
    // Tuples with role, years, and APS level
    let aps_tuple_office_admin: (&str, Vec<(&str, (i32, i32), &str)>) = ("Office Administrator", vec![
        ("Intern", (0, 2), "APS 1-2"),
        ("Administrator", (3, 5), "APS 3-5"),
        ("Senior Administrator", (5, 8), "APS 5-8"),
        ("Office Manager", (8, 10), "EL1 8-10"),
        ("Director", (10, 13), "EL2 10-13"),
        ("CEO", (13, 50), "SES"),
    ]);

    let aps_tuple_academic: (&str, Vec<(&str, (i32, i32), &str)>) = ("Academic", vec![
        ("Research Assistant", (3, 5), "APS 3-5"),
        ("PhD Candidate", (5, 8), "APS 5-8"),
        ("Post-Doc Researcher", (8, 10), "EL1 8-10"),
        ("Senior Lecturer", (10, 13), "EL2 10-13"),
        ("Dean", (13, 50), "SES"),
    ]);

    let aps_tuple_lawyer: (&str, Vec<(&str, (i32, i32), &str)>) = ("Lawyer", vec![
        ("Paralegal", (0, 2), "APS 1-2"),
        ("Junior Associate", (3, 5), "APS 3-5"),
        ("Associate", (5, 8), "APS 5-8"),
        ("Senior Associate 1-2", (8, 10), "EL1 8-10"),
        ("Senior Associate 3-4", (10, 13), "EL2 10-13"),
        ("Partner", (13, 50), "SES"),
    ]);

    let aps_tuple_teacher: (&str, Vec<(&str, (i32, i32), &str)>) = ("Teacher", vec![
        ("Placement", (0, 2), "APS 1-2"),
        ("Classroom Teacher", (3, 5), "APS 3-5"),
        ("Snr Teacher", (5, 8), "APS 5-8"),
        ("Leading Teacher", (8, 10), "EL1 8-10"),
        ("Deputy Principal", (10, 13), "EL2 10-13"),
        ("Principal", (13, 50), "SES"),
    ]);

    // Get user input
    let mut job_title = String::new();
    println!("Enter your job title (e.g., 'Associate Lawyer'): ");
    io::stdin().read_line(&mut job_title).unwrap();
    let job_title = job_title.trim(); // Remove trailing newline

    let mut experience_years = String::new();
    println!("Enter your years of experience (1-50): ");
    io::stdin().read_line(&mut experience_years).unwrap();
    let experience_years_int: i32 = experience_years.trim().parse().expect("Invalid input");

    // Function to check APS level for a given category
    fn check_aps_level(job_title: &str, experience_years_int: i32, roles: Vec<(&str, (i32, i32), &str)>) {
        for (role, years, aps_level) in roles {
            if job_title.to_lowercase() == role.to_lowercase() && experience_years_int >= years.0 && experience_years_int <= years.1{
                println!("Your APS level is {}", aps_level);
                process::exit(0);
            }
        }
        println!("The job title you entered is not covered in our database or your level of experience doesn't match your role.");
    }

    // Check APS level for each category
    check_aps_level(job_title, experience_years_int, aps_tuple_academic.1.clone());
    check_aps_level(job_title, experience_years_int, aps_tuple_lawyer.1.clone());
    check_aps_level(job_title, experience_years_int, aps_tuple_office_admin.1.clone());
    check_aps_level(job_title, experience_years_int, aps_tuple_teacher.1.clone());
}