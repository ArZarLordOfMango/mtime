extern crate mtime;

fn main() {
    let example =  mtime::MTime::new(3600); // 1 hour
    // Getting various time-related information
    println!("Days: {}", example.get_count_days()); // Number of days
    println!("Years: {}", example.get_count_years()); // Number of years
    println!("Is Leap Year? {}", example.is_leap_year()); // Checking if it's a leap year
    println!("Hours: {}", example.get_visual_hours()); // Hours (formatted)
    println!("Minutes: {}", example.get_visual_minutes()); // Minutes (formatted)
    println!("Seconds: {}", example.get_visual_seconds()); // Seconds (formatted)
}

