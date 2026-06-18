use crate::query::Property;
use crate::value::*;
use pracstro::time;

/// A set of functions that handle the formatting of queries
pub struct Driver {
    /// Starting information
    pub start: fn() -> (),
    /// Headers for columns, usually
    pub propheader: fn(&[Property]) -> (),
    /// The formatting in a normal query
    pub query: fn(Vec<Value>) -> (),
    /// The formatting in a ephemeris query
    pub ephemq: fn(Vec<Value>, &[Property], time::Date) -> (),
    /// Ending information
    pub footer: fn() -> (),
}

pub fn nop() {}
pub fn nop_fa(_: &[Property]) {}

fn term_proph(rs: &[Property]) {
    println!("{:=<1$}", "", 29 * rs.len() + 22);
    print!("{:^22}", "Date");
    rs.iter().for_each(|x| print!("{:^29}", x.to_string()));
    println!("\n{:=<1$}", "", 29 * rs.len() + 22);
}
fn term_q(rs: Vec<Value>) {
    println!(
        "{}",
        rs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
fn term_eq(rs: Vec<Value>, _: &[Property], d: time::Date) {
    print!("{:^22}", Value::Date(d).to_string());
    rs.iter().for_each(|x| print!("{:<29}", x.to_string()));
    println!();
}
pub const TERM: Driver = Driver {
    start: nop,
    propheader: term_proph,
    query: term_q,
    ephemq: term_eq,
    footer: nop,
};

fn raw_term_q(rs: Vec<Value>) {
    println!(
        "{}",
        rs.iter()
            .map(|x| format!("{:#}", x))
            .collect::<Vec<String>>()
            .join(" ")
    );
}
pub const RAWTERM: Driver = Driver {
    start: nop,
    propheader: term_proph,
    query: raw_term_q,
    ephemq: term_eq,
    footer: nop,
};

fn csv_proph(rs: &[Property]) {
    println!(
        "Date,{}",
        rs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}
fn csv_q(rs: Vec<Value>) {
    println!(
        "{}",
        rs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}
fn csv_eq(rs: Vec<Value>, _: &[Property], d: time::Date) {
    println!(
        "{},{}",
        Value::Date(d),
        rs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}
pub const CSV: Driver = Driver {
    start: nop,
    propheader: csv_proph,
    query: csv_q,
    ephemq: csv_eq,
    footer: nop,
};

fn json_init() {
    print!("{{ \"q\": [");
}
fn json_q(rs: Vec<Value>) {
    print!("[");
    rs.iter().for_each(|x| print!("{:#},", x));
    print!("\"endq\" ],");
}
fn json_eq(rs: Vec<Value>, nm: &[Property], d: time::Date) {
    print!("{{ \"timestamp\": {},", d.unix());
    rs.iter()
        .enumerate()
        .for_each(|(n, x)| print!("\"{}\": {:#},", nm[n], x));
    print!("\"isq\": true }},");
}
fn json_footer() {
    print!("{{\"isq\": false}} ] }}");
}
pub const JSON: Driver = Driver {
    start: json_init,
    propheader: nop_fa,
    query: json_q,
    ephemq: json_eq,
    footer: json_footer,
};
