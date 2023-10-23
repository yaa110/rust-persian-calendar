// In the name of Allah

//! Provides functionality for conversion among Persian (Solar Hijri) and Gregorian calendars.
//! A Julian calendar is used as an interface for all conversions.
//! The crate name is ptime and it is compatible with the crate [time](https://crates.io/crates/time).
//! This source code is licensed under MIT license that can be found in the LICENSE file.
//!
//! # Example
//! ```
//! extern crate ptime;
//!
//! fn main() {
//!     let p_tm = ptime::from_gregorian_date(2016, 2, 21).unwrap();
//!
//!     assert_eq!(p_tm.tm_year, 1395);
//!     assert_eq!(p_tm.tm_mon, 0);
//!     assert_eq!(p_tm.tm_mday, 2);
//! }
//! ```

extern crate time;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

/// Represents the components of a moment in time in Persian Calendar.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
pub struct Tm {
    /// The same as `tm_sec` of `time::Tm`
    pub tm_sec: i32,

    /// The same as `tm_min` of `time::Tm`
    pub tm_min: i32,

    /// The same as `tm_hour` of `time::Tm`
    pub tm_hour: i32,

    /// MonthDay - [1, 31]
    pub tm_mday: i32,

    /// Month since Farvardin - [0, 11]
    pub tm_mon: i32,

    /// Year
    pub tm_year: i32,

    /// Weekday since Shanbe - [0, 6]. 0 = Shanbeh, ..., 6 = Jomeh.
    pub tm_wday: i32,

    /// YearDay since Farvardin 1 - [0, 365]
    pub tm_yday: i32,

    /// The same as `tm_isdst` of `time::Tm`
    pub tm_isdst: i32,

    /// The same as `tm_utcoff` of `time::Tm`
    pub tm_utcoff: i32,

    /// The same as `tm_nsec` of `time::Tm`
    pub tm_nsec: i32,
}

impl fmt::Display for Tm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string("yyyy-MM-ddTHH:mm:ss.ns"))
    }
}

impl Add<time::Duration> for Tm {
    type Output = Tm;

    // FIXME: The timezone of `self` is different from resulting time
    fn add(self, other: time::Duration) -> Tm {
        at_utc(self.to_timespec() + other)
    }
}

impl Sub<time::Duration> for Tm {
    type Output = Tm;

    // FIXME: The timezone of `self` is different from resulting time
    fn sub(self, other: time::Duration) -> Tm {
        at_utc(self.to_timespec() - other)
    }
}

impl Sub<Tm> for Tm {
    type Output = time::Duration;

    fn sub(self, other: Tm) -> time::Duration {
        self.to_timespec() - other.to_timespec()
    }
}

impl Sub<time::Tm> for Tm {
    type Output = time::Duration;

    fn sub(self, other: time::Tm) -> time::Duration {
        self.to_timespec() - other.to_timespec()
    }
}

impl PartialOrd for Tm {
    fn partial_cmp(&self, other: &Tm) -> Option<Ordering> {
        self.to_timespec().partial_cmp(&other.to_timespec())
    }
}

impl Ord for Tm {
    fn cmp(&self, other: &Tm) -> Ordering {
        self.to_timespec().cmp(&other.to_timespec())
    }
}

impl Tm {
    /// Converts Persian calendar to Gregorian calendar
    pub fn to_gregorian(&self) -> time::Tm {
        let jdn = fixed_get_jdn(self);

        let mut seconds = jdn as i64 * 86_400;
        seconds += self.tm_hour as i64 * 3600;
        seconds += self.tm_min as i64 * 60;
        seconds += self.tm_sec as i64;
        let ts = time::Timespec { sec: seconds, nsec: self.tm_nsec};
        let time_tm = time::at_utc(ts);

        time_tm
    }

    /// Returns the number of seconds since January 1, 1970 UTC
    pub fn to_timespec(&self) -> time::Timespec {
        self.to_gregorian().to_timespec()
    }

    /// Returns true if the year is a leap year
    pub fn is_leap(&self) -> bool {
        is_persian_leap(self.tm_year)
    }

    /// Convert time to the local timezone
    pub fn to_local(&self) -> Tm {
        match self.tm_utcoff {
            0 => at(self.to_timespec()),
            _ => *self,
        }
    }

    /// Convert time to the UTC
    pub fn to_utc(&self) -> Tm {
        match self.tm_utcoff {
            0 => *self,
            _ => at_utc(self.to_timespec()),
        }
    }

    /// Returns the formatted representation of time
    ///     yyyy, yyy, y     year (e.g. 1394)
    ///     yy               2-digits representation of year (e.g. 94)
    ///     MMM              the Persian name of month (e.g. فروردین)
    ///     MM               2-digits representation of month (e.g. 01)
    ///     M                month (e.g. 1)
    ///     DD               day of year (starting from 1)
    ///     D                day of year (starting from 0)
    ///     dd               2-digits representation of day (e.g. 01)
    ///     d                day (e.g. 1)
    ///     E                the Persian name of weekday (e.g. شنبه)
    ///     e                the Persian short name of weekday (e.g. ش)
    ///     A                the Persian name of 12-Hour marker (e.g. قبل از ظهر)
    ///     a                the Persian short name of 12-Hour marker (e.g. ق.ظ)
    ///     HH               2-digits representation of hour [00-23]
    ///     H                hour [0-23]
    ///     kk               2-digits representation of hour [01-24]
    ///     k                hour [1-24]
    ///     hh               2-digits representation of hour [01-12]
    ///     h                hour [1-12]
    ///     KK               2-digits representation of hour [00-11]
    ///     K                hour [0-11]
    ///     mm               2-digits representation of minute [00-59]
    ///     m                minute [0-59]
    ///     ss               2-digits representation of seconds [00-59]
    ///     s                seconds [0-59]
    ///     ns               nanoseconds
    pub fn to_string<'a>(&'a self, format: &'a str) -> String {
        format
            .replace("yyyy", &self.tm_year.to_string())
            .replace("yyy", &self.tm_year.to_string())
            .replace("yy", &self.tm_year.to_string()[2..])
            .replace("y", &self.tm_year.to_string())
            .replace(
                "MMM",
                match self.tm_mon {
                    0 => "فروردین",
                    1 => "اردیبهشت",
                    2 => "خرداد",
                    3 => "تیر",
                    4 => "مرداد",
                    5 => "شهریور",
                    6 => "مهر",
                    7 => "آبان",
                    8 => "آذر",
                    9 => "دی",
                    10 => "بهمن",
                    11 => "اسفند",
                    _ => panic!("invalid month value of {}", self.tm_mon),
                },
            )
            .replace("MM", &format!("{:02}", self.tm_mon + 1))
            .replace("M", &format!("{}", self.tm_mon + 1))
            .replace("DD", &format!("{}", self.tm_yday + 1))
            .replace("D", &self.tm_yday.to_string())
            .replace("dd", &format!("{:02}", self.tm_mday))
            .replace("d", &self.tm_mday.to_string())
            .replace(
                "E",
                match self.tm_wday {
                    0 => "شنبه",
                    1 => "یک‌شنبه",
                    2 => "دوشنبه",
                    3 => "سه‌شنبه",
                    4 => "چهارشنبه",
                    5 => "پنج‌شنبه",
                    6 => "جمعه",
                    _ => panic!("invalid weekday value of {}", self.tm_wday),
                },
            )
            .replace(
                "e",
                match self.tm_wday {
                    0 => "ش",
                    1 => "ی",
                    2 => "د",
                    3 => "س",
                    4 => "چ",
                    5 => "پ",
                    6 => "ج",
                    _ => panic!("invalid weekday value of {}", self.tm_wday),
                },
            )
            .replace(
                "A",
                if self.tm_hour < 12 {
                    "قبل از ظهر"
                } else {
                    "بعد از ظهر"
                },
            )
            .replace("a", if self.tm_hour < 12 { "ق.ظ" } else { "ب.ظ" })
            .replace("HH", &format!("{:02}", self.tm_hour))
            .replace("H", &self.tm_hour.to_string())
            .replace("kk", &format!("{:02}", self.tm_hour + 1))
            .replace("k", &format!("{}", self.tm_hour + 1))
            .replace(
                "hh",
                &format!(
                    "{:02}",
                    if self.tm_hour > 11 {
                        self.tm_hour - 12
                    } else {
                        self.tm_hour
                    } + 1
                ),
            )
            .replace(
                "h",
                &format!(
                    "{}",
                    if self.tm_hour > 11 {
                        self.tm_hour - 12
                    } else {
                        self.tm_hour
                    } + 1
                ),
            )
            .replace(
                "KK",
                &format!(
                    "{:02}",
                    if self.tm_hour > 11 {
                        self.tm_hour - 12
                    } else {
                        self.tm_hour
                    }
                ),
            )
            .replace(
                "K",
                &format!(
                    "{}",
                    if self.tm_hour > 11 {
                        self.tm_hour - 12
                    } else {
                        self.tm_hour
                    }
                ),
            )
            .replace("mm", &format!("{:02}", self.tm_min))
            .replace("m", &self.tm_min.to_string())
            .replace("ns", &self.tm_nsec.to_string())
            .replace("ss", &format!("{:02}", self.tm_sec))
            .replace("s", &self.tm_sec.to_string())
    }
}

/// Creates an empty `ptime::Tm`
pub fn empty_tm() -> Tm {
    Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    }
}

/// Converts Gregorian calendar to Persian calendar
pub fn from_gregorian(gregorian_tm: time::Tm) -> Tm {
    let mut year: i32;
    let gy = gregorian_tm.tm_year + 1900;
    let gm = gregorian_tm.tm_mon + 1;
    let gd = gregorian_tm.tm_mday;

    let jdn: i32 = if gy > 1582 || (gy == 1582 && gm > 10) || (gy == 1582 && gm == 10 && gd > 14) {
        ((1461 * (gy + 4800 + ((gm - 14) / 12))) / 4)
            + ((367 * (gm - 2 - 12 * ((gm - 14) / 12))) / 12)
            - ((3 * ((gy + 4900 + ((gm - 14) / 12)) / 100)) / 4)
            + gd
            - 32075
    } else {
        367 * gy - ((7 * (gy + 5001 + ((gm - 9) / 7))) / 4) + ((275 * gm) / 9) + gd + 1729777
    };

    let dep = jdn - get_jdn(475, 1, 1);
    let cyc = dep / 1029983;
    let rem = dep % 1029983;
    let ycyc = if rem == 1029982 {
        2820
    } else {
        let a = rem / 366;
        (2134 * a + 2816 * (rem % 366) + 2815) / 1028522 + a + 1
    };

    year = ycyc + 2820 * cyc + 474;
    if year <= 0 {
        year -= 1;
    }

    let dy: f64 = (jdn - get_jdn(year, 1, 1) + 1) as f64;
    let month: i32 = if dy <= 186f64 {
        let mod_dy: f64 = dy / 31f64;
        mod_dy.ceil() as i32
    } else {
        let mod_dy: f64 = (dy - 6f64) / 30f64;
        mod_dy.ceil() as i32
    } - 1;
    let day = jdn - get_jdn(year, month + 1, 1) + 1;

    Tm {
        tm_sec: gregorian_tm.tm_sec,
        tm_min: gregorian_tm.tm_min,
        tm_hour: gregorian_tm.tm_hour,
        tm_mday: day,
        tm_mon: month,
        tm_year: year,
        tm_wday: get_persian_weekday(gregorian_tm.tm_wday),
        tm_yday: get_persian_yday(month, day),
        tm_isdst: gregorian_tm.tm_isdst,
        tm_utcoff: gregorian_tm.tm_utcoff,
        tm_nsec: gregorian_tm.tm_nsec,
    }
}

/// Creates a new instance of Persian time from Gregorian date
pub fn from_gregorian_date(g_year: i32, g_month: i32, g_day: i32) -> Option<Tm> {
    from_gregorian_components(g_year, g_month, g_day, 0, 0, 0, 0)
}

/// Creates a new instance of Persian time from Persian date
pub fn from_persian_date(p_year: i32, p_month: i32, p_day: i32) -> Option<Tm> {
    from_persian_components(p_year, p_month, p_day, 0, 0, 0, 0)
}

/// Creates a new instance of Persian time from Gregorian date components
pub fn from_gregorian_components(
    g_year: i32,
    g_month: i32,
    g_day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    nanosecond: i32,
) -> Option<Tm> {
    if is_time_valid(hour, minute, second, nanosecond)
        && is_gregorian_date_valid(g_year, g_month, g_day)
    {
        let tm = time::Tm {
            tm_sec: second,
            tm_min: minute,
            tm_hour: hour,
            tm_mday: g_day,
            tm_mon: g_month,
            tm_year: g_year - 1900,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_utcoff: 0,
            tm_nsec: nanosecond,
        };
        return Some(at_utc(tm.to_timespec()));
    }
    None
}

/// Creates a new instance of Persian time from Persian date components
// FIXME: Calculate the weekday without converting to Gregorian calendar
pub fn from_persian_components(
    p_year: i32,
    p_month: i32,
    p_day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    nanosecond: i32,
) -> Option<Tm> {
    if is_time_valid(hour, minute, second, nanosecond)
        && is_persian_date_valid(p_year, p_month, p_day)
    {
        let mut tm = Tm {
            tm_sec: second,
            tm_min: minute,
            tm_hour: hour,
            tm_mday: p_day,
            tm_mon: p_month,
            tm_year: p_year,
            tm_wday: 0,
            tm_yday: get_persian_yday(p_month, p_day),
            tm_isdst: 0,
            tm_utcoff: 0,
            tm_nsec: nanosecond,
        };
        tm.tm_wday = get_persian_weekday(time::at_utc(tm.to_timespec()).tm_wday);
        return Some(tm);
    }
    None
}

/// Creates a new instance of Persian time from the number of seconds since January 1, 1970 in UTC
pub fn at_utc(clock: time::Timespec) -> Tm {
    from_gregorian(time::at_utc(clock))
}

/// Creates a new instance of Persian time from the number of seconds since January 1, 1970 in the local timezone
pub fn at(clock: time::Timespec) -> Tm {
    from_gregorian(time::at(clock))
}

/// Creates a new instance of Persian time corresponding to the current time in UTC
pub fn now_utc() -> Tm {
    from_gregorian(time::now_utc())
}

/// Creates a new instance of Persian time corresponding to the current time in the local timezone
pub fn now() -> Tm {
    from_gregorian(time::now())
}

fn divider(num: i32, den: i32) -> i32 {
    if num > 0 {
        num % den
    } else {
        num - ((((num + 1) / den) - 1) * den)
    }
}

const J_UTC_EPOCH_YEAR: i32 = 1348;
const J_UTC_EPOCH_DIFF: i32 = 286;

fn fixed_get_jdn(tm: &Tm) -> i32 {
    let mut p: i32 = 0;
    let s: i32;
    let sd: i32;
    let e: i32;
    let ed: i32;
    let mut f: i32 = 1;

    if tm.tm_yday > 365 || tm.tm_yday < 0 {
        return 0;
    }

    if tm.tm_year == J_UTC_EPOCH_YEAR {
        p = tm.tm_yday - J_UTC_EPOCH_DIFF;
        return p;
    } else if tm.tm_year > J_UTC_EPOCH_YEAR {
        s = J_UTC_EPOCH_YEAR + 1;
        sd = J_UTC_EPOCH_DIFF;
        e = tm.tm_year - 1;
        ed = tm.tm_yday + 1;
    } else {
        f = -1;
        s = tm.tm_year + 1;
        sd = tm.tm_yday;
        e = J_UTC_EPOCH_YEAR - 1;
        ed = J_UTC_EPOCH_DIFF + 1;
    }

    for i in s..=e {
        let inc = if is_persian_leap(i) { 366 } else { 365 };
        p += inc;
    }

    let r = if is_persian_leap(s) {
        366 - sd - 1
    } else {
        365 - sd - 1
    };

    p += r + ed;
    p *= f;

    p
}

fn get_jdn(year: i32, month: i32, day: i32) -> i32 {
    let base = if year >= 0 { year - 474 } else { year - 473 };

    let epy = 474 + (base % 2820);

    let md = if month <= 7 {
        (month - 1) * 31
    } else {
        (month - 1) * 30 + 6
    };

    let res =
        day + md + (epy * 682 - 110) / 2816 + (epy - 1) * 365 + base / 2820 * 1029983 + 1948320;
    println!("{}", res);
    res
}

fn get_persian_weekday(wd: i32) -> i32 {
    match wd {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 6,
        6 => 0,
        _ => panic!("invalid weekday value of {}", wd),
    }
}

fn get_persian_yday(month: i32, day: i32) -> i32 {
    [
        0,   // Farvardin
        31,  // Ordibehesht
        62,  // Khordad
        93,  // Tir
        124, // Mordad
        155, // Shahrivar
        186, // Mehr
        216, // Aban
        246, // Azar
        276, // Dey
        306, // Bahman
        336, // Esfand
    ][month as usize]
        + day
        - 1
}


fn is_persian_leap(year: i32) -> bool {
    divider(25 * year + 11, 33) < 8
}

fn is_gregorian_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn is_persian_date_valid(year: i32, month: i32, day: i32) -> bool {
    if month < 0 || month > 11 {
        return false;
    }

    [
        [31, 31],
        [31, 31],
        [31, 31],
        [31, 31],
        [31, 31],
        [31, 31],
        [30, 30],
        [30, 30],
        [30, 30],
        [30, 30],
        [30, 30],
        [29, 30],
    ][month as usize][is_persian_leap(year) as usize]
        >= day
}

fn is_gregorian_date_valid(year: i32, month: i32, day: i32) -> bool {
    if month < 0 || month > 11 {
        return false;
    }

    [
        [31, 31],
        [28, 29],
        [31, 31],
        [30, 30],
        [31, 31],
        [30, 30],
        [31, 31],
        [31, 31],
        [30, 30],
        [31, 31],
        [30, 30],
        [31, 31],
    ][month as usize][is_gregorian_leap(year) as usize]
        >= day
}

fn is_time_valid(hour: i32, minute: i32, second: i32, nanosecond: i32) -> bool {
    !(hour < 0
        || hour > 23
        || minute < 0
        || minute > 59
        || second < 0
        || second > 59
        || nanosecond < 0
        || nanosecond > 999999999)
}
