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
        let http_version: String = parts[0].into();
        let status_code: i32 = parts[1].parse::<i32>().unwrap();
        let reason_phrase: String =  parts[2..].join(" ");

        StatusLine { 
            http_version: http_version,
            status_code: status_code,
            reason_phrase: reason_phrase,
        }
    }
}