use chrono::DateTime;
use simpledateformat::fmt;

pub fn format_date(date: &str) -> Result<String, String> {
    // get the date from the pubDate tag
    let format = fmt("%Y-%m-%d %H:%M:%S").unwrap();
    let date_time = DateTime::parse_from_rfc2822(&date).unwrap();
    let formatted_date = format.format(&date_time);
    println!("{}", formatted_date);
    Ok(formatted_date)
}

