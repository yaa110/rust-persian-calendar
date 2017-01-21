Rust Persian Calendar
=====================
**Rust Persian Calendar v0.1.0** provides functionality for conversion among Persian (Solar Hijri) and Gregorian calendars. A Julian calendar is used as an interface for all conversions. The crate name is `ptime` and it is compatible with the crate [time](https://crates.io/crates/time). This source code is licensed under MIT license that can be found in the LICENSE file.

## Installation
Add `ptime = "0.1"` to `dependencies` section of `Cargo.toml`:

```toml
[dependencies]
time = "0.1"
ptime = "0.1"
```

## Getting started
1- Import the crate `ptime`. Most of the time you need to import `time` crate, too.

```rust
extern crate ptime;
extern crate time;
```

2- Convert Gregorian calendar to Persian calendar.

```rust
let p_tm = ptime::from_gregorian_date(2016, 2, 21).unwrap();

assert_eq!(p_tm.tm_year, 1395);
assert_eq!(p_tm.tm_mon, 0);
assert_eq!(p_tm.tm_mday, 2);
```

3- Convert Persian calendar to Gregorian calendar.

```rust
let g_tm = ptime::from_persian_date(1395, 0, 2).unwrap().to_gregorian();

assert_eq!(g_tm.tm_year, 2016);
assert_eq!(g_tm.tm_mon, 2);
assert_eq!(g_tm.tm_mday, 21);
```

4- Get the current time.

```rust
let p_tm = ptime::now();
println!("Current time: {}", p_tm);

let p_tm_utc = ptime::now_utc();
println!("Current time at UTC: {}", p_tm_utc);
```

5- Format the time.

```rust
let p_tm = ptime::from_gregorian(time::now());
println!("{}", p_tm.to_string("yyyy-MM-dd HH:mm:ss.ns"));

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
```

For more information, please check the test files in `tests` folder.