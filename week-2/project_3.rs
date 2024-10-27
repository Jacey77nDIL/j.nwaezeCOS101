fn main() {
    let p:f64 = 510_000.00;
    let r:f64 = 5.0;
    let t:f64 = 3.0;

    //simple interest
    let a = p * (1.0 + (r /100.0)).powf(t);
    let d = a - p;
    println!("Depreciation is {}", d);
    let v = p - d;
    println!("Value of TV after 3 years {}", v);
}