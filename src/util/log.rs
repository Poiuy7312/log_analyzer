use std::{collections::HashMap, fmt, str};

// use chrono::{TimeZone, Utc};

#[derive(Clone, Debug)]
pub(crate) struct Log {
    pub(crate) ip: String,
    pub(crate) client_id: String,
    pub(crate) user_id: String,
    pub(crate) time: String,
    pub(crate) request: String,
    pub(crate) status_code: (u16, String),
    pub(crate) size: f64,
}

impl Log {
    pub(crate) fn get_parsed_date(self) -> Vec<u32> {
        // Turn month into associated number
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
        let mut date: Vec<u32> = date
            .into_iter()
            .map(|x| x.trim_matches('['))
            .map(|x| {
                x.parse()
                    .expect(&format!("Unable to parse into u32: {}", x))
            })
            .collect();
        let (year, day) = (date[2], date[0]);
        date[2] = day;
        date[0] = year;
        return date;
    }

    pub(super) fn get_values_string(self) -> String {
        return format!(
            "{},{},{},{},{}",
            self.ip, self.time, self.client_id, self.user_id, self.status_code.0,
        );
    }
    /* Deprecated
    pub(super) fn utc_time(self) -> i64 {
        let d1 = self.get_parsed_date();
        let time = Utc
            .with_ymd_and_hms(d1[0] as i32, d1[1], d1[2], d1[3], d1[4], d1[5])
            .unwrap()
            .timestamp();

        return time;
    }*/
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        <Log as Clone>::clone(&self).get_values_string()
            == <Log as Clone>::clone(&other).get_values_string()
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
