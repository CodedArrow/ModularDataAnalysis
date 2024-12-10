use plotters::prelude::*;
use crate::Nmap::entryData;

const OUT_FILE_NAME: &str = "plotters-doc-data/frequency.png";
pub fn graph() -> Result<(), Box<dyn std::error::Error>> {
    let sd = 0.60;

    let nmap_points: entryData;

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(3)
        .caption("Distribution of Ports", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .build_cartesian_2d(-4f64..4f64, 0f64..0.1)?
        .set_secondary_coord(
            (-4f64..4f64).step(0.1).use_round().into_segmented(),
            0u32..500u32,
        );

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_label_formatter(&|y| format!("{:.0}%", *y * 100.0))
        .y_desc("Frequency")
        .draw()?;

    chart.configure_secondary_axes().y_desc("").draw()?;

    let actual = Histogram::vertical(chart.borrow_secondary())
        .style(RED.filled())
        .margin(3)
        .data(nmap_points.iter().map(|x| (*x, 1)));

    chart
        .draw_secondary_series(actual)?
        .label("Values")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], RED.filled()));

    let pdf = LineSeries::new(
        (-400..400).map(|x| x as f64 / 100.0).map(|x| {
            (
                x,
                (-x * x / 2.0 / sd / sd).exp() / (2.0 * std::f64::consts::PI * sd * sd).sqrt()
                    * 0.1,
            )
        }),
        &BLACK,
    );

    chart
        .draw_series(pdf)?
        .label("Trend")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK.filled()));

    chart.configure_series_labels().draw()?;

    Ok(())
}
#[test]
fn entry_point() {
    graph().unwrap()
}