//! Celestial object trait for generics

use crate::coord::{Coord, Position};
use crate::time::{self, Angle};

/// A celestial object in pracstro is defined by the ability to query its position from time
pub trait CelObj {
    /// The heliocentric 3d coordinates of the Object (Sun-fixed, equatorial, AU)
    fn position(&self, d: time::Date) -> Position;
    /// The Absolute Magnitude of the object
    fn brightness(&self, d: time::Date) -> f64;
    /// The Name of the Object
    fn name(&self) -> String;

    /// The 2D Polar Coordinates of the object
    fn location(&self, d: time::Date) -> Coord {
        self.position(d).coords_geo(d)
    }

    /// The distance from the reference frame to the object, in AU
    fn distance(&self, d: time::Date) -> f64 {
        self.position(d).solar_to_geo(d).dist()
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
