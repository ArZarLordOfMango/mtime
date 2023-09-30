extern crate mtime;

fn main() {
    // Exmaple work with Date
    let example = mtime::MTime::new(1234); 
    let example_date = example.get_date(); 
    println!("{}", example_date.format("%h:%m:%s %D.%M.%Y")); 
}
