# mtime

Unix time like library for Rust

Start point(SP) it's 00:00:00 01.01.2000.

## Methods

| name                 | return type | description                                        |
|----------------------|-------------|----------------------------------------------------|
| get_count_days()     | u64         | return number of days from SP.                     |
| get_count_years()    | u64         | return number of year from SP.                     |
| is_leap_year()       | bool        | return true if current year is leap else false. |
| get_seconds()        | String      | return current seconds from 0 to 59.            |
| get_minutes()        | String      | return current minutes from 0 to 59.            |
| get_hours()          | String      | return current hours from 0 to 23.              |
| get_day()            | String      | return current day from 1 to 28-31.             |
| get_month()          | String      | return number of current month.                 |
| get_month_name()     | String      | return name of current month.                   |
| get_year()           | String      | return current year.                            |
| get_full_date()      | String      | return full date (HH:MM:SS DD.MM.YYYY).         |
