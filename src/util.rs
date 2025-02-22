use chrono::{TimeZone, Utc};
use indexmap::IndexMap;
use log::Log;
use std::{collections::HashMap, f64::consts::E, fs::File, io::Read, path::Path, str};

pub(super) mod log;
pub(crate) mod log_analyzer;
pub(super) mod log_data;

pub(crate) fn get_byte_info(logs: Vec<Log>) -> (f64, f64) {
    let log_count = logs.len();
    let mut total_bytes = 0.0;
    for log in logs {
        total_bytes += log.size;
    }
    let average = total_bytes as f64 / log_count as f64;
    return (total_bytes, average);
}

pub(crate) fn get_users(logs: Vec<Log>) -> (IndexMap<std::string::String, Vec<Vec<u32>>>, usize) {
    let mut users: IndexMap<String, Vec<Vec<u32>>> = IndexMap::new();
    let mut count = 0 as usize;
    for log in logs {
        let ip = log.clone().ip;
        if let Some(user) = users.get_mut(&ip) {
            user.push(log.get_parsed_date());
        } else {
            users.insert(ip.clone(), Vec::from([log.get_parsed_date()]));
            count += 1
        }
    }
    return (users, count);
}
pub(crate) fn get_avg_time(logs: Vec<Log>) -> f64 {
    let total_logs = logs.len() as i64;
    let range = logs.len() as i64 - 1;
    let mut avg_time = 0.0;
    for i in 0..range as usize {
        let time_one = <log::Log as Clone>::clone(&logs[i]).get_parsed_date();
        let time_two = <log::Log as Clone>::clone(&logs[i + 1]).get_parsed_date();
        avg_time += time_difference(to_time(&time_one), to_time(&time_two));
    }
    return avg_time / total_logs as f64;
}
/// Returns difference between two dates in seconds.
pub(crate) fn time_difference(d1: f64, d2: f64) -> f64 {
    let difference = d2 - d1;
    return difference.abs();
}

pub(crate) fn to_time(d1: &Vec<u32>) -> f64 {
    let time: i64 = Utc
        .with_ymd_and_hms(d1[2] as i32, d1[1], d1[0], d1[3], d1[4], d1[5])
        .unwrap()
        .timestamp();

    return time as f64;
}

pub(crate) fn get_sessions(sec: f64, users: IndexMap<String, Vec<Vec<u32>>>) -> u64 {
    let mut total_sessions: u64 = 0;
    for user in users.values() {
        total_sessions += 1;
        let log_count = user.len();
        for i in 0..log_count - 1 {
            let st = to_time(&user[i]);
            let difference = time_difference(st, to_time(&user[i + 1]));
            if difference > sec {
                total_sessions += 1
            }
        }
    }
    return total_sessions;
}

fn model(b: f64, t: f64) -> f64 {
    let one = 1.0 / (f64::powf(E, b) - 1.0);
    let two = t / (f64::powf(E, b * t) - 1.0);
    return one - two;
}

fn model_prime(b: f64, t: f64) -> f64 {
    let one = f64::powf(E, b) / f64::powi(f64::powf(E, b) - 1.0, 2);
    let two = f64::powi(t, 2) * f64::powf(E, b * t) / f64::powi(f64::powf(E, b * t) - 1.0, 2);
    return two - one;
}

pub(crate) fn newton_raphson(b: f64, t: f64, c: f64) -> f64 {
    let mut b = b;
    for _ in 0..100 {
        let f_val = model(b, t) - c;
        let f_deriv = model_prime(b, t);
        let new_b = b - f_val / f_deriv;
        println!("{}", new_b);

        if (new_b - b).abs() < 1e-4 {
            return new_b;
        }
        b = new_b;
    }
    return b;
}

pub(crate) fn count_errors(logs: Vec<Log>) -> (u64, Vec<Log>) {
    let mut error_count = 0;
    let mut error_logs: Vec<Log> = Vec::new();
    for log in logs {
        let (status_code, _) = log.status_code;
        if status_code >= 400 {
            error_count += 1;
            error_logs.push(log);
        }
    }
    return (error_count, error_logs);
}

/// Takes path to a log file and reads and returns the results a String.
pub(crate) fn read_file(log_file: &Path) -> String {
    let mut file = File::open(log_file).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("unable to read file");
    let content = contents.as_slice();
    return String::from_utf8_lossy(&content).to_string();
}

/// Creates a hashmap of all HTTP status codes
pub(crate) fn create_http_hashmap() -> HashMap<String, String> {
    let mut code_map: HashMap<String, String> = HashMap::new();
    let code_file = Path::new("src/data/http_codes.csv");
    let codes = read_file(code_file);
    for code in codes.lines() {
        let v: Vec<_> = code.split(",").collect();
        let key = v[0];
        let value = v[1];
        code_map.insert(key.to_string(), value.to_string());
    }
    return code_map;
}

/// Allows the logs to split by spaces and separate each part properly and stores them as a Log struct
pub(crate) fn parse_log(log_contents: String) -> Vec<log::Log> {
    let http_codes: HashMap<String, String> = create_http_hashmap();
    let mut logs: Vec<log::Log> = Vec::new();
    let contents: Vec<_> = log_contents.lines().collect();

    for log in contents {
        let log = make_log_parsable(log.to_string());
        let parsed_log: Vec<String> = log.split(" ").map(|x| x.to_string()).collect();
        let (status_code, description) = http_codes.get_key_value(&parsed_log[5]).unwrap();
        let status_code = match status_code.parse::<u16>() {
            Ok(x) => x,
            Err(e) => {
                println!("Error converting number: {}", e);
                404
            }
        };

        let byte_size = match parsed_log[6].parse::<f64>() {
            Ok(x) => x / 1000.0,
            Err(e) => {
                println!("Error converting number: {}", e);
                0.0
            }
        };
        logs.push(log::Log {
            ip: parsed_log[0].to_owned(),
            client_id: parsed_log[1].to_owned(),
            user_id: parsed_log[2].to_owned(),
            time: parsed_log[3].to_owned(),
            request: parsed_log[4].to_owned(),
            status_code: (status_code, description.to_owned()),
            size: byte_size,
        });
    }
    return logs;
}

pub(crate) fn make_log_parsable(log: String) -> String {
    let mut in_block: bool = false;
    let mod_symbols: Vec<char> = vec!['[', ']', '"'];
    let mut parsable_str = String::new();
    for mut c in log.chars() {
        match in_block {
            false => {
                if mod_symbols.contains(&c) {
                    in_block = true;
                }
                parsable_str.push(c);
            }
            true => {
                if mod_symbols.contains(&c) {
                    in_block = false;
                }
                if c.is_whitespace() {
                    c = '%';
                }
                parsable_str.push(c);
            }
        }
    }
    return parsable_str;
}
