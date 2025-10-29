use time::OffsetDateTime;

pub fn format_date(date_str: &str) -> String {
    // Handle empty or too-short strings
    if date_str.len() < 10 {
        return "".to_owned();
    }
    
    // Take first 10 chars (YYYY-MM-DD) and split
    let parts: Vec<&str> = date_str[..10].split('-').collect();
    
    if parts.len() == 3 {
        let year = parts[0];
        let month = parts[1];
        let day = parts[2].trim_start_matches('0'); // Remove leading zero
        
        let month_name = match month {
            "01" => "January",
            "02" => "February",
            "03" => "March",
            "04" => "April",
            "05" => "May",
            "06" => "June",
            "07" => "July",
            "08" => "August",
            "09" => "September",
            "10" => "October",
            "11" => "November",
            "12" => "December",
            _ => return "".to_owned(),
        };
        
        format!("{} {}, {}", month_name, day, year)
    } else {
        "".to_owned()
    }
}

pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(' ', "-")
        .replace('&', "and")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

pub fn get_current_year() -> i32 {
    OffsetDateTime::now_utc().year()
}

