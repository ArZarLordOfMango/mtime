extern crate mtime;

fn main() {
    let example =  mtime::MTime::new(0); 
    let example_date = example.get_date();
    let date_string = "15-30-45-13-10-2023"; // Format: %h-%m-%s-%D-%M-%Y

    // Parsing a date from the string
    let parsed_date = example_date.from_string(date_string).unwrap();
    
    println!("Parsed Date: {:?}", parsed_date); // Displaying the parsed Date
    
    // Formatting the parsed Date according to a custom format string
    let formatted_date = parsed_date.format("%h:%m:%s %Y-%M-%D");
    println!("Formatted Date: {}", formatted_date);
}