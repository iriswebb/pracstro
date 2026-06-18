use crate::text::TextFormatting;
use crate::text::{self, TextAtom};
use crate::text::{ANSIColors, Color};
use crate::tile::*;
use crate::value::Value;

const ANGDIAS: [(&str, f64); 10] = [
    ("Moon (max): 34'6\"", 2046.0),
    ("Sun (max): 32'32\"", 1952.0),
    ("Sun (min): 31'27\"", 1887.0),
    ("Moon (min): 29'20\"", 1760.0),
    ("Venus (max): 1'6\"", 66.0),
    ("Human Eye Resolution: 1'0\"", 60.0),
    ("Jupiter (max): 0'50.1\"", 50.1),
    ("Mars (max): 0'25.1\"", 25.1),
    ("Uranus (max): 0'4.1\"", 4.1),
    ("Hubble Resolution: 0'0.1\"", 0.1),
];

pub fn distance_tile_noangdia(distance: f64, line: usize) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Distance "),

            special_formatting: None,
        }],
        2 => vec![
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:^53}", format!("Distance: {:.2}", Value::Dist(distance))),
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
        7 => vec![TextAtom {
            content: format!("|{:^53}|", "No Angular Diameter Available"),
            special_formatting: None,
        }],
        14 => vec![text::TextAtom {
            content: TILE_FOOTER.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}

pub fn distance_tile(
    distance: f64,
    angdia: Option<pracstro::time::Angle>,
    line: usize,
) -> Vec<TextAtom> {
    if angdia.is_none() {
        return distance_tile_noangdia(distance, line);
    }
    let angdia = angdia.unwrap();
    let mut index = 0;
    let angdiaarcsec = angdia.degrees() * 3600.0;
    for (i, x) in ANGDIAS.iter().enumerate() {
        if x.1 < angdiaarcsec {
            index = i;
            break;
        }
    }

    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Distance "),
            special_formatting: None,
        }],
        2 => vec![
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:^53}", format!("Distance: {:.2}", Value::Dist(distance))),
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
                ANGDIAS.get(index + (line - 4) - 3).unwrap_or(&("", 0.0))
            } else {
                &("", 0.0)
            };
            vec![
                TextAtom {
                    content: "|".into(),
                    special_formatting: None,
                },
                TextAtom {
                    content: if !item.0.is_empty() {
                        format!("{:^53}", item.0)
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
                content: "|".into(),
                special_formatting: None,
            },
            TextAtom {
                content: format!(
                    "{:^53}",
                    format!(
                        "Current Observation: {}",
                        Value::Ang(angdia, crate::value::AngView::Angle)
                    )
                ),
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
            content: TILE_FOOTER.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}
