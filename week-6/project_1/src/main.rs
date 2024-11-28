use std::io;

fn main() {
    println!("Our Menu consists of \n \nP = Poundo Yam/Edinkaiko soup for N3200 \nF = Fried Rice & Chicken for N3000 \nA = Amala & Ewedu Soup for N2000 \nE = Eba & Egusi Soup for N2000 \nW = White Rice & Stew for N2500");

    //How much food
    println!("\nHow many meal options will you be ordering? ");
    let mut foods = String::new();
    io::stdin()
        .read_line(&mut foods)
        .expect("Failed to read input");
    let foods:i32 = foods.trim().parse().expect("Input not an integer");

    //Prices

    let p_price:f32 = 3200.00;
    let f_price:f32 = 3000.00;
    let a_price:f32 = 2500.00;
    let e_price:f32 = 2000.00;
    let w_price:f32 = 2500.00;

    let mut total_cost:f32 = 0.0;


    for food in 0..foods {
        //Input Order
        println!("\nPlease Enter your order. (P or F or A or E or W)");
        let mut order = String::new();
        io::stdin()
            .read_line(&mut order)
            .expect("Failed to read input");
        let order_string = order.trim();

        //Input Quantity
        println!("\nQuantity? ");
        let mut quantity = String::new();
        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read input");
        let quantity:f32 = quantity.trim().parse().expect("Input not an integer");


        if order_string == "P" || order == "p"{
            total_cost += p_price * quantity;
        }
        if order_string == "F" || order == "f"{
            total_cost += f_price * quantity;
        }
        if order_string == "A" || order == "a"{
            total_cost += a_price * quantity;
        }
        if order_string == "E" || order == "e"{
            total_cost += e_price * quantity;
        }
        if order_string == "W" || order == "w"{
            total_cost += w_price * quantity;
        }

    }

    println!("Your total cost is {}", total_cost);

    if total_cost >= 10000.0{
        let discounted_price:f32 = total_cost * 0.95;
        println!("You qualify for our discount so your new price is {}", discounted_price);
    }
    
    

}
