use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use friedrich::gaussian_process::GaussianProcess;
use serde::{Serialize, Deserialize};

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
        "generate" => {
            match lib::utils::create_dir_all("./dataset") {
                Ok(path) => path,
                Err(_) => panic!("Failed to make directory."),
            }
        },
        "preprocess" => {
            match lib::utils::create_dir_all("./result") {
                Ok(path) => path,
                Err(_) => panic!("Failed to make directory."),
            }
        },
        _ => {
            let path = args[2].to_string();
            println!("Save directory: {}", path);
            path
        },
    };
    

    // Run each execution mode
    if exec_mode == "generate" {
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

        // Save
        let ts_save_path = format!("{}/{}", save_dir, "ts.csv");
        lib::file_io::save_as_multi_col_csv(&ts, ts_col_names, &ts_save_path);

    } else if exec_mode == "preprocess" {
        // Get CSV path
        let csv_path = &args[2];

        // Load data
        let ts = match lib::file_io::load_all(csv_path) {
            Ok(csv) => csv,
            Err(e) => {
                panic!("There was a problem to load csv file] {:?}", e)
            },
        };

        // Preprocess
        let prerocessed_ts = lib::preprocess::procedure(&ts);

        // Extract training exp and obj data
        let (train_exp, train_obj) = lib::utils::extract_data(prerocessed_ts);

        // Save training input and output data
        let train_input_save_path = format!("{}/{}", save_dir, "train_exp.csv");
        lib::file_io::save_as_multi_col_csv(&train_exp, vec![String::from("exp_var")], &train_input_save_path);
        let train_output_save_path = format!("{}/{}", save_dir, "train_obj.csv");
        lib::file_io::save_as_single_col_csv(&train_obj, vec![String::from("obj_var")], &train_output_save_path);

        // Generate input data in prediction term
        const N_DAY: i32 = 366; // include leap day (2/29)
        let test_col_names = vec![String::from("exp_var")];
        let mut test_exp = Vec::new();
        for idx in 0..N_DAY {
            test_exp.push(vec![(idx+1) as f64]);
        }

        // Save input data in test term
        let test_save_path = format!("{}/{}", save_dir, "test_exp.csv");
        lib::file_io::save_as_multi_col_csv(&test_exp, test_col_names, &test_save_path);

    } else if exec_mode == "train" {

        // Load explanatory variables
        let train_exp_path = format!("{}/{}", save_dir, "train_exp.csv");
        let train_exp = match lib::file_io::load_exp(&train_exp_path) {
            Ok(csv) => csv,
            Err(e) => {
                panic!("There was a problem to load csv file] {:?}", e)
            },
        };
        // Load objective variables
        let train_obj_path = format!("{}/{}", save_dir, "train_obj.csv");
        let train_obj = match lib::file_io::load_obj(&train_obj_path) {
            Ok(csv) => csv,
            Err(e) => {
                panic!("There was a problem to load csv file] {:?}", e)
            },
        };

        // Define model and fit
        println!("Fitting...");
        let model = GaussianProcess::default(train_exp, train_obj);

        // Save trained model (kernel)
        let kernel_save_path = format!("{}/{}", save_dir, "model_kernel.json");
        let kernel_serialized = serde_json::to_string_pretty(&model.kernel).unwrap();
        let mut f = File::create(kernel_save_path).unwrap();
        f.write_all(kernel_serialized.as_bytes()).unwrap();
        // Save trained model (prior)
        let prior_save_path = format!("{}/{}", save_dir, "model_prior.json");
        let prior_serialized = serde_json::to_string_pretty(&model.prior).unwrap();
        let mut f = File::create(prior_save_path).unwrap();
        f.write_all(prior_serialized.as_bytes()).unwrap();
        // Save trained model (noise)
        let noise_save_path = format!("{}/{}", save_dir, "model_noise.json");
        let noise_serialized = serde_json::to_string_pretty(&model.noise).unwrap();
        let mut f = File::create(noise_save_path).unwrap();
        f.write_all(noise_serialized.as_bytes()).unwrap();

    } else if exec_mode == "predict" {

        // Load model
        let kernel_save_path = format!("{}/{}", save_dir, "model_kernel.json");
        let prior_save_path = format!("{}/{}", save_dir, "model_prior.json");
        let noise_save_path = format!("{}/{}", save_dir, "model_noise.json");
        let kernel_json = fs::read_to_string(kernel_save_path).expect("Faild to load model_kernel.json");
        let prior_json = fs::read_to_string(prior_save_path).expect("Faild to load model_prior.json");
        let noise_json = fs::read_to_string(noise_save_path).expect("Faild to load noise_prior.json");

        let mut model = GaussianProcess::default(vec![vec![1.], vec![2.]], vec![1., 2.]);
        model.kernel = serde_json::from_str(kernel_json.as_ref()).unwrap();
        model.prior = serde_json::from_str(prior_json.as_ref()).unwrap();
        model.noise = serde_json::from_str(noise_json.as_ref()).unwrap();

        // Load test data
        let test_exp_path = format!("{}/{}", save_dir, "test_exp.csv");
        let test_exp = match lib::file_io::load_exp(&test_exp_path) {
            Ok(csv) => csv,
            Err(e) => {
                panic!("There was a problem to load csv file] {:?}", e)
            },
        };

        // Predict
        println!("Predicting means...");
        let means = model.predict(&test_exp);
        println!("Predicting variances...");
        let variances = model.predict_variance(&test_exp);

        // Calculate additional information
        let stds = lib::postprocess::calc_std(&variances);
        let (uppers, lowers) = lib::postprocess::calc_bounds(&means, &stds);

        // Save results
        let result_save_path = format!("{}/{}", save_dir, "result.csv");
        lib::postprocess::save_results(
          test_exp,
          means,
          variances,
          stds,
          uppers,
          lowers,
          &result_save_path
        );

    } else if exec_mode == "visualize" {
        // Load result.csv
        let result_path = format!("{}/{}", save_dir, "result.csv");
        let result = match lib::file_io::load_result(&result_path) {
            Ok(csv) => csv,
            Err(e) => {
                panic!("There was a problem to load csv file] {:?}", e)
            },
        };

        // Extract data to draw figure
        let mut x = Vec::new();
        let mut mean = Vec::new();
        let mut upper = Vec::new();
        let mut lower = Vec::new();
        for row in result {
            x.push(row[0]);
            mean.push(row[1]);
            upper.push(row[4]);
            lower.push(row[5]);
        }

        let fig_save_path = format!("{}/{}", save_dir, "fig.png");
        lib::visualize::draw_fig(x, mean, upper, lower, fig_save_path);

    } else {
        lib::utils::show_usage_and_exit(&args[0]);
    }
}
