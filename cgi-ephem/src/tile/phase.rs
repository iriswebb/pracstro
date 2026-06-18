use crate::text::TextFormatting;
use crate::text::{self, TextAtom};
use crate::text::{ANSIColors, Color};
use crate::tile::*;
use ephem_core::value::CelestObj;

pub fn color_of(obj: &CelestObj) -> Color {
    match obj {
        CelestObj::Moon => Color(ANSIColors::White, false),
        CelestObj::Planet(p) => match p.name {
            "Venus" | "Pluto" => Color(ANSIColors::White, true),
            "Mars" => Color(ANSIColors::Red, false),
            "Jupiter" | "Saturn" => Color(ANSIColors::Yellow, false),
            "Uranus" => Color(ANSIColors::Cyan, true),
            "Neptune" => Color(ANSIColors::Blue, false),
            "Mercury" => Color(ANSIColors::White, false),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

/// Generates a slice of a crecent at a phase angle
///
/// Loose adaptation from phoon's source code at <https://www.acme.com/software/phoon/>
pub fn crecent_slice(phaseangle: f64, line: usize, aspect_ratio: f64, numlines: usize) -> String {
    let mcap = -(phaseangle.cos());
    let yrad = (numlines as f64) / 2.0;
    let xrad = yrad / aspect_ratio;

    // Compute Edges of Slice
    let y = (line as f64) + 0.5 - yrad;
    let mut xright = xrad * (1.0 - (y * y) / (yrad * yrad)).sqrt();
    let mut xleft = -xright;

    if (0.0..std::f64::consts::PI).contains(&phaseangle) {
        xleft *= mcap;
    } else {
        xright *= mcap;
    }

    let colleft = ((xrad + 0.5) as isize) + ((xleft + 0.5) as isize);
    let colright = ((xrad + 0.5) as isize) + ((xright + 0.5) as isize);

    let mut result = String::new();

    (0..(colleft as usize)).for_each(|_| result.push(' '));
    ((colleft as usize)..=(colright as usize)).for_each(|_| result.push('@'));

    result
}

pub fn phase_tile(phaseangle: f64, line: usize, obj: &CelestObj) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("{:-^53}+", " Phase "),
            special_formatting: None,
        }],
        7 => vec![
            text::TextAtom {
                content: format!("{:<23}", crecent_slice(phaseangle, line - 2, 0.5, 10)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(color_of(obj)),
                    bgcolor: None,
                }),
            },
            text::TextAtom {
                content: format!(
                    "{:^30}",
                    crate::value::Value::Phase(
                        pracstro::time::Angle::from_radians(phaseangle),
                        crate::value::PhaseView::Default(true)
                    )
                    .to_string()
                ),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: None,
                    bgcolor: None,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        2..12 => vec![
            text::TextAtom {
                content: format!("{:<53}", crecent_slice(phaseangle, line - 2, 0.5, 10)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(color_of(obj)),
                    bgcolor: None,
                }),
            },
            text::TextAtom {
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
