use std::{collections::HashMap, fmt, str};

#[derive(Clone, Debug)]
pub(crate) struct Log {
    pub(crate) ip: String,
    pub(crate) client_id: String,
    pub(crate) user_id: String,
    pub(crate) time: String,
    pub(crate) request: String,
    pub(crate) status_code: (u16, String),
    pub(crate) size: u64,
}

impl Log {
    pub(crate) fn get_parsed_date(self) -> Vec<u32> {
        let month_map: HashMap<&str, u32> = HashMap::from([
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("May", 5),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ]);
        let mut date: Vec<_> = self.time.split(['/', ':', '%']).collect();
        date.pop();
        let binding = month_map.get(date[1]).unwrap().to_string();
        date[1] = binding.as_str();
        let date: Vec<u32> = date
            .iter()
            .map(|x| x.trim_matches('['))
            .map(|x| {
                x.parse()
                    .expect(&format!("Unable to parse into u32: {}", x))
            })
            .collect();
        return date;
    }
    pub(crate) fn get_log_values(self) -> (String, String, String, String, (u16, String)) {
        return (
            self.ip,
            self.client_id,
            self.user_id,
            self.request,
            self.status_code,
        );
    }
}
impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ip: {}\nclient_id: {}\nuser_id: {}\ntime: {}\nrequest: {}\nstatus_code: {:?}\nsize: {}\n",
            self.ip, self.client_id, self.user_id,self.time,self.request,self.status_code,self.size
        )
    }
}
