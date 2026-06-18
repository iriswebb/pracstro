use crate::catalog;
use pracstro::{coord, sol, time};
use std::fmt;

pub type Location = Option<(time::Angle, time::Angle)>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RefFrame {
    pub latlong: Location,
    pub date: time::Date,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PhaseView {
    Default(bool),
    Emoji(bool),
    Illumfrac,
    PhaseName,
    PhaseAngle,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CrdView {
    Equatorial,
    Horizontal(RefFrame),
    Ecliptic(time::Date),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CelestObj {
    Planet(sol::Planet),
    Moon,
    Sun,
    Star(catalog::Star),
    Crd(coord::Coord),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AngView {
    Angle,
    Latitude,
    Time,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Date(time::Date),
    Ang(time::Angle, AngView),
    Crd(coord::Coord, CrdView),
    Num(f64),
    Dist(f64),
    Phase(time::Angle, PhaseView),
    RsTime(Option<time::Date>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const EMOJIS: [&str; 8] = ["🌑", "🌘", "🌗", "🌖", "🌕", "🌔", "🌓", "🌒"];
        const SEMOJI: [&str; 8] = ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];
        const PNAMES: [&str; 8] = [
            "New",
            "Waxing Crescent",
            "First Quarter",
            "Waxing Gibbous",
            "Full",
            "Waning Gibbous",
            "Last Quarter",
            "Waning Crescent",
        ];

        fn phaseidx(ilumfrac: f64, ang: time::Angle) -> usize {
            match (ilumfrac, ang.degrees() > 90.0) {
                (0.00..0.04, _) => 0,
                (0.96..1.00, _) => 4,
                (0.46..0.54, true) => 6,
                (0.46..0.54, false) => 2,
                (0.54..0.96, true) => 5,
                (0.54..0.96, false) => 3,
                (_, true) => 7,
                (_, false) => 1,
            }
        }

        if !f.alternate() {
            use chrono::prelude::*;
            match self {
                Value::Date(d) => write!(
                    f,
                    "{}",
                    DateTime::<Local>::from(
                        DateTime::from_timestamp(d.unix() as i64, 0)
                            .expect("Failed to Format Date")
                    )
                    .format("%Y-%m-%dT%T")
                ),
                Value::Ang(p, AngView::Angle) => {
                    let (d, m, s) = p.degminsec();
                    write!(f, "{:02}°{:02}′{:02.1}″", d, m, s)
                }
                Value::Ang(p, AngView::Latitude) => {
                    let (d, m, s) = p.to_latitude().degminsec();
                    write!(f, "{:+02}°{:02}′{:02.1}″", d, m, s)
                }
                //Value::Per(p, AngView::Raw) => write!(f, "{:.5}", p.degrees()),
                Value::Ang(p, AngView::Time) => {
                    let (h, m, s) = p.clock();
                    write!(f, "{:02}h{:02}m{:02}s", h, m, s.trunc())
                }
                Value::Dist(d) => match d {
                    0.0..0.003342293561 => write!(f, "{:.1} km", d * 149597870.7),
                    20000.0.. => write!(f, "{:.2} ly", d / 63241.07708),
                    _ => write!(f, "{:.2} AU", d),
                },
                Value::Crd(c, CrdView::Equatorial) => {
                    let d = c.equatorial();
                    write!(
                        f,
                        "{} {}",
                        Value::Ang(d.0, AngView::Time),
                        Value::Ang(d.1, AngView::Latitude)
                    )
                }
                Value::Crd(c, CrdView::Horizontal(rf)) => {
                    let (lat, long) = rf.latlong.unwrap();
                    let d = c.horizon(rf.date, lat, long);
                    write!(
                        f,
                        "{} {}",
                        Value::Ang(d.0, AngView::Angle),
                        Value::Ang(d.1, AngView::Latitude)
                    )
                }
                Value::Crd(c, CrdView::Ecliptic(d)) => {
                    let d = c.ecliptic(*d);
                    write!(
                        f,
                        "{} {}",
                        Value::Ang(d.0, AngView::Angle),
                        Value::Ang(d.1, AngView::Latitude)
                    )
                }
                Value::Phase(pa, PhaseView::Default(n)) => {
                    let ilf = (1.0 - pa.cos()) / 2.0;
                    let pi = phaseidx(ilf, *pa);
                    write!(
                        f,
                        "{} {} ({:2.1}%)",
                        if *n { EMOJIS[pi] } else { SEMOJI[pi] },
                        PNAMES[pi],
                        ilf * 100.0
                    )
                }
                Value::Phase(pa, PhaseView::Emoji(true)) => {
                    write!(f, "{}", EMOJIS[phaseidx((1.0 - pa.cos()) / 2.0, *pa)])
                }
                Value::Phase(pa, PhaseView::Emoji(false)) => {
                    write!(f, "{}", SEMOJI[phaseidx((1.0 - pa.cos()) / 2.0, *pa)])
                }
                Value::Phase(pa, PhaseView::Illumfrac) => {
                    write!(f, "{:2.1}", 100.0 * (1.0 - pa.cos()) / 2.0)
                }
                Value::Phase(pa, PhaseView::PhaseName) => {
                    write!(f, "{}", PNAMES[phaseidx((1.0 - pa.cos()) / 2.0, *pa)])
                }
                Value::Phase(pa, PhaseView::PhaseAngle) => {
                    write!(f, "{}°", pa.degrees())
                }
                Value::Num(n) => write!(f, "{:0.2}", n),
                Value::RsTime(d) => {
                    if d.is_none() {
                        write!(f, "none")
                    } else {
                        write!(
                            f,
                            "{}",
                            DateTime::<Local>::from(
                                DateTime::from_timestamp(d.unwrap().unix() as i64, 0)
                                    .expect("Failed to Format Date")
                            )
                            .format("%H:%M")
                        )
                    }
                }
            }
        } else {
            match self {
                Value::Date(d) => write!(f, "{}", d.unix()),
                Value::RsTime(d) => {
                    if d.is_none() {
                        write!(f, "none")
                    } else {
                        write!(f, "{}", d.unwrap().unix())
                    }
                }
                Value::Ang(p, AngView::Angle) => {
                    write!(f, "{:.5}", p.degrees())
                }
                Value::Ang(p, AngView::Latitude) => {
                    write!(f, "{:.5}", p.to_latitude().degrees())
                }
                Value::Ang(p, AngView::Time) => {
                    write!(f, "{:.5}", p.decimal())
                }
                Value::Dist(d) => write!(f, "{}", d),
                Value::Crd(c, CrdView::Equatorial) => {
                    let d = c.equatorial();
                    write!(
                        f,
                        "[{:#}, {:#}]",
                        Value::Ang(d.0, AngView::Time),
                        Value::Ang(d.1, AngView::Latitude)
                    )
                }
                Value::Crd(c, CrdView::Horizontal(rf)) => {
                    let (lat, long) = rf.latlong.unwrap();
                    let d = c.horizon(rf.date, lat, long);
                    write!(
                        f,
                        "[{:#}, {:#}]",
                        Value::Ang(d.0, AngView::Angle),
                        Value::Ang(d.1.refract(), AngView::Latitude)
                    )
                }
                Value::Crd(c, CrdView::Ecliptic(d)) => {
                    let d = c.ecliptic(*d);
                    write!(
                        f,
                        "[{:#}, {:#}]",
                        Value::Ang(d.0, AngView::Angle),
                        Value::Ang(d.1, AngView::Latitude)
                    )
                }
                Value::Phase(pa, PhaseView::Default(h)) => {
                    let ilf = (1.0 - pa.cos()) / 2.0;
                    let pi = phaseidx(ilf, *pa);
                    write!(
                        f,
                        "\"{} {} ({:2.1}%)\"",
                        if *h { EMOJIS[pi] } else { SEMOJI[pi] },
                        PNAMES[pi],
                        ilf * 100.0
                    )
                }
                Value::Phase(pa, PhaseView::Emoji(true)) => {
                    write!(f, "\"{}\"", EMOJIS[phaseidx((1.0 - pa.cos()) / 2.0, *pa)])
                }
                Value::Phase(pa, PhaseView::Emoji(false)) => {
                    write!(f, "\"{}\"", SEMOJI[phaseidx((1.0 - pa.cos()) / 2.0, *pa)])
                }
                Value::Phase(pa, PhaseView::Illumfrac) => {
                    write!(f, "{:2.1}", 100.0 * (1.0 - pa.cos()) / 2.0)
                }
                Value::Phase(pa, PhaseView::PhaseName) => {
                    write!(f, "\"{}\"", PNAMES[phaseidx((1.0 - pa.cos()) / 2.0, *pa)])
                }
                Value::Phase(pa, PhaseView::PhaseAngle) => {
                    write!(f, "{}", pa.radians())
                }
                Value::Num(n) => write!(f, "{:0.2}", n),
            }
        }
    }
}
