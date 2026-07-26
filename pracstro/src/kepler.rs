//! Functions for modeling arbitrary solar keplerian orbits
//!
//! Adapted from XEphem's `libastro`

use crate::celobj::CelObj;
use crate::coord::Coord;
use crate::time::Angle;
use crate::time::Date;
use std::f64;

/// The result
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectResult {
    /// The Coordinate (Earth Relative 2D Polar)
    pub coord: Coord,
    /// Distance from the Earth (AU)
    pub earthdist: f64,
    /// Distance from the Sun (AU)
    pub sundist: f64,
    /// Speed (meters per second, Sun Relative)
    pub speed: f64,
}

/// This is a tuple so that CSV data from JPL Horizons can be effortlessly translated into code
///
/// To make one of these, use the JPL Horizons interface @ <https://ssd.jpl.nasa.gov/horizons/app.html#/>,
/// with ephemeris type as Osculating Orbital Elements, CSV format enabled, and reference plane set to ecliptic XY.
/// Any other settings are wrong.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JPLRawElements(
    /// Epoch of parameters, JDTDB
    pub f64,
    /// * EC     Eccentricity, e
    pub f64,
    /// * QR     Periapsis distance, q (au)
    pub f64,
    /// * IN     Inclination w.r.t X-Y plane, i (degrees)
    pub f64,
    /// * OM     Longitude of Ascending Node, OMEGA, (degrees)
    pub f64,
    /// * W      Argument of Perifocus, w (degrees)
    pub f64,
    /// * Tp     Time of periapsis (Julian Day Number)
    pub f64,
    /// * N      Mean motion, n (degrees/day)
    pub f64,
    /// * MA     Mean anomaly, M (degrees)
    pub f64,
    /// * TA     True anomaly, nu (degrees)
    pub f64,
    /// * A      Semi-major axis, a (au)
    pub f64,
    /// * AD     Apoapsis distance (au)
    pub f64,
    /// * PR     Sidereal orbit period (day)
    pub f64,
);

/// Calculates the instantaneous orbital speed for any keplerian orbit.
///
/// Arguments are AU, Returns meters per second.
///
/// Accepts the inverse of a instead of a because
/// for parabolic orbits a is infinite.
/// (In which case the inverse of is 0)
fn vis_viva(inverse_of_a: f64, sundist: f64) -> f64 {
    const G: f64 = 6.6743015E-11;
    const M: f64 = 1.988475E+30;
    const M_IN_AU: f64 = 149597870700.0;

    // Convert to M
    let dist = sundist * M_IN_AU;
    let inverse_of_a = inverse_of_a * M_IN_AU.recip();

    ((G * M) * ((2.0 / dist) - inverse_of_a)).sqrt()
}

// For some reason, all the two body formula math uses degrees instead of radians
/// Solve the equation of Kepler
fn kepler(m: Angle, ecc: f64) -> f64 {
    let m = m.radians();
    let mut e = m;
    let mut delta: f64;

    delta = e - ecc * e.sin() - m;
    e -= delta / (1.0 - ecc * e.cos());
    while delta.abs() > 1E-6 {
        delta = e - ecc * e.sin() - m;
        e -= delta / (1.0 - ecc * e.cos());
    }
    return e.to_degrees();
}

/// Solves True Anomaly and the radius vector for Elliptic orbits only:
/// computes: v = true anomaly   (degrees)
///           r = radius vector  (a.u.)
///   from:   m = mean anomaly   (degrees)
///           e = eccentricity
///           a = semimajor axis (a.u.)
///
fn vr(m: Angle, e: f64, a: f64) -> (Angle, f64) {
    let ean = kepler(m, e).to_radians();
    let x = a * (ean.cos() - e);
    let y = a * (1. - e * e).sqrt() * ean.sin();
    let r = x.hypot(y);
    let v = Angle::atan2(y, x);

    (v, r)
} /* vr */

fn vrc(tp: f64, e: f64, q: f64) -> (Angle, f64) {
    if tp == 0.0 {
        let v = Angle::default();
        let r = q;
        return (v, r);
    }

    let lambda = (1.0 - e) / (1.0 + e);

    // Elliptic
    if lambda > 0.0 {
        let a = q / (1.0 - e);
        // Gauss gravitational constant
        let m = 1.720209895E-2f64.to_degrees() * tp / (a * a * a).sqrt();
        return vr(Angle::from_degrees(m), e, a);
    }
    // Hyperbolic
    else {
        let a = q / (e - 1.0); /* Semi-major axis */
        let n = 1.720209895E-2 * tp / (a * a * a).sqrt(); /* "Daily motion" */
        let g = n / e;
        let adgg = 1E+10;

        let mut adgg2 = adgg;
        let mut gs = (g * g + 1.0).sqrt();
        let mut dg = -(e * g - (g + gs).ln() - n) / (e - 1.0 / gs);
        let mut g = g + dg;
        let mut adgg = (dg / g).abs();

        while adgg < adgg2 && adgg > 1E-5 {
            adgg2 = adgg;
            gs = (g * g + 1.0).sqrt();
            dg = -(e * g - (g + gs).ln() - n) / (e - 1.0 / gs);
            g = g + dg;
            adgg = (dg / g).abs();
        }
        gs = (g * g + 1.0).sqrt();
        let v = Angle::from_degrees(
            2.0 * (((e + 1.0) / (e - 1.0)).sqrt() * g / (gs + 1.0))
                .atan()
                .to_degrees(),
        );
        let r = q * (1.0 + e) / (1.0 + e * v.cos());
        return (v, r);
    }
}

fn helio_to_object(psi: f64, lpd: f64, d: Date, r: f64, op10_inv: f64) -> ObjectResult {
    let (lsn, rsn) = (
        crate::sol::SUN.location(d).ecliptic(d).0.radians(),
        crate::sol::SUN.distance(d),
    );
    /* heliocentric latitude (ecl) */
    let lg = lsn + f64::consts::PI;
    let cpsi = psi.cos();
    let spsi = psi.sin();
    let rpd = r * cpsi;

    /* helio angle between object and earth */
    let ll = lpd - lg;
    let (sll, cll) = ll.sin_cos();

    /* distance from the Earth */
    let rho = (rsn * rsn + r * r - 2.0 * rsn * r * cpsi * ll.cos()).sqrt();

    /* find geocentric ecliptic longitude and latitude */
    let lam = if rpd < rsn {
        ((-1.0 * rpd * sll) / (rsn - (rpd * cll))).atan() + lg + f64::consts::PI
    } else {
        ((rsn * sll) / (rpd - (rsn * cll))).atan() + lpd
    };

    let lam = Angle::from_radians(lam).radians();
    let bet = ((rpd * spsi * (lam - lpd).sin()) / (cpsi * rsn * sll)).atan();

    let coord = Coord::from_ecliptic(Angle::from_radians(lam), Angle::from_radians(bet), d);

    ObjectResult {
        coord,
        earthdist: rho,
        sundist: r,
        speed: vis_viva(op10_inv, r),
    }
}

fn obj_parabolic(date: Date, op: JPLRawElements) -> ObjectResult {
    let errlimit = 0.0001;

    let inc = op.3.to_radians();
    let ap = op.5.to_radians();
    let qp = op.2;
    let om = op.4.to_radians();

    let w = ((date.julian() - op.6) * 3.649116e-02) / (qp * (qp.sqrt()));
    let mut s = w / 3.0;
    let mut s2 = s * s;
    let mut d;
    for _ in 0..100 {
        s2 = s * s;
        d = (s2 + 3.0) * s - w;
        if d.abs() <= errlimit {
            break;
        }
        s = ((2.0 * s * s2) + w) / (3.0 * (s2 + 1.0));
    }
    let nu = 2.0 * s.atan();
    let rp = qp * (1.0 + s2);
    let l = nu + ap;
    let (sl, cl) = l.sin_cos();
    let spsi = sl * inc.sin();
    let psi = spsi.asin();
    let y = sl * inc.cos();
    let mut lpd = (y / cl).atan() + om;
    if cl < 0.0 {
        lpd += f64::consts::PI
    }
    lpd = Angle::from_radians(lpd).radians();

    helio_to_object(psi, lpd, date, rp, 0.0)
}

fn obj_elliptical(d: Date, op: JPLRawElements) -> ObjectResult {
    // True anomaly and sun-object distance
    let (v, r) = vrc(d.julian() - op.6, op.1, op.10 * (1.0 - op.1));

    /* angle from ascending node */
    let (slo, clo) = (v + Angle::from_degrees(op.5)).sin_cos();

    let psi = (slo * op.3.to_radians().sin()).asin();
    let y = slo * op.3.to_radians().cos();

    /* heliocentric longitude (ecl) */
    let mut lpd = (y / clo) + op.4.to_radians();
    if clo < 0.0 {
        lpd += f64::consts::PI;
    }

    helio_to_object(psi, lpd, d, r, op.10.recip())
}

fn obj_hyperbolic(d: Date, op: JPLRawElements) -> ObjectResult {
    // True anomaly and sun-object distance
    let (v, r) = vrc(d.julian() - op.6, op.1, op.10 * (1.0 - op.1));

    /* angle from ascending node */
    let (slo, clo) = (v + Angle::from_degrees(op.5)).sin_cos();

    let psi = (slo * op.3.to_radians().sin()).asin();
    let y = slo * op.3.to_radians().cos();

    /* heliocentric longitude (ecl) */
    let mut lpd = (y / clo).atan() + op.4.to_radians();
    if clo < 0.0 {
        lpd += f64::consts::PI;
    }

    helio_to_object(psi, lpd, d, r, op.10.recip())
}

/// Converts Orbital Elements into Coordinates, Distance, and Speed
pub fn kepl_obj(d: Date, elements: JPLRawElements) -> ObjectResult {
    match elements.3 {
        ..=0.998 => obj_elliptical(d, elements),
        0.998..=1.002 => obj_parabolic(d, elements),
        _ => obj_hyperbolic(d, elements),
    }
}

#[cfg(test)]
mod tests {
    use super::{obj_elliptical, obj_hyperbolic, obj_parabolic, JPLRawElements};

    // Test Objects (Generated using JPL Horizons, all elements dated 2026-01-01T00:00:00 TBD)
    const VENUS: JPLRawElements = JPLRawElements(
        2461041.500000000,
        6.780942266658413E-03,
        7.184295614530095E-01,
        3.394392156342094E+00,
        7.660714242821713E+01,
        5.487317496286026E+01,
        2.460950913045631E+06,
        1.602123093497366E+00,
        1.451314515634737E+02,
        1.455726079640048E+02,
        7.233344506020272E-01,
        7.282393397510447E-01,
        2.247018356212165E+02,
    );
    const HALEBOP: JPLRawElements = JPLRawElements(
        2461041.500000000,
        9.949126551461109E-01,
        9.165130544288572E-01,
        8.976952938034762E+01,
        2.817372689030623E+02,
        1.306503045023052E+02,
        2.450536543945273E+06,
        4.075987636321965E-04,
        4.281807099917236E+00,
        1.655992632658057E+02,
        1.801554800689909E+02,
        3.593944470835529E+02,
        8.832215210663691E+05,
    );
    const VOYAGER2: JPLRawElements = JPLRawElements(
        2461041.500000000,
        6.283315906824292E+00,
        2.124898323240814E+01,
        7.900274168484320E+01,
        1.018247301091292E+02,
        1.300377591052990E+02,
        2.445451634079524E+06,
        1.221959218909291E-01,
        1.905018038308568E+03,
        8.916028523029459E+01,
        -4.021902836618472E+00,
        9.999999999999998E+99,
        9.999999999999998E+99,
    );

    const THRESHOLD: f64 = 1E-5;

    macro_rules! assert_float_eq {
        ($e:expr, $f:expr) => {
            let d = ($e - $f).abs();
            if d > THRESHOLD || d.is_nan() {
                panic!(
                    "FP Approx Expect {} but got {}, not within {}",
                    $f, $e, THRESHOLD
                );
            }
        };
    }

    #[test]
    fn test_kepl() {
        use crate::time::Angle;
        use crate::time::Date;

        let testdate_vvoyager = Date::from_calendar(2026, 07, 20, Angle::default());
        let testdate_halebopp = Date::from_calendar(1997, 04, 01, Angle::default());

        let venus_loc = obj_elliptical(testdate_vvoyager, VENUS);
        let halebop_loc = obj_parabolic(testdate_halebopp, HALEBOP);
        let voyager2_loc = obj_hyperbolic(testdate_vvoyager, VOYAGER2);

        assert_float_eq!(venus_loc.coord.equatorial().0.degrees(), 162.6198935);
        assert_float_eq!(venus_loc.coord.equatorial().1.degrees(), 8.325871295);
        assert_float_eq!(venus_loc.earthdist, 0.9045147);
        assert_float_eq!(venus_loc.sundist, 0.72468058);
        assert_float_eq!(venus_loc.speed, 34956.0593699279);

        assert_float_eq!(halebop_loc.coord.equatorial().0.degrees(), 31.0226061);
        assert_float_eq!(halebop_loc.coord.equatorial().1.degrees(), 43.0141461);
        assert_float_eq!(halebop_loc.earthdist, 1.35560729);
        assert_float_eq!(halebop_loc.sundist, 0.918052202);
        assert_float_eq!(halebop_loc.speed, 43962.419042);

        assert_float_eq!(voyager2_loc.coord.equatorial().0.degrees(), 303.15603528);
        assert_float_eq!(
            voyager2_loc.coord.equatorial().1.to_latitude().degrees(),
            -59.79625509
        );
        assert_float_eq!(voyager2_loc.earthdist, 142.6615206);
        assert_float_eq!(voyager2_loc.sundist, 143.45106309);
        assert_float_eq!(voyager2_loc.speed, 15262.7067879);
    }
}
