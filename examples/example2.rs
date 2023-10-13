extern crate mtime;

fn main() {
    let example =  mtime::MTime::new(366000); 
    let example_date = example.get_date();
    println!("Formatted Date: {}", example_date.format("%Y-%M-%D %h:%m:%s")); // Example format
}