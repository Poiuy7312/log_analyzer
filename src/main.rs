use std::{collections::HashMap, f64::consts::E, path::Path};

use clap::Parser;
use util::{log_analyzer::LogAnalyzer, newton_raphson, parse_log, read_file};

mod plot;
mod util;

#[derive(Parser)]
struct Cli {
    /// time|cumulative|ratio|
    mode: String,
    /// errors|total_bytes|avg_bytes|sessions|hits|
    x_axis: String,
    y_axis: String,
    /// year|month|day|hour|min|sec|
    time: String,
}

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
    }
    .remove_repeats();
    let (log_data_by_time, total_log_data) = log_data.get_data(time);
    let mut data_point: Vec<(f64, f64)> = Vec::new();

    let mode = args.mode.trim();
    let x_axis = args.x_axis.trim();
    let y_axis = args.y_axis.trim();
    //println!("{}", time);
    match mode.trim() {
        "time" => {
            let total_time = total_log_data.time;
            // Move to different file later
            for data in log_data_by_time {
                //println!("{:#?}\n", data);
                let (x, y) = data.get_data_point("time", y_axis);

                //println!("{:?}", (x, y));
                data_point.push((x, y));
            }
            // println!("{:#?}", data_point);
            plot::plot_graph(
                total_time * 1.1,
                y_axis.to_owned(),
                time.to_owned(),
                data_point,
            );
        }

        "cumulative" => {
            let mut count = vec![0.0, 0.0];
            for data in log_data_by_time {
                let (current_x, current_y) = data.get_data_point(x_axis, y_axis);
                count[0] += current_x;
                count[1] += current_y;
                data_point.push((count[0], count[1] as f64));
            }
            plot::plot_graph(count[0], y_axis.to_string(), x_axis.to_string(), data_point);
        }
        "curve" => {
            let total_errors = total_log_data.errors as f64;
            let t = log_data_by_time.len();
            let mut rate = 0.0;
            let mut count = 1.0;
            for log in &log_data_by_time {
                let current_errors = log.errors as f64;
                rate += count as f64 * current_errors / total_errors;
                count += 1.0;
            }

            let b = newton_raphson(0.1, t as f64, rate);
            let a = b * total_errors / (1.0 - f64::powf(E, -b * t as f64));
            println!("{},{}", a, b);
            for i in 0..t {
                let y = a / b * (1.0 - f64::powf(E, -b * i as f64));

                data_point.push((i as f64, y));
            }
            plot::plot_graph(
                t as f64,
                format!("Model"),
                format!("Time Interval"),
                data_point,
            );
        }
        /*
        let mut count = vec![0.0, 0.0];
        let datatype = format!("errors/{}", data_type);
        for (_, stats) in data {
        count[0] =
        stats.clone().get_data_type("errors") / stats.clone().get_data_type(&data_type);
        let current_time = stats.time;
        count[1] = current_time as f64 / total_time as f64;
        data_point.push((count[1], count[0]));
        }
        plot::plot_graph(1.0, datatype, String::from("time/total time"), data_point);
        }
        "curve" => {
        let mut count = vec![0.0, 0.0];
        for (_, stats) in &data {
        count[0] += stats.clone().get_data_type(&data_type);
        count[1] += stats.clone().get_data_type("errors");
        data_point.push((count[0], count[1] as f64));
        }
        let max = count[0].clone();
        let x_avg = count[0] / data_point.len() as f64;
        let y_avg = count[1] / data_point.len() as f64;
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        for data in &data_point {
        numerator += (data.0 - x_avg) * (data.1 - y_avg);
        denominator += f64::powi(data.0 - x_avg, 2);
        }
        let b = numerator / denominator;
        data_point.clear();

        for stats in data.values() {
        let x = stats.clone().get_data_type(&data_type);
        let y = total_errors * (1.0 - f64::powf(E, -b * x));
        data_point.push((x, y));
        }

        let mut b: f64 = 0.0;
        let mut current_error_count = 0.0;
        for i in 0..data.len() {
        current_error_count += data[i].clone().get_data_type("errors");
        b += (1.0 - (current_error_count / (total_errors * 1.5))).ln();
        }
        println!("{}", b);
        let datatype = format!("errors/total error");
        //let start =
        //    data[0].clone().get_data_type("errors") / data[0].clone().get_data_type(&data_type);
        //let last = data.last().unwrap().1;
        //let last =
        //  last.clone().get_data_type("errors") / last.clone().get_data_type(&data_type);
        let total_data = get_total(&data_type, data.clone());
        for (_, stats) in data {
        let x = stats.clone().get_data_type(&data_type) / total_data;
        let m = -b * x;
        println!("{}", f64::powf(E, m));
        let y = total_errors * (1.0 - f64::powf(E, m));
        //let y = Decimal::from_f64_retain(f64::powf(E, m));
        data_point.push((x, y));
        println!("{:#?},{},{}", data_point, m, b);
        println!("{}", x);
        }
        //plot::plot_graph(max * 1.1, "errors".to_string(), data_type, data_point);
         */
        _ => {}
    }
}
