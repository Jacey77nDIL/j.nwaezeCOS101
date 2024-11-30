use std::io;

fn area_of_trapezium(){
    let mut height = String::new();
    println!("What is the height of the trapezium: ");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height_fl:f32 = height.trim().parse().expect("Invalid Input");

    let mut base1 = String::new();
    println!("Value of the 1st base: ");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1_fl:f32 = base1.trim().parse().expect("Invalid Input");

    let mut base2 = String::new();
    println!("Value of the 2nd base: ");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2_fl:f32 = base2.trim().parse().expect("Invalid Input");

    let trapezium_area = height_fl / (2.0*(base1_fl+base2_fl));
    println!("The area of the trapezium is {}", trapezium_area);
}

fn area_of_rhombus(){
    let mut diagonal1 = String::new();
    println!("Value of the 1st diagonal: ");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1_fl:f32 = diagonal1.trim().parse().expect("Invalid Input");

    let mut diagonal2 = String::new();
    println!("Value of the 2nd diagonal: ");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let diagonal2_fl:f32 = diagonal2.trim().parse().expect("Invalid Input");

    let rhombus_area = 0.5 * diagonal1_fl * diagonal2_fl;
    println!("The area of the rhombus is {}", rhombus_area);
}

fn area_of_parallelogram(){
    let mut base = String::new();
    println!("Value of the base: ");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base_fl:f32 = base.trim().parse().expect("Invalid Input");

    let mut altitude = String::new();
    println!("Value of the altitude: ");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude_fl:f32 = altitude.trim().parse().expect("Invalid Input");

    let parallelogram_area = base_fl * altitude_fl;
    println!("The area of the parallelogram is {}", parallelogram_area);
}

fn area_of_cube(){
    let mut length_of_side = String::new();
    println!("Length of the Side: ");
    io::stdin().read_line(&mut length_of_side).expect("Failed to read input");
    let length_of_side_fl:f32 = length_of_side.trim().parse().expect("Invalid Input");

    let cube_area = 6.0 * length_of_side_fl.powf(2.0);
    println!("The area of the cube is {}", cube_area);
}

fn volume_of_cylinder(){
    let mut radius = String::new();
    println!("Value of the radius: ");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius_fl:f32 = radius.trim().parse().expect("Invalid Input");

    let mut height = String::new();
    println!("Value of the height: ");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height_fl:f32 = height.trim().parse().expect("Invalid Input");

    let cylinder_volume = 3.142 * radius_fl.powf(2.0) * height_fl;
    println!("The volume of the Cylinder is {}", cylinder_volume);
}


fn main() {
    println!("This calculator performs the following functions \n \nT - Area of a Trapezium \nR - Area of a Rhombus \nP - Area of a Parallelogram \nC - Area of a cube \nVC - Volume of Cylinder \n\nThank you");

    let mut equation = String::new();
    println!("Which of our functions would you like to use?  ");
    io::stdin().read_line(&mut equation).expect("Failed to read input");
    let equation_string = equation.trim();

    if equation_string == "T" || equation_string == "t"{
        area_of_trapezium();
    } else if equation_string == "R" || equation_string == "r"{
        area_of_rhombus();
    } else if equation_string == "P" || equation_string == "P"{
        area_of_parallelogram();
    } else if equation_string == "C" || equation_string == "c"{
        area_of_cube();
    } else if equation_string == "VC" || equation_string == "vc"{
        volume_of_cylinder();
    }
}
