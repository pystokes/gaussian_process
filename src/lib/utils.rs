use std::fs::File;
use std::io::prelude::*;

pub fn save_ts_as_csv(
    ts: &Vec<Vec<f64>>,
    column_names: Vec<String>,
    save_path: &str) {

    // Generate contents to save
    let mut contents = vec![column_names.join(",") + "\n"];
    for row in ts {
        // Convert a vector to a string  and store it sequentially
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
        let input = vec![row[3]];
        let output = row[7];

        // Append
        inputs.push(input);
        outputs.push(output);
    }

    return (inputs, outputs)
}
