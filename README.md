# mtime

Unix time like library for Rust

Start point(SP) it's 00:00:00 01.01.2000.

# Mtime Struct

The `Mtime` struct represents a moment in time and provides various methods for working with time components.

| Field        | Type  | Description                                              |
|--------------|-------|----------------------------------------------------------|
| `time`       | `u64` | Time in Unix Timestamp format (seconds since the epoch). |

## Mtime Methods

| Method                 | Return Type | Description                                                            |
|------------------------|-------------|------------------------------------------------------------------------|
| `get_count_days()`     | `u64`       | Returns the number of days from the start point (SP).                  |
| `get_count_years()`    | `u64`       | Returns the number of years from the start point (SP).                 |
| `is_leap_year()`       | `bool`      | Returns `true` if the current year is a leap year; otherwise, `false`. |
| `get_seconds()`        | `u8`        | Returns the seconds component.                                         |
| `get_visual_seconds()` | `String`    | Returns the formatted seconds.                                         |
| `get_minutes()`        | `u8`        | Returns the minutes component.                                         |
| `get_visual_minutes()` | `String`    | Returns the formatted minutes.                                         |
| `get_hours()`          | `u8`        | Returns the hours component.                                           |
| `get_visual_hours()`   | `String`    | Returns the formatted hours.                                           |
| `get_day()`            | `u8`        | Returns the day component.                                             |
| `get_visual_day()`     | `String`    | Returns the formatted day.                                             |
| `get_month()`          | `u8`        | Returns the month number.                                              |
| `get_visual_month()`   | `String`    | Returns the formatted month number.                                    |
| `get_month_name()`     | `String`    | Returns the name of the month.                                         |
| `get_year()`           | `u64`       | Returns the year.                                                      |
| `get_visual_year()`    | `String`    | Returns the formatted year.                                            |
| `get_date()`           | `Date`      | Returns a `Date` object.                                               |

# Date Struct

The `Date` struct represents a date and time and provides methods for formatting.

| Field        | Type     | Description                                      |
|--------------|----------|--------------------------------------------------|
| `year`       | `String` | The year as a string.                            |
| `month`      | `String` | The month as a string.                           |
| `day`        | `String` | The day as a string.                             |
| `hour`       | `String` | The hour as a string.                            |
| `minute`     | `String` | The minute as a string.                          |
| `second`     | `String` | The second as a string.                          |

## Date Methods

| Method                           | Return Type            | Description                                                |
|----------------------------------|------------------------|------------------------------------------------------------|
| `format(format: &str)`           | `String`               | Formats the date according to the specified format string. |
| `from_string(date_string: &str)` | `Result<Date, String>` | Create Date from string(%h-%m-%s-%D-%M-%Y format only‼️).   |

## Date Format Codes

- `%s`: Represents seconds.
- `%m`: Represents minutes.
- `%h`: Represents hours.
- `%D`: Represents the day.
- `%M`: Represents the month.
- `%Y`: Represents the year.

## Examples

[Examples](https://github.com/ArZarLordOfMango/mtime/tree/main/examples)