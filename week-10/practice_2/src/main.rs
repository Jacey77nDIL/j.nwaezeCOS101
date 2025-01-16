fn main() {
    let v = vec![10, 20, 30];
    //vector v owns the obeject in heap

    let v2 = v;

    display(v2);
    //v2 is moved to display and v2 is invalidated

    println!("In main {:?}", v2);
    //v2 is no longer usable here
}

fn display(v: Vec<i32>) {
    println!("inside display {:?}", v);
}