//! Celestial object trait for generics

use crate::coord::Coord;
use crate::time::{self, Angle};

/// A celestial object in pracstro is defined by the ability to query its cartesian coordinates from time
pub trait CelObj {
    /// The geocentric cartesian coordinates of the object (Equatorial, Earth-Centric)
    fn locationcart(&self, d: time::Date) -> (f64, f64, f64);
    /// The Absolute Magnitude of the object
    fn brightness(&self, d: time::Date) -> f64;
    /// The Name of the Object
    fn name(&self) -> String;

    /// The 2D Polar Coordinates of the object
    fn location(&self, d: time::Date) -> Coord {
        let (x, y, z) = self.locationcart(d);
        Coord::from_cartesian(x, y, z)
    }

    /// The distance from the reference frame to the object, in AU
    fn distance(&self, d: time::Date) -> f64 {
        let (x, y, z) = self.locationcart(d);
        (x * x + y * y + z * z).sqrt()
    }
}

/// A general trait implemented by the base catalog so that new types can be added to it
pub trait BaseCatalogObject: CelObj {
    /// The Angular Diameter of the object
    fn angdia(&self, d: time::Date) -> Option<Angle>;
    /// The phase angle of the object
    fn phaseangle(&self, d: time::Date) -> Option<Angle>;

    /// Returns the illuminated fraction of a object
    fn illumfrac(&self, d: time::Date) -> Option<f64> {
        Some(0.5 * (1.0 - self.phaseangle(d)?.cos()))
    }
}
