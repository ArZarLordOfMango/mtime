extern crate mtime;

fn main() {
    // Get how many seconds from 0 to 59
    let example = mtime::MTime::new(1234);
    println!("Seconds: {}", example.get_visual_seconds());
}
