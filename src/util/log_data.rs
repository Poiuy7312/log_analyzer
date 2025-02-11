use std::fmt;

#[derive(Clone)]
pub(crate) struct LogData {
    pub(crate) time: f64,
    pub(crate) users: usize,
    pub(crate) sessions: u64,
    pub(crate) total_bytes: f64,
    pub(crate) avg_bytes: f64,
    pub(crate) log_count: usize,
    pub(crate) errors: u64,
    pub(crate) atbl: f64,
    pub(crate) atbe: f64,
}
impl LogData {
    pub(crate) fn get_data(self, data_point: &str) -> f64 {
        match data_point {
            "time" => self.time,
            "users" => self.users as f64,
            "sessions" => self.sessions as f64,
            "total_bytes" => self.total_bytes,
            "avg_bytes" => self.avg_bytes,
            "hits" => self.log_count as f64,
            "errors" => self.errors as f64,
            "atbl" => self.atbl,
            "atbe" => self.atbe,
            _ => self.errors as f64,
        }
    }
    pub(crate) fn get_data_point(self, x_value: &str, y_value: &str) -> (f64, f64) {
        let x = match x_value {
            "time" => self.time,
            "users" => self.users as f64,
            "sessions" => self.sessions as f64,
            "total_bytes" => self.total_bytes,
            "avg_bytes" => self.avg_bytes,
            "hits" => self.log_count as f64,
            "errors" => self.errors as f64,
            "atbl" => self.atbl,
            "atbe" => self.atbe,
            _ => self.errors as f64,
        };
        let y = match y_value {
            "time" => self.time,
            "users" => self.users as f64,
            "sessions" => self.sessions as f64,
            "total_bytes" => self.total_bytes,
            "avg_bytes" => self.avg_bytes,
            "hits" => self.log_count as f64,
            "errors" => self.errors as f64,
            "atbl" => self.atbl,
            "atbe" => self.atbe,
            _ => self.errors as f64,
        };
        return (x, y);
    }
}

impl fmt::Display for LogData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,"Time(s): {}\nUser Count: {}\nSessions: {}\nTotal Bytes(mb): {}\nAvg Bytes(mb): {:.3}\nNumber of Logs: {}\nError Count: {}\nAvg time between Logs(s): {}\nAvg time between Errors(s): {}",
            self.time,self.users,self.sessions,self.total_bytes, self.avg_bytes, self.log_count, self.errors,self.atbl,self.atbe
        )
    }
}

impl fmt::Debug for LogData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,"Time(s): {}\nUser Count: {}\nSessions: {}\nTotal Bytes(mb): {}\nAvg Bytes(mb): {:.3}\nNumber of Logs: {}\nError Count: {}\nAvg time between Logs(s): {}\nAvg time between Errors(s): {}",
            self.time,self.users,self.sessions,self.total_bytes, self.avg_bytes, self.log_count, self.errors,self.atbl,self.atbe
        )
    }
}
