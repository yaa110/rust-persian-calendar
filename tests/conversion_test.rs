extern crate ptime;
extern crate time;

// year, month, day of month, weekday, day of year
static PERSIAN_GREGORIAN: [[[i32; 5]; 2]; 9] = [
    [
        [1383, 3, 15, 2, 107],
        [2004, 6, 5, 1, 186],
    ],
    [
        [1394, 11, 9, 1, 344],
        [2016, 1, 28, 0, 58],
    ],
    [
        [1394, 9, 11, 6, 286],
        [2016, 0, 1, 5, 0],
    ],
    [
        [1394, 11, 11, 3, 346],
        [2016, 2, 1, 2, 60],
    ],
    [
        [1394, 11, 29, 0, 364],
        [2016, 2, 19, 6, 78],
    ],
    [
        [1395, 0, 1, 1, 0],
        [2016, 2, 20, 0, 79],
    ],
    [
        [1395, 0, 2, 2, 1],
        [2016, 2, 21, 1, 80],
    ],
    [
        [1395, 0, 3, 3, 2],
        [2016, 2, 22, 2, 81],
    ],
    [
        [1395, 9, 11, 0, 286],
        [2016, 11, 31, 6, 365],
    ],
];

#[test]
fn gregorian_to_persian() {
    for pair in PERSIAN_GREGORIAN.iter() {
        let p_tm = ptime::from_gregorian(time::Tm{
            tm_year: pair[1][0] - 1900,
            tm_mon: pair[1][1],
            tm_mday: pair[1][2],
            tm_hour: 10,
            tm_min: 30,
            tm_sec: 50,
            tm_nsec: 121,
            tm_wday: pair[1][3],
            tm_yday: pair[1][4],
            tm_isdst: 0,
            tm_utcoff: 0,
        });

        assert_eq!(p_tm.tm_year, pair[0][0]);
        assert_eq!(p_tm.tm_mon, pair[0][1]);
        assert_eq!(p_tm.tm_mday, pair[0][2]);
        assert_eq!(p_tm.tm_wday, pair[0][3]);
        assert_eq!(p_tm.tm_yday, pair[0][4]);
        assert_eq!(p_tm.tm_hour, 10);
        assert_eq!(p_tm.tm_min, 30);
        assert_eq!(p_tm.tm_sec, 50);
        assert_eq!(p_tm.tm_nsec, 121);
    }
}

#[test]
fn persian_to_gregorian() {
    for pair in PERSIAN_GREGORIAN.iter() {
        let g_tm = ptime::Tm{
            tm_year: pair[0][0],
            tm_mon: pair[0][1],
            tm_mday: pair[0][2],
            tm_hour: 10,
            tm_min: 30,
            tm_sec: 50,
            tm_nsec: 121,
            tm_wday: pair[0][3],
            tm_yday: pair[0][4],
            tm_isdst: 0,
            tm_utcoff: 0,
        }.to_gregorian();

        assert_eq!(g_tm.tm_year, pair[1][0] - 1900);
        assert_eq!(g_tm.tm_mon, pair[1][1]);
        assert_eq!(g_tm.tm_mday, pair[1][2]);
        assert_eq!(g_tm.tm_wday, pair[1][3]);
        assert_eq!(g_tm.tm_yday, pair[1][4]);
        assert_eq!(g_tm.tm_hour, 10);
        assert_eq!(g_tm.tm_min, 30);
        assert_eq!(g_tm.tm_sec, 50);
        assert_eq!(g_tm.tm_nsec, 121);
    }
}

#[test]
fn gregorian_components_to_persian() {
    for pair in PERSIAN_GREGORIAN.iter() {
        match ptime::from_gregorian_components(
            pair[1][0],
            pair[1][1],
            pair[1][2],
            10,
            30,
            50,
            121,
        ) {
            Some(p_tm) => {
                assert_eq!(p_tm.tm_year, pair[0][0]);
                assert_eq!(p_tm.tm_mon, pair[0][1]);
                assert_eq!(p_tm.tm_mday, pair[0][2]);
                assert_eq!(p_tm.tm_wday, pair[0][3]);
                assert_eq!(p_tm.tm_yday, pair[0][4]);
                assert_eq!(p_tm.tm_hour, 10);
                assert_eq!(p_tm.tm_min, 30);
                assert_eq!(p_tm.tm_sec, 50);
                assert_eq!(p_tm.tm_nsec, 121);
            },
            None => panic!("invalid input validation of {:?}", pair[1])
        };
    }
}

#[test]
fn persian_components_to_gregorian() {
    for pair in PERSIAN_GREGORIAN.iter() {
        match ptime::from_persian_components(
            pair[0][0],
            pair[0][1],
            pair[0][2],
            10,
            30,
            50,
            121,
        ) {
            Some(p_tm) => {
                let g_tm = p_tm.to_gregorian();

                assert_eq!(g_tm.tm_year, pair[1][0] - 1900);
                assert_eq!(g_tm.tm_mon, pair[1][1]);
                assert_eq!(g_tm.tm_mday, pair[1][2]);
                assert_eq!(g_tm.tm_wday, pair[1][3]);
                assert_eq!(g_tm.tm_yday, pair[1][4]);
                assert_eq!(g_tm.tm_hour, 10);
                assert_eq!(g_tm.tm_min, 30);
                assert_eq!(g_tm.tm_sec, 50);
                assert_eq!(g_tm.tm_nsec, 121);
            },
            None => panic!("invalid input validation of {:?}", pair[1])
        }
    }
}

#[test]
fn compare_now_utc() {
    let mut p_tm = ptime::now_utc();
    let g_tm = time::now_utc();
    p_tm.tm_hour = g_tm.tm_hour;
    p_tm.tm_min = g_tm.tm_min;
    p_tm.tm_sec = g_tm.tm_sec;
    p_tm.tm_nsec = g_tm.tm_nsec;
    assert_eq!(p_tm.to_timespec(), g_tm.to_timespec());
}
