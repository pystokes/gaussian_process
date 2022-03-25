use std::env;

use friedrich::gaussian_process::GaussianProcess;
mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        lib::utils::show_usage_and_exit();
    }

    // Run each execution mode
    let exec_mode = &args[1];
    if exec_mode == "preprocess" {
        // Get CSV path
        let csv_file = &args[1];

        // Generate Time-Series data
        let ts = lib::generate_ts::procedure(csv_file);
        let ts_col_names = vec![
            String::from("year"),
            String::from("month"),
            String::from("day"),
            String::from("n_day"),
            String::from("base"),
            String::from("new_year_holiday_ratio"),
            String::from("base_with_holiday_ratio"),
            String::from("base_with_noise"),
        ];

        // Preprocess
        lib::preprocess::procedure();

        // Save all data before subsequent processes
        let save_dir = match lib::utils::create_dir_all("./result") {
            Ok(path) => path,
            Err(_) => panic!("Failed to make directory."),
        };
        let ts_save_path = format!("{}/{}", save_dir, "ts.csv");
        lib::utils::save_as_csv(&ts, ts_col_names, &ts_save_path);

        // Extract training input and output data
        let (train_inputs, train_outputs) = lib::utils::extract_data(ts);
        // Save training input and output data

        // Generate input data in prediction term
        const N_DAY: i32 = 366; // include leap day (2/29)
        let test_col_names = vec![String::from("var")];
        let mut test_inputs = Vec::new();
        for idx in 0..N_DAY {
            test_inputs.push(vec![(idx+1) as f64]);
        }

        // Save input data in prediction term
        let test_save_path = format!("{}/{}", save_dir, "input.csv");
        lib::utils::save_as_csv(&test_inputs, test_col_names, &test_save_path);

    } else if exec_mode == "train" {

        // Load training input and output data

        // Define model and fit
        println!("Fitting...");
        let model = GaussianProcess::default(train_inputs, train_outputs);

        // Save trained model

    } else if exec_mode == "predict" {

        // Load input data in predictin term

        // Load trained model

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
    } else {
        lib::utils::show_usage_and_exit();
    }
}
