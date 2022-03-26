use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct ExpRecord {
    exp_var: f64,
}

#[derive(Deserialize)]
struct ObjRecord {
    obj_var: f64,
}

pub fn load_exp(csv_path: &String) -> Result<Vec<Vec<f64>>, csv::Error> {
    // Open and load CSV file
    let mut csv_rows = String::new();
    let mut f = File::open(csv_path).expect("File not found.");
    f.read_to_string(&mut csv_rows).expect("Something went wrong reading the file");

    // Get reader
    let mut reader = csv::Reader::from_reader(csv_rows.as_bytes());
    
    // Convert to 2D vector
    let mut train_exp = Vec::new();
    for record in reader.deserialize() {
        let record: ExpRecord = record?;
        let row = vec![
            record.exp_var,
        ];
        train_exp.push(row);
    }

    // Return 2D vector
    Ok(train_exp)
}

pub fn load_obj(csv_path: &String) -> Result<Vec<f64>, csv::Error> {
    // Open and load CSV file
    let mut csv_rows = String::new();
    let mut f = File::open(csv_path).expect("File not found.");
    f.read_to_string(&mut csv_rows).expect("Something went wrong reading the file");

    // Get reader
    let mut reader = csv::Reader::from_reader(csv_rows.as_bytes());
    
    // Convert to 2D vector
    let mut train_obj = Vec::new();
    for record in reader.deserialize() {
        let record: ObjRecord = record?;
        train_obj.push(record.obj_var);
    }

    // Return 2D vector
    Ok(train_obj)
}
