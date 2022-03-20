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

    // Load base data as 2D vector
    let base = match lib::utils::load_base(csv_file) {
        Ok(csv) => csv,
        Err(e) => {
            panic!("There was a problem to load csv file] {:?}", e)
        },
    };

    // Generate sample time-series data
    let ts = lib::generate_ts::run(base);

    // Preprocess(ad hoc)
    let ts_with_noise = lib::preprocess::run(ts);
    println!("{:?}", ts_with_noise);

    // Train

    // Predict

}
