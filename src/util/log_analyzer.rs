use std::collections::HashMap;

use indexmap::IndexMap;

use super::{log_data::LogData, *};

#[derive(Clone)]
pub(crate) struct LogAnalyzer {
    pub(crate) logs: Vec<Log>,
    pub(crate) time_multi: i64,
}

impl LogAnalyzer {
    /// Group the logs by a specified time and store the stats in as well
    fn group_logs_by(self, range: &str) -> IndexMap<String, Vec<Log>> {
        let mut log_by_hour: IndexMap<String, Vec<Log>> = IndexMap::new();
        let range_to_index: HashMap<&str, usize> = HashMap::from([
            ("sec", 6),
            ("min", 5),
            ("hour", 4),
            ("day", 3),
            ("month", 2),
            ("year", 1),
        ]);
        let range = range_to_index[range];
        for log in self.logs {
            let mut time_group = String::new();
            let mut time = log.clone().get_parsed_date();
            // removes any values outside the specified range
            time.drain(range..);
            let time: String = time
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("|");
            time_group.push_str(&time);
            if let Some(v_log) = log_by_hour.get_mut(&time_group) {
                v_log.push(log);
            } else {
                log_by_hour.insert(time_group, Vec::from([log]));
            }
        }

        return log_by_hour;
    }

    fn get_total_data(self) -> LogData {
        let start_time = self
            .logs
            .clone()
            .into_iter()
            .map(|x| to_time(&x.get_parsed_date()) as i64)
            .min()
            .unwrap();
        let end_time = self
            .logs
            .clone()
            .into_iter()
            .map(|x| to_time(&x.get_parsed_date()) as i64)
            .max()
            .unwrap();
        let time =
            time_difference(start_time as f64, end_time as f64) as f64 / self.time_multi as f64;
        let log_count = self.logs.len();
        let (users, count) = get_users(self.logs.to_vec());
        let sessions = get_sessions(7200.0, users);
        let (errors, error_logs) = count_errors(self.logs.to_vec());
        let (total_bytes, avg_bytes) = get_byte_info(self.logs.to_vec());
        let atbl = get_avg_time(self.logs.to_vec());
        let atbe = get_avg_time(error_logs);
        let data = LogData {
            time,
            users: count,
            sessions,
            total_bytes,
            avg_bytes,
            errors,
            log_count,
            atbl,
            atbe,
        };
        return data;
    }

    pub(crate) fn get_data(self, time: &str) -> (Vec<LogData>, LogData) {
        let start_time = self
            .logs
            .clone()
            .into_iter()
            .map(|x| to_time(&x.get_parsed_date()) as i64)
            .min()
            .unwrap();
        let grouped_logs = self.clone().group_logs_by(time);
        let mut data: Vec<LogData> = Vec::new();
        for group in grouped_logs.values() {
            let log_count = group.len();
            let time = time_difference(
                start_time as f64,
                to_time(&<log::Log as Clone>::clone(&group[0]).get_parsed_date()),
            ) / self.time_multi as f64;
            let (users, count) = get_users(group.to_vec());
            let sessions = get_sessions(7200.0, users);
            let (errors, error_logs) = count_errors(group.to_vec());
            let (total_bytes, avg_bytes) = get_byte_info(group.to_vec());
            let atbl = get_avg_time(group.to_vec());
            let atbe = match error_logs.len() {
                0 => 0.0,
                _ => get_avg_time(error_logs),
            };

            data.push(LogData {
                time,
                users: count,
                sessions,
                total_bytes,
                avg_bytes,
                errors,
                log_count,
                atbl,
                atbe,
            })
        }
        let total_data = self.get_total_data();
        /* `LogData` value */
        return (data, total_data);
    }
}
