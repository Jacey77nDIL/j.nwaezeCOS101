fn main () {
    let total_for_toshiba = 2 * 450_000 ;
    let total_for_mac = 1_500_000 ;
    let total_for_hp = 3 * 750_000 ; 
    let total_for_dell = 3 * 2_850_000 ;
    let total_for_acer = 250_000 ;
    let sum_of_qty = 10;

    let sum = total_for_toshiba + total_for_mac + total_for_hp + total_for_dell + total_for_acer;
    let avg = sum / sum_of_qty;

    println!("The sum of the sales is ₦{}", sum);

    println!("The average of the sales is ₦{}", avg);
}