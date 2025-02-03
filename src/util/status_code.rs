use std::fmt;

#[derive(Clone)]
pub(crate) struct StatusCode {
    pub(crate) status_code: u16,
    pub(crate) description: String,
    pub(crate) count: u32,
    pub(crate) percent: f32,
}

impl fmt::Display for StatusCode {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StatusCode: {}\nDescription: {}\nCount: {}\nPercent: {:.8}\n",
            self.status_code, self.description, self.count, self.percent
        )
    }
}

impl fmt::Debug for StatusCode {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StatusCode: {}\nDescription: {}\nCount: {}\nPercent: {:.8}%\n",
            self.status_code, self.description, self.count, self.percent
        )
    }
}
