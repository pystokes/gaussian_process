use std::fs::File;
use std::io::prelude::*;

use rand_distr::{Uniform, Distribution};
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    month: f64,
    day: f64,
    n_day: f64,
    base: f64,
    new_year_holiday_ratio: f64,
}

pub fn procedure(csv_file: &String) -> Vec<Vec<f64>> {
    // Load base data as 2D vector
    let base = match load_base(csv_file) {
        Ok(csv) => csv,
        Err(e) => {
            panic!("There was a problem to load csv file] {:?}", e)
        },
    };

    // Generate ideal multi year data
    let ts = base_to_multi_year(base);

    // Add noise
    let ts_with_noise = add_noise(ts);

    return ts_with_noise
}

fn load_base(csv_file: &String) -> Result<Vec<Vec<f64>>, csv::Error> {
    // Show information
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

fn base_to_multi_year(base: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut ts = Vec::new();

    // Generate multi-year data
    for year in 2012..2022 {

        // Generate data for each day of each year
        for row in &base {

            // Skip 29 Feb. except for leap years
            if year % 4 != 0 && row[0] == 2. && row[1] == 29. {
                continue;
            }

            // Get objective value
            let mut obj_var = vec![(row[3] * row[4]).round()];

            // Convert to a vector
            let mut daily_data = vec![year as f64];
            daily_data.extend(row);
            daily_data.append(&mut obj_var);

            // Append to Time-Series 2D vector
            ts.push(daily_data);
        }
    }

    return ts
}

fn add_noise(ts: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut ts_with_noise = Vec::new();

    // Define normal distribution
    let prob_dist = Uniform::new(0.8, 1.2);

    // Get objective value with noise
    for row in ts {

        // Generate noise and get noisy value
        let noise = prob_dist.sample(&mut rand::thread_rng());
        
        let mut obj_with_noise = vec![(row[6] * noise).round()];

        // Convert to a vector
        let mut daily_data = row;
        daily_data.append(&mut obj_with_noise);

        // Append to Time-Series 2D vector
        ts_with_noise.push(daily_data);

    }

    return ts_with_noise
}
