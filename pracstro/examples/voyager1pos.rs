use pracstro::{probe::gen_voyager_1, time::Angle, *};

fn main() {
    let sd = time::Date::from_calendar(1977, 10, 01, Angle::ZERO);
    let fd = time::Date::from_calendar(2026, 07, 31, Angle::ZERO);
    let voyager1 = gen_voyager_1();

    let mut d = sd.julian();
    loop {
        let date = time::Date::from_julian(d);

        let (c, dist) = voyager1
            .position(date)
            .solar_to_geo(date)
            .polar_referenceobject_relative();

        let cal = date.calendar();

        println!(
            "{}-{}-{}, {}, {}, {},",
            cal.0,
            cal.1,
            cal.2,
            c.equatorial().0.degrees(),
            c.equatorial().1.to_latitude().degrees(),
            dist,
        );

        if date > fd {
            break;
        }
        d += 1.0;
    }
}
