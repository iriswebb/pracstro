use crate::value::*;
use pracstro::{coord, time};

#[derive(Clone, Debug, PartialEq)]
pub struct Star {
    pub loc_j2k: coord::Coord,
    pub mag: f64,
    pub pi: time::Angle,
    pub pm_ra: time::Angle,
    pub pm_dec: time::Angle,
}

/// Creates the catalog as a hash table
///
/// This operation takes about 500 µs on my machine
pub fn read() -> std::collections::HashMap<&'static str, CelestObj> {
    use pracstro::sol;

    let mut cat = std::collections::HashMap::from([
        ("sun", CelestObj::Sun),
        ("mercury", CelestObj::Planet(sol::MERCURY)),
        ("venus", CelestObj::Planet(sol::VENUS)),
        ("moon", CelestObj::Moon),
        ("mars", CelestObj::Planet(sol::MARS)),
        ("jupiter", CelestObj::Planet(sol::JUPITER)),
        ("saturn", CelestObj::Planet(sol::SATURN)),
        ("uranus", CelestObj::Planet(sol::URANUS)),
        ("neptune", CelestObj::Planet(sol::NEPTUNE)),
        ("pluto", CelestObj::Planet(sol::PLUTO)),
    ]);

    include_str!("dat/stars.csv")
        .lines()
        .skip(1)
        .map(|star| {
            let p: Vec<&str> = star.split(',').collect();
            (
                p[0],
                CelestObj::Star(Star {
                    loc_j2k: coord::Coord::from_equatorial(
                        time::Angle::from_degrees(p[1].parse().unwrap()),
                        time::Angle::from_degrees(p[2].parse().unwrap()),
                    ),
                    mag: p[3].parse().unwrap(),
                    pi: time::Angle::from_degrees(p[4].parse::<f64>().unwrap() / 3_600_000.0),
                    pm_ra: time::Angle::from_degrees(p[5].parse::<f64>().unwrap() / 3_600_000.0),
                    pm_dec: time::Angle::from_degrees(p[6].parse::<f64>().unwrap() / 3_600_000.0),
                }),
            )
        })
        .for_each(|(n, s)| {
            cat.insert(n, s);
        });

    cat
}
