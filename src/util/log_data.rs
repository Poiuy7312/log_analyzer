use std::fmt;

#[derive(Clone)]
pub(crate) struct LogData {
    pub(crate) time: Vec<u32>,
    pub(crate) user_count: usize,
    pub(crate) sessions: u64,
    pub(crate) total_bytes: u64,
    pub(crate) avg_bytes: f64,
    pub(crate) log_count: usize,
    pub(crate) error_count: u64,
}

impl LogData {
    pub(crate) fn get_data_type(self, data_type: &str) -> f64 {
        match data_type {
            "users" => return self.user_count as f64,
            "avg byte" => return self.avg_bytes,
            "total bytes" => return self.total_bytes as f64,
            "sessions" => return self.sessions as f64,
            "errors" => return self.error_count as f64,
            "hits" => return self.log_count as f64,
            _ => return self.user_count as f64,
        }
    }
}

impl fmt::Display for LogData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "\nUser Count: {}\nSessions: {}\nTotal Bytes: {}\nAvg Bytes: {:.3}\nNumber of Logs: {}\nError Count: {}",
            self.user_count,self.sessions,self.total_bytes, self.avg_bytes, self.log_count, self.error_count
        )
    }
}

impl fmt::Debug for LogData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "\nUser Count: {}\nSessions: {}\nTotal Bytes: {}\nAvg Bytes: {:.3}\nNumber of Logs: {}\nError Count: {}",
            self.user_count,self.sessions,self.total_bytes, self.avg_bytes, self.log_count, self.error_count
        )
    }
}
