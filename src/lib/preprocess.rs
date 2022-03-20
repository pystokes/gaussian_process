use rand_distr::{Normal, Distribution};

pub fn run(ts: Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let ts_with_noise = add_noise(ts);

    // ADD OTHER PREPROCESS HERE

    return ts_with_noise
}

fn add_noise(ts: Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut ts_with_noise = Vec::new();

    // Define normal distribution
    let normal_dist = Normal::new(0.0, 1.0_f64.sqrt()).unwrap();

    // Get objective value with noise
    for row in ts {

        // Generate noise and get noisy value
        let noise = normal_dist.sample(&mut rand::thread_rng());
        let mut obj_with_noise = vec![row[6] + noise];

        // Convert to a vector
        let mut daily_data = row;
        daily_data.append(&mut obj_with_noise);

        // Append to Time-Series 2D vector
        ts_with_noise.push(daily_data);

    }

    return ts_with_noise
}
