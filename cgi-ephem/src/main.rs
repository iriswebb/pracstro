use ephem_core::catalog;
use pracstro::time;
use std::env;

use crate::text::{ANSI_DRIVER, HTML_DRIVER, TEXT_DRIVER};

/// Handles the reading and querying of the catalog of celestial objects
pub mod parse;
pub mod query;
pub mod text;
pub mod tile;
pub mod value;

/// pracstro provides a way to do this, but that isn't functional in a lot of contexts
///
/// Used in ephemeris generation and date reading
pub mod timestep {
    use chrono::prelude::*;
    use pracstro::time;
    #[derive(Copy, Clone, Debug, PartialEq)]
    /// Most things can be represented as seconds or months
    /// * 1 second: 1 second
    /// * 1 minute: 60 seconds
    /// * 1 hour: 3600 seconds
    /// * 1 day: 86400 seconds
    /// * 1 week: 604800 seconds
    /// * 1 month: 1 month
    /// * 1 year: 12 months
    pub enum Step {
        S(f64),
        M(chrono::Months),
    }
    pub fn step_forward_date(d: time::Date, s: Step) -> time::Date {
        match s {
            Step::S(sec) => time::Date::from_julian(d.julian() + (sec.abs() / 86400.0)),
            Step::M(m) => time::Date::from_unix(
                (DateTime::from_timestamp(d.unix() as i64, 0).unwrap() + m).timestamp() as f64,
            ),
        }
    }
    pub fn step_back_date(d: time::Date, s: Step) -> time::Date {
        match s {
            Step::S(sec) => time::Date::from_julian(d.julian() - (sec.abs() / 86400.0)),
            Step::M(m) => time::Date::from_unix(
                (DateTime::from_timestamp(d.unix() as i64, 0).unwrap() - m).timestamp() as f64,
            ),
        }
    }
    pub struct EphemIter {
        now: time::Date,
        step: Step,
        end: time::Date,
    }
    impl EphemIter {
        pub fn new(start: time::Date, step: Step, end: time::Date) -> EphemIter {
            EphemIter {
                now: start,
                step,
                end,
            }
        }
    }
    impl Iterator for EphemIter {
        type Item = time::Date;

        fn next(&mut self) -> Option<Self::Item> {
            if self.now.julian() < self.end.julian() {
                let s = self.now;
                self.now = step_forward_date(self.now, self.step);
                Some(s)
            } else {
                None
            }
        }
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let driver = match argv[1].as_str() {
        "html" => HTML_DRIVER,
        "ansi" => ANSI_DRIVER,
        _ => TEXT_DRIVER,
    };

    let date = time::Date::now();
    let obj = parse::object(argv[2].as_str(), &catalog::read());

    if obj.is_err() {
        println!("The specified object does not exist");
        return;
    }
    let obj = obj.unwrap();

    let data = query::generate_cgi_data(&obj, date);

    print!("{}", driver.header);
    print!(
        "Report for {} on JD{:0.2}{}",
        argv[2],
        date.julian(),
        driver.eol
    );

    let location_tile: Vec<String> = (0..=14)
        .map(|x| {
            tile::location::location_tile(data.location, x, date)
                .into_iter()
                .map(driver.render_atom)
                .collect::<String>()
        })
        .collect();

    let phase_tile: Vec<String> = if let Some(phaseangle) = data.phaseangle {
        (0..=14)
            .map(|x| {
                tile::phase::phase_tile(phaseangle, x, &obj)
                    .into_iter()
                    .map(driver.render_atom)
                    .collect::<String>()
            })
            .collect()
    } else {
        (0..=14)
            .map(|x| {
                tile::na_nostart_tile(x, " Phase ".into())
                    .into_iter()
                    .map(driver.render_atom)
                    .collect::<String>()
            })
            .collect()
    };

    let brightness_tile: Vec<String> = (0..=14)
        .map(|x| {
            tile::brightness::brightness_tile(data.brightness, x)
                .into_iter()
                .map(driver.render_atom)
                .collect::<String>()
        })
        .collect();

    let distance_tile: Vec<String> = (0..=14)
        .map(|x| {
            tile::distance::distance_tile(data.dist, data.angdia, x)
                .into_iter()
                .map(driver.render_atom)
                .collect::<String>()
        })
        .collect();

    for i in 0..=13 {
        print!("{}{}{}", location_tile[i], phase_tile[i], driver.eol);
    }
    for i in 0..=14 {
        print!("{}{}{}", distance_tile[i], brightness_tile[i], driver.eol);
    }

    print!("{}", driver.footer);
}
