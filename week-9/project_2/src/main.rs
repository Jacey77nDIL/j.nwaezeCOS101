use std::io::Write;
use std::fs::File;
use itertools::izip;

fn main() {
    let mut file = std::fs::File::create("PAU_SIMS.txt").unwrap();

    let student_data_tuple: (
        Vec<(&str)>,
        Vec<(&str)>,
        Vec<(&str)>,
        Vec<(&str)>,
    ) = (
        vec![
            "Student Name",
            "Oluchi Mordi",
            "Adams Aliyu",
            "Shania Bolade",
            "Adekunle Gold",
            "Blanca Edemoh",
        ],
        vec![
            "Matric Number",
            "ACC102PAU1111",
            "ECO101PAU1111",
            "CSC102PAU1111",
            "EEE101PAU1111",
            "MEE102PAU1111",
        ],
        vec![
            "Department",
            "Accounting",
            "Economics",
            "Computer",
            "Electrical",
            "Mechanical",
        ],
        vec![
            "Level",
            "100",
            "300",
            "200",
            "500",
            "100",
        ],
    );    

    let (names, matricnums, departments, levels) = student_data_tuple;
    for (name, matricnum, department, level) in izip!(names, matricnums, departments, levels) {
        writeln!(file, "{:<15}| {:<15}| {:<15}| {:<15}\n", name, matricnum, department, level).expect("Cannot write to file");
    }

    print!("Done!");
}
