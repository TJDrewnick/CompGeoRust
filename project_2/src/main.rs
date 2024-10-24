use crate::gift_wrapping::gift_wrapping_upper_hull;
use crate::grahams_scan::grahams_scan;
use crate::grahams_scan_parallel::grahams_scan_parallel;
use crate::input_generation::{Curve, Line, UniformCircle, UniformSquare};
use crate::plotting::plot;
use crate::types::{Experiment, InputFunction, Plot};
use std::time::Instant;

mod gift_wrapping;
mod grahams_scan;
mod grahams_scan_parallel;
mod input_generation;
mod plotting;
mod types;
mod utils;

fn main() {
    //let args: Vec<String> = std::env::args().collect();

    // get running times for all wanted plots

    // grahams scan, gift wrapping, parallel grahams scan (8 cores) on all inputs
    // annotate with number of points on upper hull
    let input_sizes: Vec<i64> = (2..=7).map(|exp| 10i64.pow(exp)).collect();
    let input_types: Vec<(InputFunction, String)> = vec![
        (UniformSquare::get_input, "Uniform Square".to_string()),
        (UniformCircle::get_input, "Uniform Circle".to_string()),
        (Curve::get_input, "Curve".to_string()),
        (Line::get_input, "Line".to_string()),
    ];

    let grahams_all_inputs = Plot {
        title: "Grahams Scan on different Inputs".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan,
        args: (Option::from(true), None),
    };

    let gift_wrapping_all_inputs = Plot {
        title: "Gift Wrapping on different Inputs".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: gift_wrapping_upper_hull,
        args: (None, None),
    };

    let grahams_parallel_all_inputs = Plot {
        title: "Parallel Grahams Scan on different Inputs".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel,
        args: (Option::from(true), Option::from(8)),
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
                upper_hull_lengths: vec![],
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
                let mut total_runtime: f64 = (0..4).map(|_| {
                    let input = function(*input_size);
                    let now = Instant::now();
                    upper_hull_algorithm(input, input_plot.args.0, input_plot.args.1);
                    now.elapsed().as_secs_f64()
                }).sum();

                let input = function(*input_size);
                let now = Instant::now();
                let result = upper_hull_algorithm(input, input_plot.args.0, input_plot.args.1);
                total_runtime += now.elapsed().as_secs_f64();
                
                let avg_runtime = total_runtime / 5.0;
                println!("{}: {}", input_size, avg_runtime);

                experiment.run_times.push(avg_runtime);
                experiment.upper_hull_lengths.push(result.points.len());
            }

            input_plot.experiments.push(experiment);
        }
    }

    // grahams scan vs parallel grahams scan on one input with different cores
    let threads = vec![1, 2, 4, 8, 16];
    let input_function = UniformCircle::get_input;
    
    let mut grahams_parallel_different_threads = Plot {
        title: "Parallel Grahams Scan with increasing number of threads".to_string(),
        experiments: vec![],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel,
        args: (Option::from(false), Option::from(1)),
    };
    
    for thread in threads {
        let mut experiment = Experiment {
            name: format!("{} threads", thread).to_string(),
            run_times: vec![],
            upper_hull_lengths: vec![],
        };
        
        for input_size in &input_sizes {
            let upper_hull_algorithm = grahams_parallel_different_threads.algorithm;

            // calculate the runtime 4 additional times to get the more consistent average
            let mut total_runtime: f64 = (0..4).map(|_| {
                let input = input_function(*input_size);
                let now = Instant::now();
                upper_hull_algorithm(input, grahams_parallel_different_threads.args.0, Some(thread));
                now.elapsed().as_secs_f64()
            }).sum();

            let input = input_function(*input_size);
            let now = Instant::now();
            let result = upper_hull_algorithm(input, grahams_parallel_different_threads.args.0, Some(thread));
            total_runtime += now.elapsed().as_secs_f64();

            let avg_runtime = total_runtime / 5.0;
            println!("{}: {}", input_size, avg_runtime);

            experiment.run_times.push(avg_runtime);
            experiment.upper_hull_lengths.push(result.points.len());
        }
        grahams_parallel_different_threads.experiments.push(experiment);
    }
    
    
    // plot results

    // input_plots are grouped by solver
    for solver_plot in input_plots.iter() {
        plot(solver_plot.clone())
    }
    
    // plot grouped by input generation
    let uniform_square_plot = Plot {
        title: "Performance of different Algorithms on Uniformly Distributed Points in a Square".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[0].run_times.clone(),
                upper_hull_lengths: input_plots[0].experiments[0].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[0].run_times.clone(),
                upper_hull_lengths: input_plots[1].experiments[0].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[0].run_times.clone(),
                upper_hull_lengths: input_plots[2].experiments[0].upper_hull_lengths.clone(),
            }
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
    };
    let uniform_circle_plot = Plot {
        title: "Performance of different Algorithms on Uniformly Distributed Points in a Square".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[1].run_times.clone(),
                upper_hull_lengths: input_plots[0].experiments[1].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[1].run_times.clone(),
                upper_hull_lengths: input_plots[1].experiments[1].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[1].run_times.clone(),
                upper_hull_lengths: input_plots[2].experiments[1].upper_hull_lengths.clone(),
            }
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
    };
    let curve_plot = Plot {
        title: "Performance of different Algorithms on Uniformly Distributed Points in a Square".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[2].run_times.clone(),
                upper_hull_lengths: input_plots[0].experiments[2].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[2].run_times.clone(),
                upper_hull_lengths: input_plots[1].experiments[2].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[2].run_times.clone(),
                upper_hull_lengths: input_plots[2].experiments[2].upper_hull_lengths.clone(),
            }
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
    };
    let line_plot = Plot {
        title: "Performance of different Algorithms on Uniformly Distributed Points in a Square".to_string(),
        experiments: vec![
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[0].experiments[3].run_times.clone(),
                upper_hull_lengths: input_plots[0].experiments[3].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Gift Wrapping".to_string(),
                run_times: input_plots[1].experiments[3].run_times.clone(),
                upper_hull_lengths: input_plots[1].experiments[3].upper_hull_lengths.clone(),
            },
            Experiment {
                name: "Grahams Scan".to_string(),
                run_times: input_plots[2].experiments[3].run_times.clone(),
                upper_hull_lengths: input_plots[2].experiments[3].upper_hull_lengths.clone(),
            }
        ],
        input_sizes: input_sizes.clone(),
        algorithm: grahams_scan_parallel, // not needed
        args: (None, None),
    };
    
    plot(uniform_square_plot);
    plot(uniform_circle_plot);
    plot(curve_plot);
    plot(line_plot);

    // plot parallel comparison
    plot(grahams_parallel_different_threads);
}
