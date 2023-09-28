#![allow(dead_code)]

// Start Date: 00:00:00 1 January 2000 🥭
#[derive(Debug)]
pub struct MTime {
    pub time: u64,
}

#[derive(Debug)]
pub struct Date {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub second: String,
}

impl Date {
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
        MTime {
            time
        }
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
        let minutes = (self.time / 60 ) % 60; 
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
        while month_array[i]<days {
            if self.is_leap_year() && i == 0 {
                month_array[1] += 1;
            }
            days -= month_array[i];
            i += 1;
            if i == 12 {
                i = 0;
            }
        }
        if days==0 {(days+1) as u8} else {days as u8}
    }
    
    pub fn get_visual_day(&self) -> String {
        format!("{:02}", self.get_day())
    }

    pub fn get_month(&self) -> u8 {
        let mut days = self.get_count_days();
        let mut month_array: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut i: usize = 0;
        while month_array[i]< days {
            if self.is_leap_year() && i == 0 {
                month_array[1] += 1;
            }
            days -= month_array[i];
            i += 1;
            if i == 12 {
                i = 0;
            }
        }
        (i+1) as u8
    }

    pub fn get_visual_month(&self) -> String {
        format!("{:02}", self.get_month())
    }

    pub fn get_month_name(&self) -> &str {
        const MONTHS: [&str; 12] = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];
        let mut days = self.get_count_days();
        let mut month_array: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut i: usize = 0;
        while month_array[i]< days {
            if self.is_leap_year() && i == 0 {
                month_array[1] += 1;
            }
            days -= month_array[i];
            i += 1;
            if i == 12 {
                i = 0;
            }
        }
        MONTHS[i]
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
