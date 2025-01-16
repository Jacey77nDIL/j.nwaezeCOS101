struct Laptop {
    brand: String,
    unit_price: u32,
    quantity: u32,
}

impl Laptop {
    // Method to calculate the total cost for a specific laptop brand
    fn total_cost(&self, purchase_quantity: u32) -> u32 {
        purchase_quantity * self.unit_price
    }
}

fn main() {
    // Define the laptops using the Laptop struct
    let hp = Laptop {
        brand: String::from("HP"),
        unit_price: 650_000,
        quantity: 10,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        unit_price: 755_000,
        quantity: 6,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        unit_price: 550_000,
        quantity: 10,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        unit_price: 850_000,
        quantity: 4,
    };

    // Quantity to purchase from each brand
    let purchase_quantity = 3;

    // Calculate the total cost
    let total_cost = hp.total_cost(purchase_quantity)
        + ibm.total_cost(purchase_quantity)
        + toshiba.total_cost(purchase_quantity)
        + dell.total_cost(purchase_quantity);

    println!("The total cost for purchasing {} laptops from each brand is: ₦{:?}", purchase_quantity, total_cost);
}
