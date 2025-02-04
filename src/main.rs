use indexmap::IndexMap;
use std::{collections::HashMap, path::Path};

use util::{log::Log, log_data::LogData, *};

use clap::Parser;

mod plot;
mod util;

#[derive(Parser)]
struct Cli {
    /// time|cumulative|errorcomp|
    mode: String,
    /// errors|total bytes|avg bytes|sessions|hits|
    data: String,
    /// year|month|day|hour|min|sec|
    time: String,
}

fn main() {
    let args = Cli::parse();
    let log_file = Path::new("poems.com - SSL Log (1).log");
    let log_contents = read_file(log_file);
    // Puts all logs in a vector tracking each part and removing redundant logs.
    let log_data = remove_repeated_events(parse_log(log_contents));
    let mut data_point: Vec<(f64, f64)> = Vec::new();

    let time_multiplier: HashMap<String, i64> = HashMap::from([
        ("year".to_string(), 31556952),
        ("month".to_string(), 2629800),
        ("day".to_string(), 86400),
        ("hour".to_string(), 3600),
        ("min".to_string(), 60),
        ("sec".to_string(), 1),
    ]);
    let mode = args.mode;

    let time = args.time.trim();
    println!("{}", time);

    let mut data: IndexMap<String, LogData> = IndexMap::new();
    let logs_by = group_logs_by(&time, log_data.clone());
    let (start, total_time) = get_time_scale(logs_by.clone());
    for (time, logs) in logs_by.iter() {
        let users: IndexMap<String, Vec<Log>> = get_users(logs.clone());
        let user_count = &users.len();
        let sessions = get_sessions(7200, users);
        let (total_bytes, byte_average) = get_byte_info(logs.clone());
        let (_, error_count, log_count) = count_status_code(logs.clone());

        data.insert(
            time.to_string(),
            LogData {
                time: logs[0].clone().get_parsed_date(),
                user_count: *user_count,
                sessions: sessions,
                total_bytes: total_bytes,
                avg_bytes: byte_average,
                log_count: log_count,
                error_count: error_count,
            },
        );
    }

    let data_type = args.data;
    match mode.trim() {
        "time" => {
            // Move to different file later

            let mult = time_multiplier.get(time).unwrap();
            for (_, stats) in &data {
                let column = time_difference(start.clone(), stats.time.clone());
                let column: f64 = (column as f64) / (*mult as f64);
                let data = stats.clone().get_data_type(&data_type);
                data_point.push((column, data));
            }
            let x_range = (total_time / mult + 1) as f64;

            plot::plot_graph(x_range, data_type, time.to_owned(), data_point);
        }
        "cumulative" => {
            // Get log file contents
            //let mut data: IndexMap<String, LogData> = IndexMap::new();

            // let hashed_logs: IndexMap<String, Vec<Log>> =
            //IndexMap::from([(String::from("all"), log_data.clone())]);
            // let (start, total_time) = get_time_scale(hashed_logs);
            let mut count = vec![0.0, 0.0];
            for (_, stats) in data {
                count[0] += stats.clone().get_data_type(&data_type);
                count[1] += stats.get_data_type("errors");
                data_point.push((count[0], count[1] as f64));
            }
            plot::plot_graph(count[0], String::from("error"), data_type, data_point);
        }
        "errorcomp" => {
            let mut count = vec![0.0, 0.0];
            let datatype = format!("errors/{}", data_type);
            for (_, stats) in data {
                count[0] += stats.clone().get_data_type("errors");
                count[0] /= stats.clone().get_data_type(&data_type);
                let current_time = time_difference(start.clone(), stats.time.clone());
                count[1] = current_time as f64 / total_time as f64;
                data_point.push((count[1], count[0]));
            }
            plot::plot_graph(1.0, datatype, String::from("time/total time"), data_point);
        }

        _ => {
            println!("No value");
        }
    }
}
