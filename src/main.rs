use ansi_term::Colour::Cyan;
use indexmap::IndexMap;
use std::{
    collections::{HashMap, HashSet},
    io,
    path::Path,
    str,
};
use util::{log::Log, log_data::LogData, *};

mod plot;
mod util;

enum MODE {
    CUMULATIVE,
    TIME,
}

fn input(text: String) -> String {
    println!("\n{}", text);
    let stdin: io::Stdin = io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let input: String = input.trim().to_string();
    input
}

fn main() {
    let time_multiplier: HashMap<String, i64> = HashMap::from([
        ("year".to_string(), 31556952),
        ("month".to_string(), 2629800),
        ("day".to_string(), 86400),
        ("hour".to_string(), 3600),
        ("min".to_string(), 60),
        ("sec".to_string(), 1),
    ]);
    let mut choice = MODE::TIME;
    let mode = input(format!(
        "What mode do you want?{}",
        Cyan.paint("\ntime\ncumulative")
    ));
    if mode.trim().eq("cumulative") {
        choice = MODE::CUMULATIVE;
    }

    match choice {
        MODE::TIME => {
            let time = input(format!(
                "What do you wan't to split the logs by?{}",
                Cyan.paint("\nyear\nmonth\nday\nhour\nmin\nsec")
            ));
            let data_type = input(format!(
                "What do you want to track:{}",
                Cyan.paint("\nusers\nsessions\navg bytes\ntotal bytes\nerrors\nhits")
            ));
            let log_file = Path::new("poems.com - SSL Log (1).log");
            // Get log file contents
            let mut data: IndexMap<String, LogData> = IndexMap::new();
            let log_contents = read_file(log_file);
            let log_data = parse_log(log_contents);
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

                //println!("{:#?}", data);
            }

            // Move to different file later

            let mut data_point: Vec<(f64, f64)> = Vec::new();
            let mult = time_multiplier.get(&time).unwrap();
            for (_, stats) in &data {
                let column = time_difference(start.clone(), stats.time.clone());
                let column: f64 = (column as f64) / (*mult as f64);
                println!("{}", column);
                let data = stats.clone().get_data_type(&data_type);
                data_point.push((column, data));
            }
            let x_range = (total_time / mult + 1) as f64;

            plot::plot_graph(x_range, data_type, time, data_point);
        }
        MODE::CUMULATIVE => {
            let x_axis =
                input(format!("What is your x_axis?"{},Cyan.paint("\nusers\ntotal bytes\nhits")));

            let mut data_point: Vec<(f64, f64)> = Vec::new();
            let log_file = Path::new("poems.com - SSL Log (1).log");
            // Get log file contents
            //let mut data: IndexMap<String, LogData> = IndexMap::new();
            let log_contents = read_file(log_file);
            let log_data = parse_log(log_contents);
            // let hashed_logs: IndexMap<String, Vec<Log>> =
            //IndexMap::from([(String::from("all"), log_data.clone())]);
            // let (start, total_time) = get_time_scale(hashed_logs);
            let mut bytes = 0;
            let mut log_c = 0;
            let mut error = 0;
            let mut users_c: usize = 0;
            let logs_by = group_logs_by("min", log_data.clone());
            for (_, log) in logs_by.iter() {
                //println!("{:#?}", data);
                let users: IndexMap<String, Vec<Log>> = get_users(log.clone());

                //let sessions = get_sessions(7200, users);
                let (total_bytes, _) = get_byte_info(log.clone());
                let (_, error_count, log_count) = count_status_code(log.clone());
                error += error_count;
                log_c += log_count;
                bytes += total_bytes;
                users_c += users.len();
                data_point.push((log_c as f64, error as f64));
            }

            plot::plot_graph(log_c as f64, String::from("error"), x_axis, data_point);
        }
    }
}
