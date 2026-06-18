use crate::text::TextFormatting;
use crate::text::{self, TextAtom};
use crate::text::{ANSIColors, Color};
use crate::tile::*;

const BRIGHTNESSES: [(&str, f64); 17] = [
    ("Sun (avg)", -26.83),
    ("Full Moon", -12.60),
    ("ISS", -5.9),
    ("Venus (max)", -4.92),
    ("Mars (max)", -2.94),
    ("Jupiter (max)", -2.94),
    ("Sirius", -1.47),
    ("Vega", 0.03),
    ("Polaris", 1.98),
    ("Human Eye Limit (Heavy Pollution)", 3.0),
    ("Andromeda Galaxy", 3.44),
    ("Human Eye Limit (Medium Pollution)", 4.0),
    ("Uranus (max)", 5.38),
    ("Human Eye Limit", 6.5),
    ("Binoculars Limit", 9.5),
    ("Pluto (max)", 13.65),
    ("Hubble Limit", 31.5),
];

pub fn brightness_tile(brightness: f64, line: usize) -> Vec<TextAtom> {
    let mut index = 0;
    for (i, x) in BRIGHTNESSES.iter().enumerate() {
        if x.1 > brightness {
            index = i;
            break;
        }
    }

    match line {
        0 => vec![text::TextAtom {
            content: format!("{:-^53}+", " Brightness "),
            special_formatting: None,
        }],
        2 => vec![
            text::TextAtom {
                content: format!("{:^53}", format!("Brightness: {:.2}", brightness)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(Color(ANSIColors::Blue, true)),
                    bgcolor: None,
                }),
            },
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        4..7 | 8..11 => {
            let item = if index + (line - 4) >= 3 {
                BRIGHTNESSES
                    .get(index + (line - 4) - 3)
                    .unwrap_or(&("", 0.0))
            } else {
                &("", 0.0)
            };
            vec![
                TextAtom {
                    content: if !item.0.is_empty() {
                        format!("{:^53}", format!("{}: {}", item.0, item.1))
                    } else {
                        format!("{:^53}", "-")
                    },
                    special_formatting: Some(TextFormatting {
                        bold: false,
                        italic: true,
                        underline: false,
                        color: Some(Color(ANSIColors::Magenta, false)),
                        bgcolor: None,
                    }),
                },
                TextAtom {
                    content: "|".into(),
                    special_formatting: None,
                },
            ]
        }
        7 => vec![
            TextAtom {
                content: format!("{:^53}", format!("Current Observation: {:0.2}", brightness)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: None,
                    bgcolor: None,
                }),
            },
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        14 => vec![text::TextAtom {
            content: TILE_FOOTER_NOSTART.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE_NOSTART.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}
