use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    month: f64,
    day: f64,
    n_day: f64,
    base: f64,
    new_year_holiday_ratio: f64,
}

pub fn load_base(csv_file: &String) -> Result<Vec<Vec<f64>>, csv::Error> {

    // Show information for debug
    println!("CSV file: {}", csv_file);

    // Open and load CSV file
    let mut csv_rows = String::new();
    let mut f = File::open(csv_file).expect("File not found.");
    f.read_to_string(&mut csv_rows).expect("Something went wrong reading the file");

    // Get reader
    let mut reader = csv::Reader::from_reader(csv_rows.as_bytes());
    
    // Convert to 2D vector
    let mut base = Vec::new();
    for record in reader.deserialize() {
        let record: Record = record?;
        let row = vec![
            record.month,
            record.day,
            record.n_day,
            record.base,
            record.new_year_holiday_ratio
        ];
        base.push(row);
    }

    // Return 2D vector
    Ok(base)
}
