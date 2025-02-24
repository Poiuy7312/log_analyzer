use std::{
    collections::HashMap,
    f64::{self, consts::E},
    path::Path,
};

use clap::Parser;
use util::{log_analyzer::LogAnalyzer, log_data::LogData, models::*, parse_log, read_file};

mod plot;
mod util;

#[derive(Parser)]
struct Cli {
    /// by|cumulative|curve|
    mode: String,
    /// errors|total_bytes|avg_bytes|sessions|hits|
    x_axis: String,
    y_axis: String,
    /// year|month|day|hour|min|sec|
    time: String,
}

enum MODE {
    BY,
    CUMULATIVE,
    RATIO,
    RATIOCUM,
    NA,
}

/*fn get_curve(total_errors: f64, log_data_by_time: &Vec<LogData>, length: usize) -> Vec<(f64, f64)> /* , Vec<(f64, f64)>)*/
{
    let mut data_point: Vec<(f64, f64)> = Vec::new();
    //let mut data_rate: Vec<(f64, f64)> = Vec::new();
    let mut rate = 0.0;
    for log in log_data_by_time {
        let current_errors = log.errors as f64;
        rate += log.time * current_errors / total_errors;
    }
    let mut starting_value = 0.1;
    let mut b = f64::NAN;
    while b.is_nan() {
        b = newton_raphson(
            starting_value,
            log_data_by_time[length - 1].time as f64,
            rate,
        );
        starting_value = f64::powi(starting_value, 10);
    }
    let a = b * total_errors / (1.0 - f64::powf(E, -b * log_data_by_time[length - 1].time as f64));

    for i in 0..length {
        let y = a / b * (1.0 - f64::powf(E, -b * log_data_by_time[i].time as f64));
        //let y2 = a * f64::powf(E, -b * log_data_by_time[i].time);

        data_point.push((log_data_by_time[i].time as f64, y));
        //data_rate.push((log_data_by_time[i].time as f64, y2));
    }
    return data_point; //, data_rate);
}*/

fn main() {
    let args = Cli::parse();
    let time = args.time.trim();
    let log_file = Path::new("poems.com - SSL Log (1).log");
    let log_contents = read_file(log_file);
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
    let (log_data_by_time, total_log_data) = log_data.get_data(time);
    println!("{}", total_log_data);
    let mut data_point: Vec<(f64, f64)> = Vec::new();

    let mode = match args.mode.trim() {
        "by" => MODE::BY,
        "cumulative" => MODE::CUMULATIVE,
        "ratio" => MODE::RATIO,
        "ratio_cum" => MODE::RATIOCUM,
        _ => MODE::NA,
    };

    let mut x_axis = args.x_axis.trim();
    let y_axis = args.y_axis.trim();
    //println!("{}", time);
    match mode {
        MODE::BY => {
            let length = &log_data_by_time.len();
            let model = Models {
                model: MODELTYPES::SCWIND,
                total_errors: total_log_data.errors as f64,
                log_data: log_data_by_time.clone(),
                data_point: *length,
            };
            let (_, second_prime) = model.get_curve();
            // Move to different file later
            for data in log_data_by_time {
                //println!("{:#?}\n", data);
                let (x, y) = data.get_data_point(x_axis, y_axis);

                //println!("{:?}", (x, y));
                data_point.push((x, y));
            }
            match x_axis.trim() {
                "time" => {
                    x_axis = time;
                    plot::plot_double_graph(
                        data_point[0].0,
                        data_point.last().unwrap().0,
                        y_axis.to_owned(),
                        x_axis.to_owned(),
                        data_point,
                        second_prime,
                    );
                }
                _ => {
                    data_point.sort_by(|a, b| (a.0 as i64).cmp(&(b.0 as i64)));
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

        MODE::CUMULATIVE => {
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
                            <LogData as Clone>::clone(&data).get_data_point("time", y_axis);
                        second_point[index].0 = current_x;
                        data_point.push((current_x, count[1] as f64));
                        index += 1;
                        count[1] += current_y;
                    }
                    _ => {
                        let (current_x, current_y) =
                            <LogData as Clone>::clone(&data).get_data_point(x_axis, y_axis);
                        data_point.push((count[0], count[1] as f64));
                        second_point[index].0 = count[0];
                        count[0] += current_x;
                        count[1] += current_y;
                        index += 1;
                    }
                }
            }
            plot::plot_double_graph(
                data_point[0].0,
                data_point.last().unwrap().0,
                y_axis.to_owned(),
                x_axis.to_owned(),
                data_point,
                second_point,
            );
        }
        MODE::RATIO => {
            // Move to different file later
            for data in log_data_by_time {
                //println!("{:#?}\n", data);
                let (x, mut y) = data.clone().get_data_point(x_axis, y_axis);
                y = data.get_data("errors") / y;

                //println!("{:?}", (x, y));
                data_point.push((x, y));
            }
            let y_axis = format!("errors/{y_axis}");
            match x_axis.trim() {
                "time" => {
                    x_axis = time;
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
        MODE::RATIOCUM => {
            let length = &log_data_by_time.len();
            let mut count = vec![0.0, 0.0];
            let model = Models {
                model: MODELTYPES::SCWIND,
                total_errors: total_log_data.errors as f64,
                log_data: log_data_by_time.clone(),
                data_point: *length,
            };
            let (mut second_point, second_prime) = model.get_curve();
            let mut index = 0;
            // Move to different file later
            for data in &log_data_by_time {
                //println!("{:#?}\n", data);
                let (x, current_y) = data.clone().get_data_point("time", y_axis);
                let y = <LogData as Clone>::clone(&data).get_data("errors");
                println!("{}", second_point[index].1);
                second_point[index].1 /= current_y;
                println!("{}", second_point[index].1);
                index += 1;

                data_point.push((x, count[1] / current_y as f64));
                count[1] += y;

                //println!("{:?}", (x, y));
            }
            let y_axis = format!("errors/{y_axis}");
            x_axis = time;
            plot::plot_double_graph(
                data_point[0].0,
                data_point.last().unwrap().0,
                y_axis.to_owned(),
                x_axis.to_owned(),
                data_point,
                second_point,
            );
        }
        MODE::NA => {
            println!("Invalid mode try again")
        }
    }
}
