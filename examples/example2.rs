extern crate mtime;

fn main() {
    // Create a new instance of Mtime
    let example = mtime::MTime::new(1234); 
    // Create a new instance of Date
    let example_date = example.get_date(); 
    // Print Date in format %h:%m:%s %D.%M.%Y
    println!("{}", example_date.format("%h:%m:%s %D.%M.%Y")); 
}