/**
 * Before I think of something better
 */
pub mod datefield4 {
    const MAX_YEAR: u32 = 268435455;
    const MAX_MONTH: u8 = 12;
    const MAX_DAY: u8 = 31;
    const MAX_HOUR: u8 = 24;
    const MAX_MINUTE: u8 = 59;
    const MAX_SECONDS: u8 = 60;
    const MAX_MILLISECONDS: u16 = 999;

    const DAYS_NONLEAP: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_LEAP: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    #[derive(Clone, PartialEq)]
    pub struct DateFieldCustom {
        year: u32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    }
    impl DateFieldCustom {
        /**
         * Single supported format as of now
         * all numbers
         * Year-Month-DAY HOUR:MINUTE:SECOND.MILLISECOND
         */
        pub fn new(str: String) -> DateFieldCustom {
            let separator = "-";
            let split: Vec<&str> = str.split(separator).collect();
            let year: u32 = split[0].parse().unwrap();

            let mut month = 0;
            let mut day = 0;
            let mut hour = 0;
            let mut minute = 0;
            let mut second = 0;
            let mut millisecond = 0;

            if split.len() > 1 {
                month = split[1].parse().unwrap();
            }

            if split.len() < 3 {
                return DateFieldCustom::construct_self(year, month, day, hour, minute, second, millisecond);
            }

            let split = split[2].clone();
            let split: Vec<&str> = split.split(" ").collect();
            day = split[0].parse().unwrap();

            if split.len() == 1 {
                return DateFieldCustom::construct_self(year, month, day, hour, minute, second, millisecond);
            }

            let split = split[1].clone();
            let split: Vec<&str> = split.split(":").collect();

            hour = split[0].parse().unwrap();
            minute = split[1].parse().unwrap();

            if split.len() < 3 {
                return DateFieldCustom::construct_self(year, month, day, hour, minute, second, millisecond);
            }
            let split = split[2].clone();
            let split: Vec<&str> = split.split(".").collect();

            second = split[0].parse().unwrap();
            if split.len() > 1 {
                millisecond = split[1].parse().unwrap();
            }

            return DateFieldCustom::construct_self(year, month, day, hour, minute, second, millisecond);
        }

        pub fn from_str(s: &str) -> DateFieldCustom {
            return DateFieldCustom::new(String::from(s));
        }
        pub fn dummy() -> DateFieldCustom {
            let year: u32 = 2023;
            let month: u8 = 8;
            let day: u8 = 18;
            let hour: u8 = 21;
            let minute: u8 = 48;
            let second: u8 = 10;
            let millisecond: u16 = 886;
            return DateFieldCustom {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            };
        }

        pub fn construct_self(
            year: u32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            millisecond: u16,
        ) -> DateFieldCustom {
            return DateFieldCustom {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            };
        }

        fn check_print_pass(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: u8, millisecond: u16) -> bool {
            if year > MAX_YEAR {
                println!("year not supported");
                return false;
            }
            if month > MAX_MONTH {
                println!("month not supported");
                return false;
            }
            if day > MAX_DAY {
                println!("day not supported");
                return false;
            }
            if hour > MAX_HOUR {
                println!("hour not supported");
                return false;
            }
            if minute > MAX_MINUTE {
                println!("minute not supported");
                return false;
            }

            if second > MAX_SECONDS {
                println!("second not supported");
                return false;
            }

            if millisecond > MAX_MILLISECONDS {
                println!("millisecond not supported");
                return false;
            }
            return true;
        }

        pub fn construct_self_checked(
            year: u32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            millisecond: u16,
        ) -> Option<DateFieldCustom> {
            let cppass = Self::check_print_pass(year, month, day, hour, minute, second, millisecond);

            if cppass == false {
                return None;
            }

            let is_a_leap_year = (year % 4 == 0) & (year % 100 != 0);

            if is_a_leap_year {
                let days = DAYS_LEAP[(month - 1) as usize];
                if day > days {
                    println!("more days than is in month");
                    return None;
                }
            } else {
                let days = DAYS_NONLEAP[(month - 1) as usize];
                if day > days {
                    println!("more days than is in month");
                    return None;
                }
            }

            return Some(DateFieldCustom {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            });
        }

        pub fn year(&self) -> u32 {
            return self.year;
        }
        pub fn month(&self) -> u8 {
            return self.month;
        }

        pub fn day(&self) -> u8 {
            return self.day;
        }

        pub fn hour(&self) -> u8 {
            return self.hour;
        }
        pub fn minute(&self) -> u8 {
            return self.minute;
        }

        pub fn into_datestring(&self) -> String {
            return format!("{}-{}-{}", self.year, self.month, self.day);
        }
        pub fn into_datetime_string(&self) -> String {
            return format!("{}-{}-{} {}:{}", self.year, self.month, self.day, self.hour, self.minute);
        }
        pub fn into_fulldatetime_string(&self) -> String {
            return format!(
                "{}-{}-{} {}:{}:{}.{}",
                self.year, self.month, self.day, self.hour, self.minute, self.second, self.millisecond
            );
        }
        pub fn new2(str: &str) {}

        pub fn serialize(&self) -> Vec<u8> {
            let serial: u64 = ((self.year as u64) << 36)
                + ((self.month as u64) << 32)
                + ((self.day as u64) << 27)
                + ((self.hour as u64) << 22)
                + ((self.minute as u64) << 16)
                + ((self.second as u64) << 10)
                + (self.millisecond as u64);

            return serial.to_be_bytes().to_vec();
        }

        pub fn deserialize(v: Vec<u8>) -> DateFieldCustom {
            let mut x: [u8; 8] = [0; 8];
            for i in 0..8 {
                x[i] = v[i];
            }
            let dese = u64::from_be_bytes(x);

            let year = dese >> 36;
            let month = (dese >> 32) - (year << 4);
            let day = (dese >> 27) - (month << 5) - (year << 9);

            let hour = (dese >> 22) - (day << 5) - (month << 10) - (year << 14);
            let minute = (dese >> 16) - (hour << 6) - (day << 11) - (month << 16) - (year << 20);
            let second = (dese >> 10) - (minute << 6) - (hour << 12) - (day << 17) - (month << 22) - (year << 26);

            let milisecond = dese - (second << 10) - (minute << 16) - (hour << 22) - (day << 27) - (month << 32) - (year << 36);

            return DateFieldCustom {
                year: year.try_into().unwrap(),
                month: month.try_into().unwrap(),
                day: day.try_into().unwrap(),
                hour: hour.try_into().unwrap(),
                minute: minute.try_into().unwrap(),
                second: second.try_into().unwrap(),
                millisecond: milisecond.try_into().unwrap(),
            };
        }

        pub fn test_sede() {
            let df = DateFieldCustom::dummy();
            let seri = df.serialize();

            let df2 = DateFieldCustom::deserialize(seri);

            let s1 = df.into_fulldatetime_string();

            let s2 = df2.into_fulldatetime_string();

            println!("{}", s1);
            println!("{}", s2);
        }
    }
}