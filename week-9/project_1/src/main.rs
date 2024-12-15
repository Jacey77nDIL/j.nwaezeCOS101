use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Nigerian_Breweries_Inventory.txt").expect("create failed");
    file.write_all("Lager      | Stout      | Non-Alcoholic\n
33 Export  | Legend     | Maltina\n
Desperados | Turbo King | Amstel Malta\n
Goldberg   | Williams   | Malta Gold\n
Gulder     |            | Fayrouz\n
Heineken   |            |        \n
Star       |            |".as_bytes()).expect("write failed");
}
