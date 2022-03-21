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
    // Return:
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
    for data in &ts[0..5] {
        println!("{:?}", data);
    }

    // Define model

    // Train

    // Predict
    
}
