use std::io;

fn main() {
   let mut a = String::new(); 
   let mut b = String::new();
   let mut c = String::new();

   println!("Enter your value for a: ");
   io::stdin().read_line(&mut a).expect("Not a valid number");
   let A:f32 = a.trim().parse().expect("Not a valid number");

   println!("Enter your value for b: ");
   io::stdin().read_line(&mut b).expect("Not a valid number");
   let B:f32 = b.trim().parse().expect("Not a valid number");

   println!("Enter your value for c: ");
   io::stdin().read_line(&mut c).expect("Not a valid number");
   let C:f32 = c.trim().parse().expect("Not a valid number");

    let determinant:f32 = B.powf(2.0)-4.0*A*C;

    if determinant > 0.0{
        println!("There are two distinct roots");
    }
    else if determinant > 0.0{
        println!("There are one real root");
    }
    else if determinant == 0.0{
        println!("There are no real roots");
    }

    let root1:f32 = (-B + determinant.sqrt())/(2.0*A);
    let root2:f32 = (-B - determinant.sqrt())/(2.0*A);

    println!("The two roots of the equation are {} and {}", root1, root2);
}
