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
        let date: Vec<_> = self.time.split(['/', ':', '%']).collect();
        let day: u32 = date[0].trim_matches('[').parse().unwrap();
        let month = month_map.get(date[1]).unwrap().to_owned();
        let year: u32 = date[2].parse().unwrap();
        let hour: u32 = date[3].parse().unwrap();
        let min: u32 = date[4].parse().unwrap();
        let sec: u32 = date[5].parse().unwrap();
        let ordered_date: Vec<u32> = Vec::from([year, month, day, hour, min, sec]);

        return ordered_date;
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
