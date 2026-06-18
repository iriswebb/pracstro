//! WASM frontend for websites and other tools

use ephem_core::value::CelestObj;
use ephem_core::*;
use pracstro::moon::MOON;
use pracstro::time::Angle;
use pracstro::*;
use query::Property;
use std::collections::HashMap;
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;
extern crate web_sys;

/// This is needed for speed, to not format the catalog every query.
/// My testing showed the query speed reduced by a factor of around 20 with this.
static CATALOG: OnceLock<HashMap<&'static str, value::CelestObj>> = OnceLock::new();

/// Initializes the catalog in a private hashmap value
#[wasm_bindgen]
pub unsafe fn catalog_init() -> bool {
    CATALOG.set(catalog::read()).is_ok()
}

pub fn parse_property(s: &str) -> Result<query::Property, &'static str> {
    match s {
        "equ" | "equa" | "equatorial" => Ok(Property::Equatorial),
        "horiz" | "horizontal" => Ok(Property::Horizontal),
        "ecl" | "ecliptic" => Ok(Property::Ecliptic),
        "dist" | "distance" => Ok(Property::Distance),
        "mag" | "magnitude" | "brightness" => Ok(Property::Magnitude),
        "phase" => Ok(Property::PhaseDefault),
        "phaseemoji" => Ok(Property::PhaseEmoji),
        "phasename" => Ok(Property::PhaseName),
        "phaseangle" => Ok(Property::PhaseAngle),
        "angdia" => Ok(Property::AngDia),
        "phaseprecent" | "illumfrac" => Ok(Property::IllumFrac),
        "rise" => Ok(Property::Rise),
        "set" => Ok(Property::Set),
        _ => Err("Unknown Property"),
    }
}

pub fn parse_object(
    sm: &str,
    cat: &std::collections::HashMap<&'static str, value::CelestObj>,
) -> Result<value::CelestObj, String> {
    cat.get(sm.to_lowercase().as_str())
        .cloned()
        .ok_or("Unknown Object".into())
}

#[wasm_bindgen]
pub unsafe fn location_cart_of(object: &str, time: f64) -> Result<Vec<f64>, String> {
    fn c(f: (f64, f64, f64)) -> Vec<f64> {
        vec![f.0, f.1, f.2]
    }
    let selection = parse_object(object, CATALOG.get().unwrap())?;
    let date = pracstro::time::Date::from_unix(time);
    match selection {
        CelestObj::Moon => Ok(c(MOON.locationcart(date))),
        CelestObj::Sun => Ok(c(pracstro::sol::SUN.locationcart(date))),
        CelestObj::Planet(p) => Ok(c(p.locationcart(date))),
        CelestObj::Star(s) => Ok(c(pracstro::coord::Coord::cartesian(
            s.loc_j2k,
            (1.0 / s.pi.degrees() * 1296000.0) * 3.26,
        ))),
        CelestObj::Crd(_) => {
            Err("Can not get the 3d coordinate of a 2d coordinate without distance".into())
        }
    }
}

#[wasm_bindgen]
pub unsafe fn webephem_query(
    object: &str,
    property: &str,
    time: f64,
    lat: Option<f64>,
    long: Option<f64>,
    formatted: bool,
) -> Result<String, String> {
    let latlong = if let (Some(y), Some(x)) = (lat, long) {
        Some((Angle::from_degrees(x), Angle::from_degrees(y)))
    } else {
        None
    };
    let object = parse_object(object, CATALOG.get().unwrap())?;
    let date = pracstro::time::Date::from_unix(time);

    let value = query::property_of(
        &object,
        parse_property(property)?,
        &value::RefFrame {
            latlong: latlong,
            date: date,
        },
    )?;

    if formatted {
        return Ok(format!("{}", value));
    } else {
        return Ok(format!("{:#}", value));
    }
}

/// Table Generation
#[wasm_bindgen]
pub unsafe fn webephem_batch_query(
    object: &str,
    property: &str,
    start_time: f64,
    step_time: f64,
    end_time: f64,
    lat: Option<f64>,
    long: Option<f64>,
    formatted: bool,
) -> Result<Vec<String>, String> {
    let latlong = if let (Some(x), Some(y)) = (lat, long) {
        Some((Angle::from_degrees(x), Angle::from_degrees(y)))
    } else {
        None
    };
    let object = parse_object(object, CATALOG.get().unwrap())?;
    let mut ephem = Vec::<String>::new();
    let prop = parse_property(property)?;

    for d in std::iter::successors(Some(start_time), |x| {
        if *x < end_time {
            Some(x + step_time)
        } else {
            None
        }
    }) {
        let date = pracstro::time::Date::from_unix(d);

        let value = query::property_of(
            &object,
            prop.clone(),
            &value::RefFrame {
                latlong: latlong,
                date: date,
            },
        )?;

        if formatted {
            ephem.push(format!("{}", value));
        } else {
            ephem.push(format!("{:#}", value));
        }
    }

    Ok(ephem)
}
