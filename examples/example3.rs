extern crate mtime;

fn main() {
    let custom_date = mtime::Date::builder()
    .year("2023")
    .month("10")
    .day("13")
    .hour("15")
    .minute("30")
    .second("45")
    .build()
    .expect("Failed to create custom date"); // Building a custom Date object

println!("Custom Date: {:?}", custom_date);
}