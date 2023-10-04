extern crate mtime;

fn main() {
    // Create a new instance of Mtime and print time
    let example = mtime::MTime::new(1234);
    println!("Time: {}", example.time);
}
