extern crate mtime;

#[test]
fn format_date() {
    let test = mtime::MTime::new(123456789);
    let test_date = test.get_date();
    let formatted_string = test_date.format("%Y:%M:%D %h:%m:%s");
    assert_eq!(formatted_string, String::from("2003:11:29 21:33:09"));
}

#[test]
fn leap_year() {
    let test = mtime::MTime::new(123456789);
    let test_is_leap_year = test.is_leap_year();
    assert_eq!(test_is_leap_year, false);
}
