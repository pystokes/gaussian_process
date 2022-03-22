use std::env;

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
    lib::utils::save_ts_as_csv(&ts, col_names, "ts.csv");

    // Extract input and output of train and test
    let (inputs, outputs) = lib::utils::extract_data(ts);
    println!("inputs(head=5): {:?}", &inputs[..5]);
    println!("outputs(head=5): {:?}", &outputs[..5]);

    // Generate explanatory variable

    // Define model

    // Train

    // Predict
    
}
