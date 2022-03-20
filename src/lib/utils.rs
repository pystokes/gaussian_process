use std::fs::File;
use std::io::prelude::*;

use csv::Error;

pub fn load_csv(csv_file: &String) -> Result<(), Error> {
    println!("Load CSV");
    println!("CSV file: {}", csv_file);

    // Open CSV file
    let mut csv_rows = String::new();
    let mut f = File::open(csv_file).expect("File not found.");
    f.read_to_string(&mut csv_rows).expect("Something went wrong reading the file");

    let mut reader = csv::Reader::from_reader(csv_rows.as_bytes());
    
    for record in reader.records() {
        let record = record?;
        println!("{} {} {}", &record[0], &record[1], &record[2]);
    }

    Ok(())
}
