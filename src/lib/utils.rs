use std::fs;

use chrono::Utc;

pub fn show_usage_and_exit(args: &str) {
    eprintln!("Usage: {} generate CSV_PATH", args);
    eprintln!("Example] {} generate sample.csv", args);
    eprintln!("Usage: {} preprocess CSV_PATH", args);
    eprintln!("Example] {} preprocess ./dataset/YYYYMMDD-HHMMSS/ts.csv", args);
    eprintln!("Usage: {} train RESULT_DIR", args);
    eprintln!("Example] {} train ./result/YYYYMMDD-HHMMSS", args);
    eprintln!("Usage: {} predict RESULT_DIR", args);
    eprintln!("Example] {} predict ./result/YYYYMMDD-HHMMSS", args);
    eprintln!("Usage: {} visualize RESULT_DIR", args);
    eprintln!("Example] {} visualize ./result/YYYYMMDD-HHMMSS", args);
    std::process::exit(1);
}

pub fn create_dir_all(dir_home: &str) -> std::io::Result<String> {
    let now = Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let dir_path = format!("{}/{}", dir_home, now);
    println!("Created directory: {}", dir_path);

    fs::create_dir_all(&dir_path)?;
    Ok(dir_path)
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
