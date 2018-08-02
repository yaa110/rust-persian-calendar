extern crate ptime;
use ptime::Duration;

#[test]
fn leap_years() {
    let leap_years = [1325, 1329, 1333, 1337, 1370, 1375, 1379, 1383, 1387, 1391, 1395, 1399];
    let mut tm = ptime::empty_tm();
    for year in leap_years.iter() {
        tm.tm_year = *year;
        assert_eq!(tm.is_leap(), true);
    }
}

#[test]
fn non_leap_years() {
    let non_leap_years = [1324, 1330, 1332, 1335, 1371, 1374, 1380, 1381, 1386, 1390, 1394, 1400];
    let mut tm = ptime::empty_tm();
    for year in non_leap_years.iter() {
        tm.tm_year = *year;
        assert_eq!(tm.is_leap(), false);
    }
}

#[test]
fn operators() {
    let p_tm1 = ptime::from_persian_date(1395, 0, 1).unwrap();
    let p_tm2 = ptime::from_gregorian_date(2016, 2, 21).unwrap();
    assert_eq!(p_tm2 - p_tm1, Duration::seconds(24 * 3600));
    assert_eq!(p_tm2 > p_tm1, true);
    assert_eq!(p_tm2 < p_tm1, false);
    assert_eq!(p_tm2 == p_tm1, false);
}

#[test]
fn format() {
    let p_tm = ptime::from_gregorian_date(2016, 2, 21).unwrap();
    assert_eq!(format!("{}", p_tm), "1395-01-02T00:00:00.0");
}
