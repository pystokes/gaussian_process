use std::fs;
use std::fs::File;
use std::io::prelude::*;

use chrono::Utc;

pub fn show_usage_and_exit(args: &str) {
    eprintln!("Usage: {} preprocess CSV_PATH", args);
    eprintln!("Example] {} preprocess sample.csv", args);
    eprintln!("Usage: {} train RESULT_DIR", args);
    eprintln!("Example] {} train ./result/YYYYMMDD-HHMMSS", args);
    eprintln!("Usage: {} predict RESULT_DIR", args);
    eprintln!("Example] {} predict ./result/YYYYMMDD-HHMMSS", args);
    std::process::exit(1);
}

pub fn create_dir_all(dir_home: &str) -> std::io::Result<String> {
    let now = Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let dir_path = format!("{}/{}", dir_home, now);
    println!("Created directory: {}", dir_path);

    fs::create_dir_all(&dir_path)?;
    Ok(dir_path)
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

pub fn extract_data(ts: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<f64>) {
    // Define variables to return
    let mut inputs: Vec<Vec<f64>> = Vec::new();
    let mut outputs: Vec<f64> = Vec::new();

    for row in &ts {
        // One pair of data
        let input = vec![row[3]]; // n_day
        let output = row[7]; // base_with_noise

        // Append
        inputs.push(input);
        outputs.push(output);
    }

    return (inputs, outputs)
}
