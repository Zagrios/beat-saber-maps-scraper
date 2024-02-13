use chrono::DateTime;

pub fn parse_date_rfc3339_to_timestamp(date: &str) -> i64 {
    let date = DateTime::parse_from_rfc3339(date).unwrap();
    date.timestamp()
}