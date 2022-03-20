pub fn run(base: Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut ts = Vec::new();

    // Generate multi-year data
    for year in 2012..2022 {

        // Generate data for each day of each year
        for row in &base {

            // Skip 29 Feb. except for leap years
            if year % 4 != 0 && row[0] == 2. && row[1] == 29. {
                continue;
            }

            // Get objective value
            let mut obj_var = vec![row[3] * row[4]];

            // Convert to a vector
            let mut daily_data = vec![year as f64];
            daily_data.extend(row);
            daily_data.append(&mut obj_var);

            // Append to Time-Series 2D vector
            ts.push(daily_data);
        }
    }

    return ts
}
