use crate::text::TextFormatting;
use crate::text::{self};
use crate::text::{ANSIColors, Color};
use crate::tile;
use crate::value::CrdView;
use iau_constellations;
use pracstro::coord::Coord;
use pracstro::time;

pub fn zodiac_of(coord: Coord, date: time::Date) -> &'static str {
    [
        "Aries",
        "Taurus",
        "Gemini",
        "Cancer",
        "Leo",
        "Virgo",
        "Libra",
        "Scorpio",
        "Sagittarius",
        "Capricorn",
        "Aquarius",
        "Pisces",
    ][(coord.ecliptic(date).0.degrees() / 30.0) as usize]
}

pub fn location_tile(location: Coord, line: usize, date: time::Date) -> Vec<text::TextAtom> {
    let coords_1875 = location
        .precess(
            date,
            time::Date::from_calendar(1875, 0, 0, time::Angle::default()),
        )
        .equatorial();
    let constellation = iau_constellations::CONSTELLATION_NAMES[iau_constellations::constell_1875(
        coords_1875.0.degrees(),
        coords_1875.1.to_latitude().degrees(),
    )];
    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Location "),
            special_formatting: None,
        }],
        3 => vec![
            text::TextAtom {
                content: format!("|{:>31}", "In the Constellation"),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!(" {:<21}", constellation),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    color: None,
                    bgcolor: None,
                    italic: false,
                    underline: false,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        4 => vec![
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:>25}", "Zodiac:"),
                special_formatting: Some(TextFormatting {
                    bold: false,
                    color: Some(Color(ANSIColors::Magenta, true)),
                    bgcolor: None,
                    italic: false,
                    underline: false,
                }),
            },
            text::TextAtom {
                content: format!(" {:<27}", zodiac_of(location, date)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    color: Some(Color(ANSIColors::Magenta, true)),
                    bgcolor: None,
                    italic: false,
                    underline: false,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        7 => vec![text::TextAtom {
            content: format!("|{:^53}|", "Coordinates (Equatorial):"),
            special_formatting: None,
        }],
        8 => vec![text::TextAtom {
            content: format!(
                "|{:^53}|",
                crate::value::Value::Crd(location, CrdView::Equatorial).to_string()
            ),
            special_formatting: None,
        }],
        10 => vec![text::TextAtom {
            content: format!("|{:^53}|", "Coordinates (Ecliptic):"),
            special_formatting: None,
        }],
        11 => vec![text::TextAtom {
            content: format!(
                "|{:^53}|",
                crate::value::Value::Crd(location, CrdView::Ecliptic(date)).to_string()
            ),
            special_formatting: None,
        }],
        14 => vec![text::TextAtom {
            content: tile::TILE_FOOTER.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: tile::EMPTY_LINE.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}
