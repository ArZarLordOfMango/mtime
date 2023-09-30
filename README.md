# mtime

Unix time like library for Rust

Start point(SP) it's 00:00:00 01.01.2000.

## Mtime Methods

| name                 | return type | description                                      |
|----------------------|-------------|--------------------------------------------------|
| get_count_days()     | u64         | returns number of days from SP.                  |
| get_count_years()    | u64         | returns number of year from SP.                  |
| is_leap_year()       | bool        | returns true if current year is leap else false. |
| get_seconds()        | u8          | returns seconds.                                 |
| get_visual_seconds() | String      | returns formatted seconds.                       |
| get_minutes()        | u8          | returns minutes.                                 |
| get_visual_minutes() | String      | returns formatted minutes.                       |
| get_hours()          | u8          | returns hours.                                   |
| get_visual_hours()   | String      | returns formatted hours.                         |
| get_day()            | u8          | returns day.                                     |
| get_visual_day()     | String      | returns the formatted day.                       |
| get_month()          | u8          | returns the month number.                        |
| get_visual_month()   | String      | returns the formatted month number.              |
| get_month_name()     | String      | returns the month name.                          |
| get_year()           | u64         | returns the year.                                |
| get_visual_year()    | String      | returns the formatted year.                      |
| get_date()           | Date        | returns Date                                     |

## Date Methods

| name                 | return type | description                                      |
|----------------------|-------------|--------------------------------------------------|
| format(format)       | String      | returns formate Date.                            |

### Date format

| name format          | description |
|----------------------|-------------|
| %s                   | seconds     |
| %m                   | minutes     |
| %h                   | hours       |
| %D                   | day         |
| %M                   | month       |
| %Y                   | year        |

## Example

```rust
// Create a new instance of Mtime
let example = mtime::Mtime::new(1234);
```

```rust
// Get how many seconds from 0 to 59
let example = mtime::Mtime::new(1234);
println!("Seconds: {}", example.get_visual_seconds());
```

```rust
// Exmaple work with Date
let example = mtime::Mtime::new(1234); 
let example_date = exmaple.get_date(); 
println!("{}", example_date.format("%h:%m:%s %D.%M.%Y")); 
```
