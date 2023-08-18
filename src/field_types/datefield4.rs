/**
 * Before I think of something better
 */
pub mod datefield4 {
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
            return DateFieldCustom::dummy();
        }

        fn dummy() -> DateFieldCustom {
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

        pub fn construct_self(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: u8, millisecond: u16) -> DateFieldCustom {
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
