use std::fs::File;
use std::io::Write;
use itertools::izip;

fn main() {
    // Separate datasets
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("output.txt").expect("Unable to create file");

    //Header
    writeln!(
        file,
        "{:<5} | {:<25} | {:<17} | {:<12}",
        "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE"
    )
    .expect("Unable to write to file");

    //use of izip because it's a dependency that makes iteration easier
    for (i, (commissioner, ministry, zone)) in
        izip!(commissioners, ministries, geopolitical_zones).enumerate()
    {
        writeln!(
            file,
            "{:<5} | {:<25} | {:<17} | {:<12}",
            i + 1,
            commissioner,
            ministry,
            zone
        )
        .expect("Unable to write to file");
    }

    println!("Data has been written to output.txt");
}
