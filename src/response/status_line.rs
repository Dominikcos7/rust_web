#[derive(Debug)]
pub struct StatusLine {
    http_version: String,
    status_code: i32,
    reason_phrase: String,
}

impl StatusLine {
    pub fn as_string(self: &Self) -> String {
        format!("{} {} {}", self.http_version, self.status_code, self.reason_phrase)
    }

    pub fn from_str(s: &str) -> Self {
        let parts = s.split(" ").collect::<Vec<_>>();

        StatusLine { 
            http_version: parts[0].into(),
            status_code: parts[1].parse::<i32>().unwrap(),
            reason_phrase: parts[2].into(),
        }
    }
}