use chrono::{DateTime, TimeZone, Utc};
use indexmap::IndexMap;
use log::Log;
use status_code::StatusCode;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::Path,
    str,
};

pub(crate) mod log;
pub(crate) mod log_data;
pub(crate) mod status_code;
/// Define log struct for parsing and storing server log information.

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
pub(crate) fn create_http_hashmap() -> HashMap<u16, String> {
    let mut code_map: HashMap<u16, String> = HashMap::new();
    let code_file = Path::new("src/data/http_codes.csv");
    let codes = read_file(code_file);
    for code in codes.lines() {
        let v: Vec<_> = code.split(",").collect();
        let key: Result<u16, std::num::ParseIntError> = v[0].parse();
        let value = v[1].to_string();
        match key {
            Ok(key) => {
                code_map.insert(key, value);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    return code_map;
}

/// Allows the logs to split by spaces and separate each part properly and stores them as a Log struct
pub(crate) fn parse_log(log_contents: String) -> Vec<log::Log> {
    let http_codes: HashMap<u16, String> = create_http_hashmap();
    // let mut http_code_count: HashMap<u16, u32> = HashMap::new();
    let mut logs: Vec<log::Log> = Vec::new();
    let contents: Vec<_> = log_contents.lines().collect();

    for log in contents {
        let log = log.to_string();
        let log = make_log_parsable(log);
        let parsed_log: Vec<_> = log.split(" ").collect();
        let status_code: u16 = parsed_log[5].parse().unwrap();
        let (status_code, description) = http_codes.get_key_value(&status_code).unwrap();
        let byte_size: u64 = parsed_log[6].parse().unwrap();
        logs.push(log::Log {
            ip: parsed_log[0].to_string(),
            client_id: parsed_log[1].to_string(),
            user_id: parsed_log[2].to_string(),
            time: parsed_log[3].to_string(),
            request: parsed_log[4].to_string(),
            status_code: (status_code.to_owned(), description.to_owned()),
            size: byte_size,
        });
    }
    return logs;
}

/// Group logs by a specified time range i.e.seconds,minutes,hours,days,months and years.
pub(crate) fn group_logs_by(range: &str, logs: Vec<log::Log>) -> IndexMap<String, Vec<log::Log>> {
    let mut log_by_hour: IndexMap<String, Vec<log::Log>> = IndexMap::new();
    let range_to_index: HashMap<&str, usize> = HashMap::from([
        ("sec", 5),
        ("min", 4),
        ("hour", 3),
        ("day", 2),
        ("month", 1),
        ("year", 0),
    ]);
    let range = range_to_index[range];
    for log in logs {
        let mut time_cat = String::new();
        let time = log.clone().get_parsed_date();
        for i in 0..time.len() {
            if i <= range {
                let mut time_add = time[i].to_string();
                time_add.push_str("|");
                time_cat.push_str(&time_add);
            }
        }
        if let Some(v_log) = log_by_hour.get_mut(&time_cat) {
            v_log.push(log);
        } else {
            log_by_hour.insert(time_cat, Vec::from([log]));
        }
    }
    return log_by_hour;
}

/// Average the amount of bytes transferred in logs
pub(crate) fn get_byte_info(logs: Vec<log::Log>) -> (u64, f64) {
    let log_count = logs.len();
    let mut total_bytes = 0;
    for log in logs {
        total_bytes += log.size;
    }
    let average = total_bytes as f64 / log_count as f64;
    return (total_bytes, average);
}

pub(crate) fn get_sessions(sec: i64, users: IndexMap<String, Vec<log::Log>>) -> u64 {
    let mut total_sessions: u64 = 0;
    for (_, user) in users {
        total_sessions += 1;
        let log_count = user.len();
        for i in 0..log_count - 1 {
            let st = user[i].clone().get_parsed_date();
            let difference = time_difference(st, user[i + 1].clone().get_parsed_date());
            if difference > sec {
                total_sessions += 1
            }
        }
    }
    return total_sessions;
}
/// Returns difference between two dates in seconds.
pub(crate) fn time_difference(d1: Vec<u32>, d2: Vec<u32>) -> i64 {
    let year1 = d1[2] as i32;
    let year2 = d2[2] as i32;
    let dt1: DateTime<Utc> = Utc
        .with_ymd_and_hms(year1, d1[1], d1[0], d1[3], d1[4], d1[5])
        .unwrap();
    let dt2: DateTime<Utc> = Utc
        .with_ymd_and_hms(year2, d2[1], d2[0], d2[3], d2[4], d2[5])
        .unwrap();
    let difference = dt1.timestamp() - dt2.timestamp();
    return difference.abs();
}

pub(crate) fn remove_repeated_events(logs: Vec<Log>) -> Vec<Log> {
    let mut repeat_set: HashSet<(String, String, String, String, (u16, String))> = HashSet::new();
    let mut log_list: Vec<Log> = Vec::new();
    for log in logs {
        let log_values = &log.clone().get_log_values();
        if !repeat_set.contains(log_values) {
            log_list.push(log.clone());
            repeat_set.insert(log_values.clone());
        }
    }
    return log_list;
}

/// Count the amount of times a status code appears in logs
pub(crate) fn count_status_code(logs: Vec<log::Log>) -> (IndexMap<u16, StatusCode>, u64, usize) {
    let log_count = logs.len();
    let mut status_count: IndexMap<u16, StatusCode> = IndexMap::new();
    let mut error_count = 0;
    for log in logs {
        let (status_code, description) = log.status_code;
        if status_code >= 400 {
            error_count += 1;
        }
        if let Some(code) = status_count.get_mut(&status_code) {
            code.count += 1;
            code.percent = code.count as f32 / log_count as f32 * 100.0;
        } else {
            status_count.insert(
                status_code,
                StatusCode {
                    status_code: status_code,
                    description: description.to_string(),
                    count: 1,
                    percent: 1.0 / log_count as f32 * 100.0,
                },
            );
        }
    }
    status_count.sort_keys();
    return (status_count, error_count, log_count);
}

pub(crate) fn get_users(logs: Vec<log::Log>) -> IndexMap<String, Vec<log::Log>> {
    let mut users: IndexMap<String, Vec<log::Log>> = IndexMap::new();
    for log in logs {
        let ip = log.clone().ip;
        if let Some(user) = users.get_mut(&ip) {
            user.push(log);
        } else {
            users.insert(ip.clone(), Vec::from([log]));
        }
    }
    return users;
}

pub(crate) fn get_time_scale(logs: IndexMap<String, Vec<Log>>) -> (Vec<u32>, i64) {
    let start = logs.first().unwrap().1;
    let start = &start[0];
    let end = logs.last().unwrap().1;
    let end = &end[0];
    let start_time = start.clone().get_parsed_date();
    let end_time = end.clone().get_parsed_date();
    let total_time = time_difference(start_time.clone(), end_time);
    return (start_time, total_time);
}

/// Allows the logs to split by spaces and separate each part properly
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
