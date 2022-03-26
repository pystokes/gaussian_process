use std::env;
use std::fs::File;
use std::io::prelude::*;

use friedrich::gaussian_process::GaussianProcess;
mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        lib::utils::show_usage_and_exit(&args[0]);
    }

    // Set execution mode
    let exec_mode = &(args[1]);
    println!("Execution mode: {}", exec_mode);

    // Set save directory
    let save_dir = match exec_mode.as_ref() {
        "preprocess" => {
            match lib::utils::create_dir_all("./result") {
                Ok(path) => path,
                Err(_) => panic!("Failed to make directory."),
            }
        },
        _ => {
            let path: Vec<&str> = args[2].split('/').collect();
            let path = &path[..(path.len()-1)];
            let path = path.join("/");    

            path
        }
    };
    println!("Save directory: {}", save_dir);

    // Run each execution mode
    if exec_mode == "preprocess" {
        // Get CSV path
        let csv_path = &args[2];

        // Generate Time-Series data
        let ts = lib::generate_ts::procedure(csv_path);
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
        let ts_save_path = format!("{}/{}", save_dir, "ts.csv");
        lib::utils::save_as_csv(&ts, ts_col_names, &ts_save_path);

        // Extract training exp and obj data
        let (train_exp, train_obj) = lib::utils::extract_data(ts);
        // Save training input and output data
        let train_input_save_path = format!("{}/{}", save_dir, "train_exp.csv");
        lib::utils::save_as_csv(&train_exp, vec![String::from("exp_var")], &train_input_save_path);
        let train_output_save_path = format!("{}/{}", save_dir, "train_obj.csv");
        lib::utils::save_as_single_column_csv(train_obj, vec![String::from("obj_var")], &train_output_save_path);

        // Generate input data in prediction term
        const N_DAY: i32 = 366; // include leap day (2/29)
        let test_col_names = vec![String::from("epx_var")];
        let mut test_exp = Vec::new();
        for idx in 0..N_DAY {
            test_exp.push(vec![(idx+1) as f64]);
        }

        // Save input data in test term
        let test_save_path = format!("{}/{}", save_dir, "test_exp.csv");
        lib::utils::save_as_csv(&test_exp, test_col_names, &test_save_path);

    } else if exec_mode == "train" {

        // Load training input and output data


        // Define model and fit
        println!("Fitting...");
        //let model = GaussianProcess::default(train_exp, train_obj);

        // Save trained model
        //let model_save_path = format!("{}/{}", save_dir, "model");
        //let mut f = File::create(model_save_path).unwrap();
        //f.write_all(model).unwrap();

        // Predict
        println!("Predicting means...");
        //let means = model.predict(&train_exp);
        println!("Predicting variances...");
        //let variances = model.predict_variance(&train_exp);

        // Calculate additional information
        //let stds = lib::utils::calc_std(&variances);
        //let (uppers, lowers) = lib::utils::calc_bounds(&means, &stds);

        // Save results
        //let result_save_path = format!("{}/{}", save_dir, "result.csv");
        //lib::utils::save_results(
        //    train_exp,
        //    means,
        //    variances,
        //    stds,
        //    uppers,
        //    lowers,
        //    &result_save_path
        //);
    } else {
        lib::utils::show_usage_and_exit(&args[0]);
    }
}
