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

    // Load CSV
    let _result = lib::utils::load_csv(csv_file);

    // Preprocess(ad hoc)
    lib::preprocess::base_to_input();

    // Train

    // Predict

}
