use plotters::prelude::*;

pub fn draw_fig(x: Vec<f64>, y: Vec<f64>, upper: Vec<f64>, lower: Vec<f64>, save_path: String) -> Result<(), Box<dyn std::error::Error>> {

    let img_w = 1080;
    let img_h = 720;

    // Set canvas and fill backend by black
    let root = BitMapBackend::new(&save_path, (img_w, img_h)).into_drawing_area();
    root.fill(&WHITE);

    // Get upper and loewr bound to restrict y-axis
    let y_min = y.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let y_max = y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let y_upper_bound = y_max + (y_max - y_min) * 0.1;
    let y_lower_bound = y_min - (y_max - y_min) * 0.1;

    // Set other information
    let caption = "Prediction";
    let font = ("sans-serif", 20);
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, font.into_font())
        .margin(20)
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