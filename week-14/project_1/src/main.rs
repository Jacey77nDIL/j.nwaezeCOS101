use std::io;
use std::io::Read;

fn administrator() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn project() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("Customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("Dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    let mut input1 = String::new();
    println!("Type a if youre an administrator\nType b if youre employee\nType c if youre a project manager\nType d if youre a customer\nType e if youre a vendor" );
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    if input1.trim().to_lowercase() == "a" {
        administrator();
    }
    else if input1.trim() == "b" {
        employee();
    }
    else if input1.trim() == "c" {
        project();
    }
    else if input1.trim() == "d" {
        customer();
    }
    else if input1.trim() == "e"  {
        vendor();
    }

}