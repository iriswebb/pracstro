use pracstro::time;
use value::*;

/// Handles the reading and querying of the catalog of celestial objects
use ephem_core::catalog;
pub mod output;
pub mod parse;
use ephem_core::query;
use ephem_core::value;

/// pracstro provides a way to do this, but that isn't functional in a lot of contexts
///
/// Used in ephemeris generation and date reading
mod timestep {
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
    use clap::{arg, command};
    let cat = catalog::read();
    let ccheck = cat.clone();
    let matches = command!()
    	.help_template("{before-help}{name} ({version}) - {about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}\n\nWritten by {author}")
        .arg(
            arg!(-d --date [Date] "Set the date")
                .value_parser(parse::date)
                .default_value("now"),
        )
        .arg(
            arg!(-l --latlong ["Latitude,Longitude"] "Set the latitude/longitude")
                .value_parser(parse::latlong)
                .default_value("none"),
        )
        .arg(arg!(-E --ephem ["Start,Step,End"] "Generates Table").value_parser(parse::ephemq))
        .arg(
            arg!(-T --format [Format] "Output Format")
                .value_parser(["term", "rawterm", "csv", "json"])
                .default_value("term"),
        )
        .arg(arg!([object] "Celestial Object").required(true).value_parser(move |s: &str| parse::object(s, &ccheck)))
        .arg(arg!([properties] ... "Properties").required(true).value_parser(move |s: &str| parse::property(s, &cat)))
        .get_matches();
    let location = *matches.get_one("latlong").unwrap();
    let date = *matches.get_one("date").unwrap();
    let formatter = match matches.get_one::<String>("format").unwrap().as_str() {
        "term" => output::TERM,
        "rawterm" => output::RAWTERM,
        "csv" => output::CSV,
        "json" => output::JSON,
        _ => todo!(),
    };

    let obj = matches.get_one::<CelestObj>("object").unwrap().clone();
    let propl: Vec<query::Property> = matches
        .get_many::<query::Property>("properties")
        .unwrap()
        .cloned()
        .collect();

    let q = |loc: Location, d: time::Date| {
        query::run(&obj, &propl, loc, d).unwrap_or_else(|x| panic!("Failed to parse query: {x}"))
    };

    (formatter.start)();

    if let Some((start, step, end)) =
        matches.get_one::<(time::Date, timestep::Step, time::Date)>("ephem")
    {
        (formatter.propheader)(&propl);
        timestep::EphemIter::new(*start, *step, *end)
            .map(|date| (q(location, date), date))
            .for_each(|(r, date)| (formatter.ephemq)(r, &propl, date))
    } else {
        (formatter.query)(q(location, date));
    }

    (formatter.footer)();
}
