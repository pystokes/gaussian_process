use plotters::prelude::*;

pub fn draw_fig(x: Vec<f64>, y: Vec<f64>, upper: Vec<f64>, lower: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    println!("lib::visualize::draw_fig");

    let img_w = 1080;
    let img_h = 720;

    // Set canvas and fill backend by black
    let root = BitMapBackend::new("fig.png", (img_w, img_h)).into_drawing_area();
    root.fill(&WHITE);

    // Get upper and loewr bound to restrict y-axis
    // let y_min: f64;
    // let y_max: f64;
    // match y.iter().min() {
    //     Some(n) => y_min = *n,
    //     None => unreachable!(),
    // }
    // match y.iter().max() {
    //     Some(n) => y_max = *n,
    //     None => unreachable!(),
    // }
    let y_min = y.into_iter().min_by(|&a, &b| a.partial_cmp(&b).unwrap()).unwrap();
    let y_max = y.into_iter().max_by(|&a, &b| a.partial_cmp(&b).unwrap()).unwrap();
    let y_upper_bound = y_max + (y_max - y_min) * 1.1;
    let y_lower_bound = y_min - (y_max - y_min) * 1.1;

    // Set other information
    let caption = "Prediction";
    let font = ("sans-serif", 20);
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, font.into_font())
        .margin(10)
        .x_label_area_size(16)
        .y_label_area_size(42)
        .build_cartesian_2d(
            *x.first().unwrap()..*x.last().unwrap(),
            y_lower_bound..y_upper_bound
        )?;

    // Draw figure
    chart.configure_mesh().draw()?;
    let line_series = LineSeries::new(
        x.iter().zip(y.iter()).map(|(x, y)| (*x, *y)),
        &BLUE
    );
    chart.draw_series(line_series)?;

    Ok(())
}