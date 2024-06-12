extern crate mtime;

fn main() {
    let example =  mtime::MTime::new(0); // Creating Mtime exmple
    let example_date = example.get_date(); // Creating Date exmple
    let date_string = "10-05-00-21-07-2098"; //

    let parsed_date = example_date.from_string(date_string).unwrap();
    println!("{:?}", parsed_date.format("%h-%m-%s-%D-%M-%Y"));
    let time_mango = mtime::MTime::new(parsed_date.get_time());
    println!("{:?}", time_mango.get_date() );
} 