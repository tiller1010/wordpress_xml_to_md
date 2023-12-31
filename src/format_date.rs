use chrono::DateTime;
use simpledateformat::fmt;

pub fn format_date(date: &str) -> Result<String, String> {
    // get the date from the pubDate tag
    let format = fmt("yyyy-MM-dd'T'HH:mm:ssz").unwrap();
    println!("{}", date);
    let date_time = DateTime::parse_from_rfc2822(&date).unwrap();
    let formatted_date = format.format(&date_time);
    println!("{}", formatted_date);
    Ok(formatted_date)
}

