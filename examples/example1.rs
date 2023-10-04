extern crate mtime;

fn main() {
    // Create a new instance of Mtime
    let example = mtime::MTime::new(1234);
    // Get how many seconds from 0 to 59
    println!("Seconds: {}", example.get_visual_seconds());
    // Get how many minutes from 0 to 59
    println!("Minutes: {}", example.get_visual_minutes());
    // Get how many hours from 0 to 23
    println!("Hours: {}", example.get_visual_hours());
}
