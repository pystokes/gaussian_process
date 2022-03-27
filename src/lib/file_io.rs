use std::fs::File;
use std::io::prelude::*;

use friedrich::gaussian_process::GaussianProcess;
use friedrich::kernel::Kernel;
use friedrich::prior::Prior;
use serde::Deserialize;

#[derive(Deserialize)]
struct AllRecord {
    year: f64,
    month: f64,
    day: f64,
    n_day: f64,
    base: f64,
    new_year_holiday_ratio: f64,
    base_with_holiday_ratio: f64,
    base_with_noise: f64,
}

#[derive(Deserialize)]
struct ExpRecord {
    exp_var: f64,
}

#[derive(Deserialize)]
struct ObjRecord {
    obj_var: f64,
}

pub fn save_as_single_col_csv(
    ts: &Vec<f64>,
    column_names: Vec<String>,
    save_path: &str) {

    // Generate contents to save
    let mut contents = vec![column_names.join(",") + "\n"];
    for row in ts {
        // Convert a float to a string and store it sequentially
        let row_str = row.to_string() + "\n";
        contents.push(row_str);
    }

    // Save as CSV file
    let mut f = File::create(save_path).unwrap();
    f.write_all(contents.join("").as_bytes()).unwrap();
}

pub fn save_as_multi_col_csv(
    ts: &Vec<Vec<f64>>,
    column_names: Vec<String>,
    save_path: &str) {

    // Generate contents to save
    let mut contents = vec![column_names.join(",") + "\n"];
    for row in ts {
        // Convert a vector to a string and store it sequentially
        let row_str: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        let row_str = row_str.join(",") + "\n";
        contents.push(row_str);
    }

    // Save as CSV file
    let mut f = File::create(save_path).unwrap();
    f.write_all(contents.join("").as_bytes()).unwrap();
}

pub fn load_all(csv_path: &String) -> Result<Vec<Vec<f64>>, csv::Error> {
    // Open and load CSV file
    let mut csv_rows = String::new();
    let mut f = File::open(csv_path).expect("File not found.");
    f.read_to_string(&mut csv_rows).expect("Something went wrong reading the file");

    // Get reader
    let mut reader = csv::Reader::from_reader(csv_rows.as_bytes());
    
    // Convert to 2D vector
    let mut ts = Vec::new();
    for record in reader.deserialize() {
        let record: AllRecord = record?;
        let row = vec![
            record.year,
            record.month,
            record.day,
            record.n_day,
            record.base,
            record.new_year_holiday_ratio,
            record.base_with_holiday_ratio,
            record.base_with_noise,
        ];
        ts.push(row);
    }

    // Return 2D vector
    Ok(ts)
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

pub fn save_model<T1: Kernel, T2: Prior>(model: &GaussianProcess<T1, T2>, save_path: &String) {
    println!("lib::file_io::handle_model");
    let _model = model;
    let _save_path = save_path;
    //let mut f = File::create(model_save_path).unwrap();
    //f.write_all(model).unwrap();
}
