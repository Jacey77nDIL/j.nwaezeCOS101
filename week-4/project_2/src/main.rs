use std::io;

fn main() {
    let mut experience_level = String::new();
    let mut age = String::new();

    println!("Enter your experience level (True or False): ");
    io::stdin().read_line(&mut experience_level).expect("Not a valid string");
    let experience_level_string = experience_level.trim();
    
    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Not a valid number");
    let age_int:f32 = age.trim().parse().expect("Not a valid number");

    if experience_level_string == "True" && age_int >= 40.0{
        println!("Your incentive is N1,560,000 per month");
    }
    else if experience_level_string == "True" && age_int >= 30.0{
        println!("Your incentive is N1,480,000 per month");
    }
    else if experience_level_string == "True" && age_int < 28.0{
        println!("Your incentive is N1,300,000 per month");

    }
    else if experience_level_string == "False"{
        println!("Your incentive is N100,000 per month");
    }

}
