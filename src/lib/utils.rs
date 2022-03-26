use std::fs;
use std::fs::File;
use std::io::prelude::*;

use chrono::Utc;

pub fn show_usage_and_exit(args: &str) {
    eprintln!("Usage: {} preprocess CSV_PATH", args);
    eprintln!("Example] {} preprocess sample.csv", args);
    eprintln!("Usage: {} train /PATH/TO/ts.csv", args);
    eprintln!("Example] {} train ./result/YYYYMMDD-HHMMSS/ts.csv", args);
    eprintln!("Usage: {} predict /PATH/TO/MODEL", args);
    eprintln!("Example] {} predict ./result/YYYYMMDD-HHMMSS/model", args);
    std::process::exit(1);
}

pub fn create_dir_all(dir_home: &str) -> std::io::Result<String> {
    let now = Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let dir_path = format!("{}/{}", dir_home, now);
    println!("Save directory: {}", dir_path);

    fs::create_dir_all(&dir_path)?;
    Ok(dir_path)
}

pub fn save_as_csv(
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

pub fn save_as_single_column_csv(
    ts: Vec<f64>,
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


pub fn extract_data(ts: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<f64>) {
    // Define variables to return
    let mut inputs: Vec<Vec<f64>> = Vec::new();
    let mut outputs: Vec<f64> = Vec::new();

    for row in &ts {
        // One pair of data
        let input = vec![row[3]];
        let output = row[7];

        // Append
        inputs.push(input);
        outputs.push(output);
    }

    return (inputs, outputs)
}

pub fn calc_std(variances: &Vec<f64>) -> Vec<f64> {
    let mut stds = Vec::new();

    for idx in 0..variances.len() {
        stds.push(variances[idx].sqrt())
    }

    return stds
}

pub fn calc_bounds(means: &Vec<f64>, stds: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let mut uppers = Vec::new();
    let mut lowers = Vec::new();

    for idx in 0..means.len() {
        uppers.push(means[idx] + stds[idx]);
        lowers.push(means[idx] - stds[idx]);
    }

    return (uppers, lowers)
}

pub fn save_results(
    inputs: Vec<Vec<f64>>,
    means: Vec<f64>,
    variances: Vec<f64>,
    stds: Vec<f64>,
    uppers: Vec<f64>,
    lowers: Vec<f64>,
    save_path: &str) {
    
    // Check data
    assert_eq!(means.len(), variances.len());

    let mut data = String::from("input,mean,variance,std,upper,lower\n");
    for idx in 0..means.len() {
        let row = format!(
            "{},{},{},{},{},{}\n",
            inputs[idx][0],
            means[idx],
            variances[idx],
            stds[idx],
            uppers[idx],
            lowers[idx],
        );
        data.push_str(&row);
    }

    // Save as CSV file
    let mut f = File::create(save_path).unwrap();
    f.write_all(data.as_bytes()).unwrap();
}
