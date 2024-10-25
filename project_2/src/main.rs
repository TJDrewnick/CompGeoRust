use crate::gift_wrapping::gift_wrapping_upper_hull;
use crate::grahams_scan::grahams_scan;
use crate::grahams_scan_parallel::grahams_scan_parallel;
use crate::input_generation::{Curve, InverseCurve, Line, UniformCircle, UniformSquare};
use crate::plotting::{plot, plot_upper_hull, plot_upper_hull_points};
use crate::types::{ConvexHullAlgorithm, Experiment, InputFunction, Plot, Point};
use std::time::Instant;

mod gift_wrapping;
mod grahams_scan;
mod grahams_scan_parallel;
mod input_generation;
mod plotting;
mod types;
mod utils;

fn main() {
    let input_sizes: Vec<i64> = (2..=8).map(|exp| 10i64.pow(exp)).collect();

    //different_inputs_runtime(input_sizes.clone());
    upper_hull_size(input_sizes.clone());
    //parallel_runtime(input_sizes);
    
    //upper_hull()
}

fn different_inputs_runtime(input_sizes: Vec<i64>) {
    // grahams scan, gift wrapping, parallel grahams scan (8 cores) on all inputs
    let input_types: Vec<(InputFunction, String)> = vec![
        (UniformSquare::get_input, "Uniform Square".to_string()),
        (UniformCircle::get_input, "Uniform Circle".to_string()),
        (Curve::get_input, "Downwards Curve".to_string()),
        (InverseCurve::get_input, "Upwards Curve".to_string()),
        (Line::get_input, "Line".to_string()),
    ];

    let grahams_all_inputs = Plot {
        title: "Grahams Scan on different Inputs".to_string(),
        path: "project_2/plots/grahams_inputs.png".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan,
        args: (Option::from(true), None),
        y_range: 5e-7..5e-2,
    };

    let gift_wrapping_all_inputs = Plot {
        title: "Gift Wrapping on different Inputs".to_string(),
        path: "project_2/plots/gift_wrapping_inputs.png".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: gift_wrapping_upper_hull,
        args: (None, None),
        y_range: 5e-7..5e-2,
    };

    let grahams_parallel_all_inputs = Plot {
        title: "8-threaded Parallel Grahams Scan on different Inputs".to_string(),
        path: "project_2/plots/grahams_parallel_inputs.png".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel,
        args: (Option::from(true), Option::from(8)),
        y_range: 5e-7..5e-2,
    };

    let mut input_plots = vec![
        grahams_all_inputs,
        gift_wrapping_all_inputs,
        grahams_parallel_all_inputs,
    ];

    for (function, name) in input_types {
        for input_plot in input_plots.iter_mut() {
            println!("{}, {}", name, input_plot.title);

            let mut experiment = Experiment {
                name: name.clone(),
                run_times: vec![],
            };

            for input_size in &input_sizes {
                // skip gift wrapping for large sizes on Curve Inputs and
                // slightly larger sizes on Uniform Square and Uniform Circle Inputs
                if input_plot.algorithm as usize == gift_wrapping_upper_hull as usize
                    && ((function as usize == Curve::get_input as usize && input_size > &100000)
                        || (function as usize == UniformSquare::get_input as usize
                            && input_size > &1000000)
                        || (function as usize == UniformCircle::get_input as usize
                            && input_size > &10000000))
                {
                    break;
                }

                let upper_hull_algorithm = input_plot.algorithm;

                // calculate the runtime 4 additional times to get the more consistent average
                let total_runtime: f64 = (0..5)
                    .map(|_| {
                        let input = function(*input_size);
                        let now = Instant::now();
                        upper_hull_algorithm(input, input_plot.args.0, input_plot.args.1);
                        now.elapsed().as_secs_f64()
                    })
                    .sum();

                let avg_runtime = total_runtime / 5.0;
                println!("{}: {}", input_size, avg_runtime);

                experiment.run_times.push(avg_runtime);
            }

            input_plot.experiments.push(experiment);
        }
    }
    // convert to plots by input generation
    let uniform_square_plot = Plot {
        title: "Performance on Uniformly Distributed Points in a Square".to_string(),
        path: "project_2/plots/uniform_square_plot.png".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[0].run_times.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[0].run_times.clone(),
            },
            Experiment {
                name: "Parallel Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[0].run_times.clone(),
            },
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
        y_range: 5e-7..5e-2,
    };
    let uniform_circle_plot = Plot {
        title: "Performance on Uniformly Distributed Points in a Circle".to_string(),
        path: "project_2/plots/uniform_circle_plot.png".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[1].run_times.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[1].run_times.clone(),
            },
            Experiment {
                name: "Parallel Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[1].run_times.clone(),
            },
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
        y_range: 5e-7..5e-2,
    };
    let curve_plot = Plot {
        title: "Performance on Points on a Downwards Curve".to_string(),
        path: "project_2/plots/curve_plot.png".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[2].run_times.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[2].run_times.clone(),
            },
            Experiment {
                name: "Parallel Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[2].run_times.clone(),
            },
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
        y_range: 5e-7..5e-2,
    };
    let upwards_curve_plot = Plot {
        title: "Performance on Points on an Upwards Curve".to_string(),
        path: "project_2/plots/upwards_curve_plot.png".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[3].run_times.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[3].run_times.clone(),
            },
            Experiment {
                name: "Parallel Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[3].run_times.clone(),
            },
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
        y_range: 5e-7..5e-2,
    };
    let line_plot = Plot {
        title: "Performance on Points on a Line".to_string(),
        path: "project_2/plots/line_plot.png".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[4].run_times.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[4].run_times.clone(),
            },
            Experiment {
                name: "Parallel Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[4].run_times.clone(),
            },
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
        y_range: 5e-7..5e-2,
    };

    // input_plots are grouped by solver
    for solver_plot in input_plots.iter() {
        plot(solver_plot.clone());
    }

    // plot grouped by input generation
    plot(uniform_square_plot);
    plot(uniform_circle_plot);
    plot(curve_plot);
    plot(upwards_curve_plot);
    plot(line_plot);
}

fn parallel_runtime(input_sizes: Vec<i64>) {
    // grahams scan vs parallel grahams scan on one input with different cores
    let threads = vec![1, 2, 4, 8, 16];
    let input_function = UniformSquare::get_input;

    let mut grahams_parallel_different_threads = Plot {
        title: "Multithreaded Parallel Grahams Scan".to_string(),
        path: "project_2/plots/grahams_parallel_threads.png".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel,
        args: (Option::from(false), Option::from(1)),
        y_range: 5e-7..5e-2,
    };

    for thread in threads {
        println!(
            "{} threads, {}",
            thread, grahams_parallel_different_threads.title
        );

        let mut experiment = Experiment {
            name: format!("{} threads", thread).to_string(),
            run_times: vec![],
        };

        for input_size in &input_sizes {
            let upper_hull_algorithm = grahams_parallel_different_threads.algorithm;

            // calculate the runtime 5 times to get the more consistent average
            let total_runtime: f64 = (0..5)
                .map(|_| {
                    let mut input = input_function(*input_size);
                    input.points.sort_by_key(|Point { x, y: _ }| *x);

                    let now = Instant::now();
                    upper_hull_algorithm(
                        input,
                        grahams_parallel_different_threads.args.0,
                        Some(thread),
                    );
                    now.elapsed().as_secs_f64()
                })
                .sum();

            let avg_runtime = total_runtime / 5.0;
            println!("{}: {}", input_size, avg_runtime);

            experiment.run_times.push(avg_runtime);
        }
        grahams_parallel_different_threads
            .experiments
            .push(experiment);
    }

    // plot parallel comparison
    plot(grahams_parallel_different_threads);
}

fn upper_hull_size(input_sizes: Vec<i64>) {
    // upper hull points given the input size
    let mut upper_hull_points = Plot {
        title: "Points on the Upper Hull".to_string(),
        path: "project_2/plots/upper_hull_points.png".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan,
        args: (Option::from(true), None),
        y_range: 3f64..4000f64,
    };

    let runs: Vec<(InputFunction, ConvexHullAlgorithm, String)> = vec![
        (
            UniformSquare::get_input,
            gift_wrapping_upper_hull,
            "Uniform Square, Gift Wrapping".to_string(),
        ),
        (
            UniformCircle::get_input,
            gift_wrapping_upper_hull,
            "Uniform Circle, Gift Wrapping".to_string(),
        ),
        (
            UniformSquare::get_input,
            grahams_scan,
            "Uniform Square, Grahams Scan".to_string(),
        ),
        (
            UniformCircle::get_input,
            grahams_scan,
            "Uniform Circle, Grahams Scan".to_string(),
        ),
    ];

    for (input_function, algorithm, name) in runs {
        println!("{}, {}", name, upper_hull_points.title);

        let mut experiment = Experiment {
            name: name.clone(),
            run_times: vec![],
        };

        for input_size in &input_sizes {
            // calculate the runtime 4 additional times to get the more consistent average
            let total_points: usize = (0..5)
                .map(|_| {
                    let input = input_function(*input_size);
                    algorithm(input, upper_hull_points.args.0, upper_hull_points.args.1)
                        .points
                        .len()
                })
                .sum();

            let avg_points = total_points / 5;
            println!("{}: {}", input_size, avg_points);

            experiment.run_times.push(avg_points as f64);
        }

        upper_hull_points.experiments.push(experiment);
    }

    // plot upper hull points by input size
    plot_upper_hull_points(upper_hull_points);
}

fn upper_hull() {
    let input = UniformSquare::get_input(10000);
    let result = grahams_scan(input, Option::from(true), None);
    plot_upper_hull(result);
}
