use crate::types::{Plot, Point, PointVector};
use plotters::coord::types::{RangedCoordi32, RangedCoordi64};
use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;

pub fn plot(plot: Plot) {
    let colors = [&RED, &GREEN, &BLUE, &ORANGE, &BLACK];

    let x_first = (plot.input_sizes[0] as f64 * 0.8) as i64;
    let x_last = (plot.input_sizes[plot.input_sizes.len() - 1] as f64 * 1.2) as i64;

    let root = BitMapBackend::new(plot.path.as_str(), (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(plot.title, ("sans-serif", 26).into_font())
        .x_label_area_size(30)
        .y_label_area_size(55)
        .build_cartesian_2d((x_first..x_last).log_scale(), plot.y_range)
        .unwrap();

    chart
        .configure_mesh()
        .max_light_lines(0)
        .y_label_formatter(&|y| format!("{:.1e}", y))
        .y_desc("Average Runtime in milliseconds / input size")
        .x_label_formatter(&|x| format!("{:.0e}", x))
        .x_desc("Input Size (logarithmic scale)")
        .draw()
        .unwrap();

    for (i, experiment) in plot.experiments.iter().enumerate() {
        chart
            .draw_series(
                LineSeries::new(
                    plot.input_sizes
                        .iter()
                        .zip(experiment.run_times.clone())
                        .map(|(x, run_time)| (*x, run_time)),
                    colors[i],
                ), //.point_size(2),
            )
            .unwrap()
            .label(experiment.name.clone())
            .legend({
                let color = colors[i];
                move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color)
            });
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.75))
        .draw()
        .unwrap();

    root.present().unwrap();
}

pub fn plot_upper_hull_points(plot: Plot) {
    let colors = [&RED, &GREEN, &BLUE, &ORANGE, &BLACK];

    let x_first = (plot.input_sizes[0] as f64 * 0.8) as i64;
    let x_last = (plot.input_sizes[plot.input_sizes.len() - 1] as f64 * 1.2) as i64;

    let root = BitMapBackend::new(plot.path.as_str(), (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(plot.title, ("sans-serif", 26).into_font())
        .x_label_area_size(30)
        .y_label_area_size(55)
        .build_cartesian_2d((x_first..x_last).log_scale(), plot.y_range.log_scale())
        .unwrap();

    chart
        .configure_mesh()
        .max_light_lines(0)
        .y_desc("Points on Upper Hull (logarithmic scale)")
        .x_label_formatter(&|x| format!("{:.0e}", x))
        .x_desc("Input Size (logarithmic scale)")
        .draw()
        .unwrap();

    for (i, experiment) in plot.experiments.iter().enumerate() {
        chart
            .draw_series(
                LineSeries::new(
                    plot.input_sizes
                        .iter()
                        .zip(experiment.run_times.clone())
                        .map(|(x, run_time)| (*x, run_time)),
                    colors[i],
                ), //.point_size(2),
            )
            .unwrap()
            .label(experiment.name.clone())
            .legend({
                let color = colors[i];
                move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color)
            });
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.75))
        .draw()
        .unwrap();

    root.present().unwrap();
}

pub fn plot_upper_hull(points: PointVector) {
    let root =
        BitMapBackend::new("project_2/plots/upper_hull_us_gs.png", (480, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Uniform Square - Grahams Scan",
            ("sans-serif", 26).into_font(),
        )
        .x_label_area_size(35)
        .y_label_area_size(35)
        .build_cartesian_2d(-25i64..525i64, -25i64..525i64)
        .unwrap();

    chart
        .configure_mesh()
        .y_desc("Y")
        .x_desc("X")
        .draw()
        .unwrap();

    chart
        .draw_series(
            LineSeries::new(points.points.iter().map(|Point { x, y }| (*x, *y)), &BLACK)
                .point_size(2),
        )
        .unwrap();

    root.present().unwrap();
}
