use crate::value::*;
use pracstro::{self, moon, sol, time};
use pracstro::{BaseCatalogObject, CelObj};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    Equatorial,
    Horizontal,
    Ecliptic,
    Distance,
    Magnitude,
    PhaseDefault,
    PhaseName,
    PhaseEmoji,
    PhaseAngle,
    AngDia,
    IllumFrac,
    Rise,
    Set,
    AngBet(CelestObj),
}
impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Property::Equatorial => "Coordinates (RA/De)",
                Property::Horizontal => "Coordinates (Azi/Alt)",
                Property::Ecliptic => "Coordinates (Ecliptic)",
                Property::Distance => "Distance",
                Property::Magnitude => "Magnitude",
                Property::PhaseDefault => "Phase",
                Property::PhaseEmoji => "Phase Emoji",
                Property::PhaseName => "Phase Name",
                Property::PhaseAngle => "Phase Angle",
                Property::IllumFrac => "Illuminated Frac.",
                Property::AngDia => "Angular Diameter",
                Property::Rise => "Rise Time",
                Property::Set => "Set Time",
                Property::AngBet(_) => "Angle Between Object",
            }
        )
    }
}

pub fn property_of(obj: &CelestObj, q: Property, rf: &RefFrame) -> Result<Value, &'static str> {
    fn hemisphere(ll: Option<(pracstro::time::Angle, pracstro::time::Angle)>) -> bool {
        if let Some((lat, _)) = ll {
            lat.to_latitude().degrees() <= 0.0
        } else {
            false
        }
    }
    match (q, obj.clone()) {
        (Property::Equatorial, CelestObj::Planet(p)) => {
            Ok(Value::Crd(p.location(rf.date), CrdView::Equatorial))
        }
        (Property::Equatorial, CelestObj::Sun) => Ok(Value::Crd(
            sol::SUN
                .location(rf.date)
                .precess(time::Date::from_julian(2451545.0), rf.date),
            CrdView::Equatorial,
        )),
        (Property::Equatorial, CelestObj::Moon) => Ok(Value::Crd(
            moon::MOON
                .location(rf.date)
                .precess(time::Date::from_julian(2451545.0), rf.date),
            CrdView::Equatorial,
        )),
        (Property::Equatorial, CelestObj::Star(s)) => Ok(Value::Crd(
            s.loc_j2k
                .precess(time::Date::from_julian(2451545.0), rf.date),
            CrdView::Equatorial,
        )),
        (Property::Equatorial, CelestObj::Crd(s)) => Ok(Value::Crd(s, CrdView::Equatorial)),
        (Property::Horizontal, _) => {
            if rf.latlong.is_none() {
                return Err("Need to specify a lat/long with -l");
            };
            let Value::Crd(p, _) = property_of(obj, Property::Equatorial, rf)? else {
                unreachable!();
            };
            Ok(Value::Crd(p, CrdView::Horizontal(*rf)))
        }
        (Property::Ecliptic, _) => {
            let Value::Crd(p, _) = property_of(obj, Property::Equatorial, rf)? else {
                unreachable!();
            };
            Ok(Value::Crd(p, CrdView::Ecliptic(rf.date)))
        }
        (Property::Rise, _) => {
            if rf.latlong.is_none() {
                return Err("Need to specify a lat/long with -l");
            };
            let Value::Crd(p, _) = property_of(obj, Property::Equatorial, rf)? else {
                unreachable!();
            };
            match p.riseset(rf.date, rf.latlong.unwrap().0, rf.latlong.unwrap().1) {
                Some((x, _)) => Ok(Value::RsTime(Some(time::Date::from_time(rf.date, x)))),
                None => Ok(Value::RsTime(None)),
            }
        }
        (Property::Set, _) => {
            if rf.latlong.is_none() {
                return Err("Need to specify a lat/long with -l");
            };
            let Value::Crd(p, _) = property_of(obj, Property::Equatorial, rf)? else {
                unreachable!();
            };
            match p.riseset(rf.date, rf.latlong.unwrap().0, rf.latlong.unwrap().1) {
                Some((_, y)) => Ok(Value::RsTime(Some(time::Date::from_time(rf.date, y)))),
                None => Ok(Value::RsTime(None)),
            }
        }
        (Property::AngBet(c), _) => {
            let Value::Crd(p, _) = property_of(obj, Property::Equatorial, rf)? else {
                unreachable!();
            };
            let Value::Crd(o, _) = property_of(&c, Property::Equatorial, rf)? else {
                unreachable!();
            };
            Ok(Value::Ang(p.dist(o), AngView::Angle))
        }
        (Property::Distance, CelestObj::Planet(p)) => Ok(Value::Dist(p.distance(rf.date))),
        (Property::Distance, CelestObj::Sun) => Ok(Value::Dist(sol::SUN.distance(rf.date))),
        (Property::Distance, CelestObj::Moon) => Ok(Value::Dist(moon::MOON.distance(rf.date))),
        (Property::Distance, CelestObj::Star(s)) => {
            Ok(Value::Dist((1.0 / (s.pi.degrees() * 3600.0)) * 206_265.0))
        }
        (Property::Magnitude, CelestObj::Planet(p)) => Ok(Value::Num(p.brightness(rf.date))),
        (Property::Magnitude, CelestObj::Star(s)) => Ok(Value::Num(s.mag)),
        (Property::Magnitude, CelestObj::Sun) => Ok(Value::Num(sol::SUN.brightness(rf.date))),
        (Property::Magnitude, CelestObj::Moon) => Ok(Value::Num(moon::MOON.magnitude(rf.date))),
        (Property::PhaseDefault, CelestObj::Planet(p)) => Ok(Value::Phase(
            p.phaseangle(rf.date).unwrap(),
            PhaseView::Default(hemisphere(rf.latlong)),
        )),
        (Property::PhaseDefault, CelestObj::Moon) => Ok(Value::Phase(
            moon::MOON.phaseangle(rf.date),
            PhaseView::Default(hemisphere(rf.latlong)),
        )),
        (Property::PhaseEmoji, _) => {
            let Value::Phase(p, _) = property_of(obj, Property::PhaseDefault, rf)? else {
                unreachable!();
            };
            // The default emojis for people who don't specify a latitude are the northern ones
            if hemisphere(rf.latlong) {
                Ok(Value::Phase(p, PhaseView::Emoji(true)))
            } else {
                Ok(Value::Phase(p, PhaseView::Emoji(false)))
            }
        }
        (Property::PhaseName, _) => {
            let Value::Phase(p, _) = property_of(obj, Property::PhaseDefault, rf)? else {
                unreachable!();
            };
            Ok(Value::Phase(p, PhaseView::PhaseName))
        }
        (Property::PhaseAngle, _) => {
            let Value::Phase(p, _) = property_of(obj, Property::PhaseDefault, rf)? else {
                unreachable!();
            };
            Ok(Value::Phase(p, PhaseView::PhaseAngle))
        }
        (Property::IllumFrac, _) => {
            let Value::Phase(p, _) = property_of(obj, Property::PhaseDefault, rf)? else {
                unreachable!();
            };
            Ok(Value::Phase(p, PhaseView::Illumfrac))
        }
        (Property::AngDia, CelestObj::Planet(p)) => {
            Ok(Value::Ang(p.angdia(rf.date).unwrap(), AngView::Angle))
        }
        (Property::AngDia, CelestObj::Sun) => Ok(Value::Ang(
            sol::SUN.angdia(rf.date).unwrap(),
            AngView::Angle,
        )),
        (Property::AngDia, CelestObj::Moon) => {
            Ok(Value::Ang(moon::MOON.angdia(rf.date), AngView::Angle))
        }
        (Property::PhaseDefault, _) => Err("Can't get phase of a star"),
        (_, CelestObj::Crd(_)) => Err("Can't get that property for a raw coordinate"),
        (Property::AngDia, CelestObj::Star(_)) => Err("Angular diameter of star not known"),
    }
}

/// An object and a CSV list of properties. The return stack is these properties.
pub fn run(
    object: &CelestObj,
    proplist: &[Property],
    latlong: Location,
    date: time::Date,
) -> Result<Vec<Value>, &'static str> {
    Ok(proplist
        .iter()
        .map(|prop| {
            property_of(object, prop.clone(), &RefFrame { latlong, date })
                .unwrap_or_else(|e| panic!("Error on property {prop}: {e}"))
        })
        .collect())
}
