use crate::input_generation::{
    alternating, left_fits_between_last_two_elements_in_right, random_sorted_halves, shuffled,
    sorted, MergeFunction,
};
#[allow(unused_imports)]
use crate::merge::{parallel_merge, sequential_merge};
#[allow(unused_imports)]
use crate::merge_sort::{fully_parallel_merge_sort, parallel_merge_sort};
use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;
use std::time::Instant;

const TOTAL_EVALUATIONS: i32 = 100;

pub fn create_data(input_sizes: Vec<usize>, threads: Vec<usize>) -> Vec<Vec<(usize, f64)>> {
    let mut data: Vec<Vec<(usize, f64)>> = Vec::with_capacity(threads.len());
    let mut input_vectors: Vec<Vec<i64>> = Vec::with_capacity(input_sizes.len());

    // generate one input for each input size
    for size in input_sizes.iter() {
        input_vectors.push(shuffled(*size as i64));
    }
    
    println!("Input Generated");

    for t in threads.iter() {
        data.push(
            input_sizes
                .iter()
                .enumerate()
                .map(|(j, n)| {
                    let total_elapsed: f64 = (0..TOTAL_EVALUATIONS)
                        .map(|_| {
                            let mut input = input_vectors[j].clone();
                            let mut scratch = vec![0; *n];
                            let now = Instant::now();
                            // switch to parallel_merge_sort if that should be used
                            fully_parallel_merge_sort(&mut input, &mut scratch, *t);
                            now.elapsed().as_secs_f64()
                        })
                        .sum();

                    // convert to milliseconds and normalise by number of evaluations
                    (
                        *n,
                        (1000 / TOTAL_EVALUATIONS) as f64 * total_elapsed / *n as f64,
                    )
                })
                .collect::<Vec<(usize, f64)>>(),
        )
    }

    data
}

pub fn create_data_for_functions(
    input_sizes: Vec<usize>,
    input_generation_functions: Vec<MergeFunction>,
) -> Vec<Vec<(usize, f64)>> {
    let mut data: Vec<Vec<(usize, f64)>> = Vec::with_capacity(input_generation_functions.len());

    for function in input_generation_functions.iter() {
        data.push(
            input_sizes
                .iter()
                .map(|n| {
                    let total_elapsed: f64 = (0..TOTAL_EVALUATIONS)
                        .map(|_| {
                            let (left, right) = function(*n as i64);
                            let mut output = vec![0; *n];
                            let now = Instant::now();
                            // switch to parallel_merge if that should be evaluated
                            sequential_merge(&left, &right, &mut output);
                            now.elapsed().as_secs_f64()
                        })
                        .sum();
                    // convert to milliseconds and normalise by number of evaluations
                    (*n, (1000 / TOTAL_EVALUATIONS) as f64 * total_elapsed)
                })
                .collect::<Vec<(usize, f64)>>(),
        )
    }

    data
}

pub fn plot_runtime_depending_on_threads() -> Result<(), Box<dyn std::error::Error>> {
    // corresponds to x axis
    let input_sizes: Vec<usize> = (3..=7).map(|exp| 10usize.pow(exp)).collect();

    // each thread and colour is a new line in the graph
    let threads: Vec<usize> = vec![1, 2, 4, 8, 16];
    let colors = [&RED, &GREEN, &BLUE, &ORANGE, &BLACK];

    // calculate the running time for each input size and thread number
    let data = create_data(input_sizes.clone(), threads.clone());

    // plot
    let root = BitMapBackend::new("runtime_plot_parallel_sort_parallel_merge.png", (640, 480))
        .into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Runtime of Parallel Sorting using Parallel Merging",
            ("times-new-roman", 30).into_font(),
        )
        .x_label_area_size(30)
        .y_label_area_size(55)
        .build_cartesian_2d((800..10000000).log_scale(), 2e-5..4e-4)?;

    chart
        .configure_mesh()
        .max_light_lines(0)
        .y_label_formatter(&|y| format!("{:.1e}", y))
        .y_desc("Average runtime in milliseconds / input size")
        .x_label_formatter(&|x| format!("{:.0e}", x))
        .x_desc("Input size (logarithmic scale)")
        .draw()?;

    for i in 0..threads.len() {
        chart
            .draw_series(LineSeries::new(data[i].clone(), colors[i]).point_size(2))
            .unwrap()
            .label(format!("{} Threads", threads[i]))
            .legend({
                let color = colors[i];
                move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color)
            });
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.75))
        .draw()
        .unwrap();

    root.present()?;
    Ok(())
}

pub fn plot_runtime_depending_on_input_generation() -> Result<(), Box<dyn std::error::Error>> {
    // corresponds to x axis
    let input_sizes: Vec<usize> = (3..=8).map(|exp| 10usize.pow(exp)).collect();

    // each type of input generation will correspond to a line in the graph
    let input_generation_function: Vec<MergeFunction> = vec![
        alternating,
        sorted,
        random_sorted_halves,
        left_fits_between_last_two_elements_in_right,
    ];
    let function_names = ["alternating", "sorted", "random_sorted_halves", "lopsided"];
    let colors = [&RED, &GREEN, &BLUE, &ORANGE /*, &BLACK*/];

    // calculate the running time for each input size and generation method
    let data = create_data_for_functions(input_sizes, input_generation_function.clone());

    // plot
    let root =
        BitMapBackend::new("runtime_plot_sequential_merge.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Runtime of Sequential Merging depending on Input",
            ("times-new-roman", 30).into_font(),
        )
        .x_label_area_size(30)
        .y_label_area_size(55)
        .build_cartesian_2d((800..100000000).log_scale(), (3e-4..1e3).log_scale())?;

    chart
        .configure_mesh()
        .max_light_lines(0)
        .y_label_formatter(&|y| format!("{:.1e}", y))
        .y_desc("Average runtime in milliseconds (logarithmic scale)")
        .x_label_formatter(&|x| format!("{:.0e}", x))
        .x_desc("Input size (logarithmic scale)")
        .draw()?;

    for i in 0..input_generation_function.len() {
        chart
            .draw_series(LineSeries::new(data[i].clone(), colors[i]).point_size(2))
            .unwrap()
            .label(function_names[i])
            .legend({
                let color = colors[i];
                move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color)
            });
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.75))
        .draw()
        .unwrap();

    root.present()?;
    Ok(())
}
