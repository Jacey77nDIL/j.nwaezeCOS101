fn main() {
    //Name Vector
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    //Age Vector
    let age = vec![16,17,19,22,20,21,18,23];

    print!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len(){
        print!("{} is {} years old\n", name[i], age[i]);
    }
}
