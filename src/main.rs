use std::{collections::HashMap, path::Path};

use clap::{Parser, Subcommand};
use util::{log_analyzer::LogAnalyzer, log_data::LogData, models::*, parse_log, read_directory};

mod plot;
mod table;
mod util;

/// Handles Commands for specifying graphs to be generated
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
    time: String,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    By { x_axis: String, y_axis: String },
    Cumulative { x_axis: String, y_axis: String },
    Ratio { x_axis: String, y_axis: String },
    CumulativeRatio { y_axis: String },
}

fn main() {
    let args = Cli::parse();
    let time = args.time.trim();
    let log_file = Path::new("logs/");
    let log_contents = read_directory(log_file);
    let time_multiplier: HashMap<String, i64> = HashMap::from([
        ("year".to_string(), 31556952),
        ("month".to_string(), 2629800),
        ("day".to_string(), 86400),
        ("hour".to_string(), 3600),
        ("min".to_string(), 60),
        ("sec".to_string(), 1),
    ]);
    // Puts all logs in a vector tracking each part and removing redundant logs.
    let log_data = parse_log(log_contents);
    let log_data = LogAnalyzer {
        logs: log_data,
        time_multi: *time_multiplier.get(time).unwrap(),
    };
    let (mut log_data_by_time, total_log_data) = log_data.get_data(time);
    log_data_by_time.sort_by(|a, b| (a.time as i64).cmp(&(b.time as i64).clone()));
    println!("{}", total_log_data);
    let mut data_point: Vec<(f64, f64)> = Vec::new();
    //println!("{}", time);
    match args.cmd {
        Commands::By { mut x_axis, y_axis } => {
            let length = &log_data_by_time.len();
            let model = Models {
                model: MODELTYPES::SCWIND,
                total_errors: total_log_data.errors as f64,
                log_data: log_data_by_time.clone(),
                data_point: *length,
            };
            let (_, second_prime) = model.get_curve();
            // Move to different file later
            match x_axis.trim() {
                "time" => {
                    let mut index = 0.0;
                    for data in log_data_by_time {
                        //println!("{:#?}\n", data);
                        let (_, y) = data.get_data_point(&x_axis, &y_axis);

                        //println!("{:?}", (x, y));
                        data_point.push((index, y));
                        index += 1.0
                    }

                    x_axis = time.to_string();
                }
                _ => {
                    for data in log_data_by_time {
                        //println!("{:#?}\n", data);
                        let (x, y) = data.get_data_point(&x_axis, &y_axis);

                        //println!("{:?}", (x, y));
                        data_point.push((x, y));
                    }
                }
            }
            plot::plot_double_graph(
                data_point[0].0,
                data_point.last().unwrap().0,
                y_axis.to_owned(),
                x_axis.to_owned(),
                data_point,
                second_prime,
            );
        }

        Commands::Cumulative { mut x_axis, y_axis } => {
            let mut count = vec![0.0, 0.0];
            let length = &log_data_by_time.len();
            let model = Models {
                model: MODELTYPES::SCWIND,
                total_errors: total_log_data.errors as f64,
                log_data: log_data_by_time.clone(),
                data_point: *length,
            };
            let (mut second_point, _) = model.get_curve();
            let mut index = 0 as usize;
            for data in &log_data_by_time {
                match x_axis.trim() {
                    "time" => {
                        let (current_x, current_y) =
                            <LogData as Clone>::clone(&data).get_data_point("time", &y_axis);
                        second_point[index].0 = current_x;
                        data_point.push((current_x, count[1] as f64));
                        index += 1;
                        count[1] += current_y;
                    }
                    _ => {
                        let (current_x, current_y) =
                            <LogData as Clone>::clone(&data).get_data_point(&x_axis, &y_axis);
                        data_point.push((count[0], count[1] as f64));
                        second_point[index].0 = count[0];
                        count[0] += current_x;
                        count[1] += current_y;
                        index += 1;
                    }
                }
            }
            if x_axis.trim() == "time" {
                x_axis = time.to_string();
            }
            table::get_line_similarity(&data_point, &second_point);
            plot::plot_double_graph(
                data_point[0].0,
                data_point.last().unwrap().0,
                y_axis.to_owned(),
                x_axis.to_owned(),
                data_point,
                second_point,
            );
        }
        Commands::Ratio { mut x_axis, y_axis } => {
            // Move to different file later
            for data in log_data_by_time {
                //println!("{:#?}\n", data);
                let (x, mut y) = data.clone().get_data_point(&x_axis, &y_axis);
                y = data.get_data("errors") / y;

                //println!("{:?}", (x, y));
                data_point.push((x, y));
            }
            let y_axis = format!("errors/{y_axis}");
            match x_axis.trim() {
                "time" => {
                    x_axis = time.to_string();
                    plot::plot_graph(
                        data_point[0].0,
                        data_point.last().unwrap().0,
                        y_axis.to_owned(),
                        x_axis.to_owned(),
                        data_point,
                    );
                }
                _ => {
                    plot::plot_graph(
                        data_point[0].0,
                        data_point.last().unwrap().0,
                        y_axis.to_owned(),
                        x_axis.to_owned(),
                        data_point,
                    );
                }
            }
        }
        Commands::CumulativeRatio { y_axis } => {
            let length = &log_data_by_time.len();
            let mut count = vec![0.0, 0.0];
            let model = Models {
                model: MODELTYPES::SCWIND,
                total_errors: total_log_data.errors as f64,
                log_data: log_data_by_time.clone(),
                data_point: *length,
            };
            let (mut second_point, _) = model.get_curve();
            let mut index = 0;
            // Move to different file later
            for data in &log_data_by_time {
                //println!("{:#?}\n", data);
                let (current_x, current_y) = data.clone().get_data_point("time", &y_axis);
                let y = <LogData as Clone>::clone(&data).get_data("errors");
                second_point[index].0 = current_x;
                second_point[index].1 = second_point[index].1 / current_y;

                data_point.push((current_x, count[1] / current_y as f64));
                index += 1;
                count[1] += y;

                //println!("{:?}", (x, y));
            }
            let y_axis = format!("errors/{y_axis}");
            //table::get_line_similarity(&data_point, &second_point);
            plot::plot_double_graph(
                data_point[0].0,
                data_point.last().unwrap().0,
                y_axis.to_owned(),
                time.to_string(),
                data_point,
                second_point,
            );
        }
    }
}
