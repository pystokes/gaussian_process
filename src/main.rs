use std::env;

use friedrich::gaussian_process::GaussianProcess;
mod lib;

fn main() {

    // Get CSV path
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} CSV_PATH", args[0]);
        eprintln!("Example] {} sample.csv", args[0]);
        std::process::exit(1);
    }
    let csv_file = &args[1];

    // Generate Time-Series data
    // Return: (Note that this may be updated)
    //   - year
    //   - month
    //   - day
    //   - n_day
    //   - base
    //   - new_year_holiday_ratio
    //   - baseh_with_holiday_weighted
    //   - base_with_noise
    let ts = lib::generate_ts::procedure(csv_file);

    // Preprocess
    lib::preprocess::procedure();

    // Save data before subsequent processes
    // (Note that this may be updated as above)
    let save_dir = match lib::utils::create_dir_all("./result") {
        Ok(path) => path,
        Err(_) => panic!("Failed to make directory."),
    };
    let ts_save_path = format!("{}/{}", save_dir, "ts.csv");
    let col_names = vec![
        String::from("year"),
        String::from("month"),
        String::from("day"),
        String::from("n_day"),
        String::from("base"),
        String::from("new_year_holiday_ratio"),
        String::from("base_with_holiday_ratio"),
        String::from("base_with_noise"),
    ];
    lib::utils::save_ts_as_csv(&ts, col_names, &ts_save_path);

    // Extract input and output of train and test
    let (train_inputs, train_outputs) = lib::utils::extract_data(ts);

    // Generate explanatory variable
    const N_DAY: i32 = 366; // include leap day (2/29)
    let mut test_inputs = Vec::new();
    for idx in 0..N_DAY {
        test_inputs.push(vec![(idx+1) as f64]);
    }

    // Define model and fit
    println!("Fitting...");
    let model = GaussianProcess::default(train_inputs, train_outputs);

    // Predict
    println!("Predicting means...");
    let means = model.predict(&test_inputs);
    println!("Predicting variances...");
    let variances = model.predict_variance(&test_inputs);

    // Calculate additional information
    let stds = lib::utils::calc_std(&variances);
    let (uppers, lowers) = lib::utils::calc_bounds(&means, &stds);

    // Save results
    let result_save_path = format!("{}/{}", save_dir, "result.csv");
    lib::utils::save_results(
        test_inputs,
        means,
        variances,
        stds,
        uppers,
        lowers,
        &result_save_path
    );
}
