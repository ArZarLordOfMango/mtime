#![allow(dead_code)]

// Start Date: 00:00:00 1 January 2000 🥭
#[derive(Debug, Clone)]
pub struct MTime {
    pub time: u64,
}

#[derive(Debug, Clone)]
pub struct DateBuilder {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub second: String,
}

impl DateBuilder {
    pub fn new() -> DateBuilder {
        DateBuilder {
            year: "2000".to_string(),
            month: "01".to_string(),
            day: "01".to_string(),
            hour: "00".to_string(),
            minute: "00".to_string(),
            second: "00".to_string(),
        }
    }
    pub fn year(mut self, value: &str) -> Self {
        self.year = value.to_string();
        self
    }
    pub fn month(mut self, value: &str) -> Self {
        self.month = value.to_string();
        self
    }
    pub fn day(mut self, value: &str) -> Self {
        self.day = value.to_string();
        self
    }
    pub fn hour(mut self, value: &str) -> Self {
        self.hour = value.to_string();
        self
    }
    pub fn minute(mut self, value: &str) -> Self {
        self.minute = value.to_string();
        self
    }
    pub fn second(mut self, value: &str) -> Self {
        self.second = value.to_string();
        self
    }
    #[allow(unused_mut)]
    pub fn build(mut self) -> Result<Date, String> {
        match self.year.parse::<u64>() {
            Ok(_) => {},
            Err(_) => return Err("DateBuildError: Year must be integer".to_string()),
        }
        match self.month.parse::<u64>() {
            Ok(_) => {},
            Err(_) => return Err("DateBuildError: Month must be integer".to_string()),
        }
        match self.day.parse::<u64>() {
            Ok(_) => {},
            Err(_) => return Err("DateBuildError: Day must be integer".to_string()),
        }
        let year_int: u64 = self.year.parse().unwrap();
        let month_int: u64 = self.month.parse().unwrap();
        let day_int: u64 = self.day.parse().unwrap();

        if year_int <= 1999 {
            return Err("DateBuildError: Year must be >= 2000.".to_string());
        }
        if month_int > 12 {
            return Err("DateBuildError: Month must be <= 12.".to_string());
        }
        
        let mut month_array: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if year_int % 4 == 0 {month_array[1]+1} else {month_array[1]} ;

        if day_int > month_array[(month_int-1) as usize] {
            return Err(format!("DateBuildError: In the current month, the maximum value of day {}.", month_array[(month_int-1) as usize]));
        }

        if self.hour > 59.to_string() {
            return Err("DateBuildError: Hour must be <= 59.".to_string());
        }

        if self.minute > 59.to_string() {
            return Err("DateBuildError: Minute must be <= 59.".to_string());
        }

        if self.second > 59.to_string() {
            return Err("DateBuildError: Second must be <= 59.".to_string());
        }
        
        Ok(Date {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Date {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub second: String,
}

impl Date {
    pub fn builder() -> DateBuilder {
        DateBuilder {
            year: "2000".to_string(),
            month: "01".to_string(),
            day: "01".to_string(),
            hour: "00".to_string(),
            minute: "00".to_string(),
            second: "00".to_string(),
        }
    }

    pub fn from_string(&self, date_string: &str) -> Result<Date, String> {
        let splitted_text: Vec<&str> = date_string.split("-").collect::<Vec<&str>>();
        if splitted_text.len() != 6 {
            return Err("DateBuildError: String must have 6 arguments".to_string());
        }
            Date::builder()
            .year(splitted_text[5])
            .month(splitted_text[4])
            .day(splitted_text[3])
            .hour(splitted_text[0])
            .minute(splitted_text[1])
            .second(splitted_text[2])
            .build()
    }

    pub fn format(&self, to_format: &str) -> String {
        let formatted_string = to_format
            .replace("%s", &self.second.to_string())
            .replace("%m", &self.minute.to_string())
            .replace("%h", &self.hour.to_string())
            .replace("%D", &self.day.to_string())
            .replace("%M", &self.month.to_string())
            .replace("%Y", &self.year.to_string());
        formatted_string
    }
}

impl MTime {
    pub fn new(time: u64) -> MTime {
        MTime { time }
    }

    pub fn get_count_days(&self) -> u64 {
        self.time / 86400
    }

    pub fn get_count_years(&self) -> u64 {
        self.get_count_days() / 365
    }

    pub fn is_leap_year(&self) -> bool {
        self.get_count_days() % (365 * 3 + 366) <= 366
    }

    pub fn get_seconds(&self) -> u8 {
        let seconds = self.time % 60;
        seconds as u8
    }

    pub fn get_visual_seconds(&self) -> String {
        format!("{:02}", self.get_seconds())
    }

    pub fn get_minutes(&self) -> u8 {
        let minutes = (self.time / 60) % 60;
        minutes as u8
    }

    pub fn get_visual_minutes(&self) -> String {
        format!("{:02}", self.get_minutes())
    }

    pub fn get_hours(&self) -> u8 {
        let hours = (self.time / 3600) % 24;
        hours as u8
    }

    pub fn get_visual_hours(&self) -> String {
        format!("{:02}", self.get_hours())
    }

    pub fn get_day(&self) -> u8 {
        let mut days = self.get_count_days();
        let mut month_array: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut i: usize = 0;
        while month_array[i] < days {
            if self.is_leap_year() && i == 0 {
                month_array[1] += 1;
            }
            days -= month_array[i];
            i += 1;
            if i == 12 {
                i = 0;
            }
        }
        if days == 0 {
            (days + 1) as u8
        } else {
            days as u8
        }
    }

    pub fn get_visual_day(&self) -> String {
        format!("{:02}", self.get_day())
    }

    pub fn get_month(&self) -> u8 {
        let mut days = self.get_count_days();
        let mut month_array: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut i: usize = 0;
        while month_array[i] < days {
            if self.is_leap_year() && i == 0 {
                month_array[1] += 1;
            }
            days -= month_array[i];
            i += 1;
            if i == 12 {
                i = 0;
            }
        }
        (i + 1) as u8
    }

    pub fn get_visual_month(&self) -> String {
        format!("{:02}", self.get_month())
    }

    pub fn get_month_name(&self) -> String {
        const MONTHS: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let mut days = self.get_count_days();
        let mut month_array: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut i: usize = 0;
        while month_array[i] < days {
            if self.is_leap_year() && i == 0 {
                month_array[1] += 1;
            }
            days -= month_array[i];
            i += 1;
            if i == 12 {
                i = 0;
            }
        }
        MONTHS[i].to_string()
    }

    pub fn get_year(&self) -> u64 {
        2000 + self.get_count_years()
    }

    pub fn get_visual_year(&self) -> String {
        (2000 + self.get_count_years()).to_string()
    }

    pub fn get_date(&self) -> Date {
        Date {
            year: self.get_visual_year(),
            month: self.get_visual_month(),
            day: self.get_visual_day(),
            hour: self.get_visual_hours(),
            minute: self.get_visual_minutes(),
            second: self.get_visual_seconds(),
        }
    }
}