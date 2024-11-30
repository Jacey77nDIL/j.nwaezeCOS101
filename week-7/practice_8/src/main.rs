fn main() {
    let city_arr:[&str;5] = ["Abuja", "Porthacourt", "Maiduguri", "Kano", "Lagos"];
    println!("array is {:?}",city_arr);
    println!("array size is :{}", city_arr.len());

    for index in 0..5 {
        println!("City index {} is located in : {}", index, city_arr[index]);
    }

    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());

    for val in arr.iter(){
        println!("value is :{}", val);
    }
}
