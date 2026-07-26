//! Coordinate handling
//!
//! This module contains two types, [`Coord`] (the 2D type) and [`Position`] (the 3D type).
//!
//! [`Coord`] has methods to convert two and from several
//! different coordinate systems. Mainly:
//! - Equatorial (Hour Angle, Declination)
//! - Horizon (Azimuth, Altitude)
//! - Ecliptic (Beta, Lambda)
//!
//! [`Position`] can be used to transfer between reference systems
//!
//! This type also contains algorithms for converting from Cartesian (rectangular) coordinates, rise and set times, distance between angles, etc.

use crate::celobj::CelObj;
use crate::time::*;

/// Gets the mean obliquity of the ecliptic at a certain date
pub fn mean_obliquity_ecl(d: Date) -> Angle {
    let t = d.centuries();
    Angle::from_degrees(
        23.439_292 - ((46.815 * t + 0.0006 * (t * t) - 0.00181 * (t * t * t)) / 3600.0),
    )
}

/**
Pair of angles, Representing "How far up" and "How far round"

| Property          | Latitude          | Longitude           | Depends On                      | To Method              | From Method                 |
|-------------------|-------------------|---------------------|---------------------------------|------------------------|-----------------------------|
| Equatorial        | Declination (δ)   | Right Ascension (α) |                                 | [`Coord::equatorial()`]| [`Coord::from_equatorial()`]|
| Horizontal        | Altitude (a)      | Azimuth (A)         | Date, Time, Latitude, Longitude | [`Coord::horizon()`]   | [`Coord::from_horizon()`]   |
| Ecliptic          | Ecl. Latitude (β) | Ecl. Longitude (λ)  | Date[^1]                        | [`Coord::ecliptic()`]  | [`Coord::from_ecliptic()`]  |

Also see [`Position`] for 3D coordinates and reference frame transformations

Additional Methods:
* Distance between coordinates: [`Coord::dist()`]
* Rise and set times of a coordinate in the sky [`Coord::riseset()`]
* Precession [`Coord::precess()`]

[^1]: The plane of the ecliptic varies slightly with perturbations in the orbit and inclination of the earth.
*/
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Coord(Angle, Angle);
impl Coord {
    /// Right Ascension and Declination
    pub const fn equatorial(self) -> (Angle, Angle) {
        (self.0, self.1)
    }
    /// Right Ascension and Declination
    pub const fn from_equatorial(x: Angle, y: Angle) -> Self {
        Coord(x, y)
    }

    /// Azimuth and Altitude, dependent on location and time
    ///
    /// From Practical Astronomy with Your Calculator, Although similar algorithms exist in other sources
    pub fn horizon(self, date: Date, lati: Angle, longi: Angle) -> (Angle, Angle) {
        let (ra, de) = self.equatorial();
        let ha = date.time().gst(date) + longi - ra;
        let alt = Angle::asin(de.sin() * lati.sin() + de.cos() * lati.cos() * ha.cos());
        let azip = Angle::acos((de.sin() - lati.sin() * alt.sin()) / (lati.cos() * alt.cos()));
        let azi = match ha.sin() < 0.0 {
            true => azip,
            false => Angle::from_degrees(360.0 - azip.degrees()),
        };
        (azi, alt)
    }
    /// Azimuth and Altitude, dependent on location, and time
    ///
    /// From Practical Astronomy with Your Calculator, Although similar algorithms exist in other sources
    pub fn from_horizon(azi: Angle, alt: Angle, date: Date, lati: Angle, longi: Angle) -> Self {
        let de = Angle::asin(alt.sin() * lati.sin() + alt.cos() * lati.cos() * azi.cos());
        let hap = Angle::acos((alt.sin() - lati.sin() * de.sin()) / (lati.cos() * de.cos()));
        let ha = match azi.sin() < 0.0 {
            true => hap,
            false => Angle::from_degrees(360.0 - hap.degrees()),
        };
        Coord::from_equatorial(date.time().gst(date) + longi - ha, de)
    }

    /// Used in solar calculations, based on the plane of the orbit of the earth
    ///
    /// From Practical Astronomy with Your Calculator, Although similar algorithms exist in other sources
    pub fn ecliptic(self, d: Date) -> (Angle, Angle) {
        let (ra, de) = self.equatorial();
        let e = mean_obliquity_ecl(d);
        let beta = Angle::asin(de.sin() * e.cos() - de.cos() * e.sin() * ra.sin());
        let y = ra.sin() * e.cos() + de.tan() * e.sin();
        let x = ra.cos();
        let lambda = Angle::atan2(y, x);
        (lambda, beta)
    }
    /// Used in solar calculations, based on the plane of the orbit of the earth
    ///
    /// From Practical Astronomy with Your Calculator, Although similar algorithms exist in other sources
    pub fn from_ecliptic(lambda: Angle, beta: Angle, d: Date) -> Self {
        let e = mean_obliquity_ecl(d);
        let de = Angle::asin(beta.sin() * e.cos() + beta.cos() * e.sin() * lambda.sin());
        let ra = Angle::atan2(lambda.sin() * e.cos() - beta.tan() * e.sin(), lambda.cos());
        Coord::from_equatorial(ra, de)
    }

    /// Returns the angle between two objects
    pub fn dist(self, from: Self) -> Angle {
        let ((a1, d1), (a2, d2)) = (self.equatorial(), from.equatorial());
        Angle::acos(d1.sin() * d2.sin() + d1.cos() * d2.cos() * (a1 - a2).cos())
    }
    /// Returns (Rise, Set) UT, This function will fail for locations in the sky that never appear over the horizon
    ///
    /// From Practical Astronomy with Your Calculator, Although similar algorithms exist in other sources
    pub fn riseset(self, date: Date, lati: Angle, longi: Angle) -> Option<(Angle, Angle)> {
        let (ra, de) = self.equatorial();
        let ar = Angle::acos(de.sin() / lati.cos());
        let h = Angle::acos(-lati.tan() * de.tan());
        if h.radians().is_nan() || ar.radians().is_nan() {
            return None;
        }
        let lsts = (ra - h - longi).ungst(date);
        let lstr = (ra + h - longi).ungst(date);
        Some((lsts, lstr))
    }

    /// (Roughly) Accounts for precession in coordinates.
    pub fn precess(self, epoch: Date, d: Date) -> Self {
        let (ra, de) = self.equatorial();
        let diff = (d.julian() - epoch.julian()) / 365.25;
        let m =
            Angle::from_clock(0, 0, 3.07234) + Angle::from_clock(0, 0, 0.00186) * epoch.centuries();
        let n = Angle::from_degminsec(0, 0, 20.0468)
            + Angle::from_degminsec(0, 0, 0.0085) * epoch.centuries();
        let n = n.to_latitude();
        let deltara = m.degrees() + n.degrees() * ra.sin() * de.tan();
        let deltade = n.to_latitude().degrees() * ra.cos();
        Coord::from_equatorial(
            ra + Angle::from_degrees(deltara * diff),
            de + Angle::from_degrees(deltade * diff),
        )
    }
}

/// A point in 3 dimensional space, equivalent to the vector space R^3.
/// ICRF, Solar Relative, and in AU by convention.
///
/// Due to the ambiguity between reference frames, Polar methods are suffixed with `referenceframe_relative`
///
/// | System      | To Method                                      | From Method                                       |
/// |-------------|------------------------------------------------|---------------------------------------------------|
/// | Polar       | [`Position::cartesian()`]                      | [`Position::from_cartesian()`]                    |
/// | Cartesian   | [`Position::polar_referenceobject_relative()`] | [`Position::from_polar_referenceobject_relative`] |
///
/// Additional Methods:
/// * coords_geo: Coords once converting from Heliocentric -> Geocentric
/// * dist: The distance of the object
/// * normalize: Convert the object so the distance is 1.0
///
/// The main uses for this type are representing a 2D coordinate with a distance, and translating between
/// reference frames.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Position(f64, f64, f64);
impl Position {
    /// The Origin, Analogous to the position of the reference frame
    pub const ZERO: Self = Self(0.0, 0.0, 0.0);

    /// The Cartesian coordinates of the position
    pub const fn cartesian(self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }

    /// Constructs a position from Cartesian coordinates
    pub const fn from_cartesian(x: (f64, f64, f64)) -> Position {
        Position(x.0, x.1, x.2)
    }

    /// The distance from an object to the reference frame; The magnitude of the vector.
    ///
    /// Conventionally in AU
    pub fn dist(self) -> f64 {
        let Self(x, y, z) = self;
        (x * x + y * y + z * z).sqrt()
    }

    /// Converts solar coordinates to geocentric
    pub fn solar_to_geo(self, d: Date) -> Self {
        self - crate::sol::EARTH.position(d)
    }

    /// Converts geocentric coordinates to solar
    pub fn geo_to_solar(self, d: Date) -> Self {
        self + crate::sol::EARTH.position(d)
    }

    /// The 2D coordinates of the object relative to the Earth, assuming the input coordinate is solar relative
    /// (which they are by default).
    ///
    /// Does not retain distance.
    pub fn coords_geo(self, d: Date) -> Coord {
        self.solar_to_geo(d).coords_referenceobject_relative()
    }

    /// From the perspective of the reference object, the direction of the vector as a 2D [`Coord`]
    ///
    /// Note: This assumes that the position reference frame is **geocentric** (and ICRF), use [`Position::coords_geo`]
    /// if your position value was returned from another function, since the API treats `Position` as heliocentric.
    ///
    /// This does not retain the distance to the object
    pub fn coords_referenceobject_relative(self) -> Coord {
        let Self(tx, ty, tz) = self;
        let r = (tx * tx + ty * ty + tz * tz).sqrt();
        let l = Angle::atan2(ty, tx);
        let t2 = Angle::from_radians(0.5 * std::f64::consts::PI - (tz / r).acos());

        Coord::from_equatorial(l, t2)
    }

    /// Decomposes a position into its 2d coordinates and distance, relative to the origin object (by default the Sun)
    pub fn polar_referenceobject_relative(self) -> (Coord, f64) {
        (self.coords_referenceobject_relative(), self.dist())
    }

    /// Constructs an object from coordinates and distance
    pub fn from_polar_referenceobject_relative(c: Coord, dist: f64) -> Self {
        let (lat, long) = c.equatorial();
        let x = dist * lat.cos() * long.cos();
        let y = dist * lat.cos() * long.sin();
        let z = dist * lat.sin();

        Self::from_cartesian((x, y, z))
    }

    /// Normalizes the vector so that the distance is 1.0 but the direction remains the same
    pub fn normalize(self) -> Self {
        self / self.dist()
    }
}
use std::ops::{Add, Div, Mul, Sub};
impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Self(x1, y1, z1) = self;
        let Self(x2, y2, z2) = rhs;

        Self(x1 + x2, y1 + y2, z1 + z2)
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let Self(x1, y1, z1) = self;
        let Self(x2, y2, z2) = rhs;

        Self(x1 - x2, y1 - y2, z1 - z2)
    }
}
impl Mul<f64> for Position {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let Self(x1, y1, z1) = self;

        Self(x1 * rhs, y1 * rhs, z1 * rhs)
    }
}
impl Div<f64> for Position {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let Self(x1, y1, z1) = self;

        Self(x1 / rhs, y1 / rhs, z1 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Many of these tests do not conform with data you can pull out of stellarium/other tools, they are correct nonetheless.
    // * How do you know?: By personal confirmation of the result data with other resources
    // * Then how can I use these functions?: In conjunction with the functions for correction (procession, nutation, abberation, refraction)
    // Note that even without correction, these tests are almost always within 16' (half the moons diameter).
    #[test]
    fn test_horiz() {
        let arcturus = Coord::from_equatorial(
            Angle::from_clock(14, 16, 50.0),
            Angle::from_degminsec(19, 02, 50.1),
        );
        let sirius = Coord::from_equatorial(
            Angle::from_clock(6, 46, 13.1),
            Angle::from_degminsec(-16, 45, 06.8),
        );
        assert_eq!(
            arcturus.horizon(
                Date::from_calendar(2025, 3, 10, Angle::from_clock(19, 52, 25.0)),
                Angle::from_degrees(55.47885),
                Angle::from_degrees(133.94531)
            ),
            (
                Angle::from_degminsec(220, 39, 16.2),
                Angle::from_degminsec(48, 6, 46.1)
            )
        );
        assert_eq!(
            sirius.horizon(
                Date::from_calendar(2025, 3, 7, Angle::from_clock(23, 36, 52.0)),
                Angle::from_degrees(5.0),
                Angle::from_degrees(-1.0)
            ),
            (
                Angle::from_degminsec(247, 58, 18.2),
                Angle::from_degminsec(28, 11, 54.8)
            )
        );
        assert_eq!(
            sirius.horizon(
                Date::from_calendar(2025, 3, 11, Angle::from_clock(2, 0, 0.0)),
                Angle::from_degrees(44.8714),
                Angle::from_degrees(-93.20801)
            ),
            (
                Angle::from_degminsec(184, 41, 2.3),
                Angle::from_degminsec(28, 15, 27.2)
            )
        );
        assert_eq!(
            Coord::from_horizon(
                Angle::from_degminsec(184, 41, 2.3),
                Angle::from_degminsec(28, 15, 6.33),
                Date::from_calendar(2025, 3, 11, Angle::from_clock(2, 0, 0.0)),
                Angle::from_degrees(44.8714),
                Angle::from_degrees(-93.20801)
            ),
            sirius
        );
        assert_eq!(sirius.dist(arcturus), Angle::from_degminsec(116, 16, 31.26));
    }

    #[test]
    fn test_riseset() {
        let c = Coord::from_equatorial(
            Angle::from_clock(23, 39, 20.0),
            Angle::from_degminsec(21, 42, 00.0),
        );
        assert_eq!(
            c.riseset(
                Date::from_calendar(1980, 8, 24, Angle::default()),
                Angle::from_degrees(30.0),
                Angle::from_degrees(64.0)
            ),
            Some((Angle::from_clock(14, 18, 9.0), Angle::from_clock(4, 6, 5.0)))
        );
        assert_eq!(
            c.riseset(
                Date::from_calendar(1980, 8, 24, Angle::default()),
                Angle::from_degrees(-85.0),
                Angle::from_degrees(0.0),
            ),
            None
        );
    }

    #[test]
    fn test_ecliptic() {
        let star1 = Coord::from_equatorial(
            Angle::from_clock(9, 34, 53.6),
            Angle::from_degminsec(19, 32, 14.2),
        );
        assert_eq!(
            star1.ecliptic(Date::from_calendar(1950, 0, 1, Angle::default())),
            (
                Angle::from_degminsec(139, 41, 10.0),
                Angle::from_degminsec(4, 52, 31.0)
            )
        );
        assert_eq!(
            Coord::from_ecliptic(
                Angle::from_degminsec(139, 41, 10.0),
                Angle::from_degminsec(4, 52, 31.0),
                Date::from_calendar(1950, 0, 1, Angle::default())
            ),
            star1
        );
    }

    #[test]
    fn test_cart() {}
}
