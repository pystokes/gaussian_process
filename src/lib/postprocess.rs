use std::fs;
use std::fs::File;
use std::io::prelude::*;

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
