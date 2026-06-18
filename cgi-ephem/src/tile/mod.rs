pub mod brightness;
pub mod distance;
pub mod location;
pub mod phase;

use crate::text::{self, TextAtom};

pub const EMPTY_LINE: &str = "|                                                     |";
pub const EMPTY_LINE_NOSTART: &str = "                                                     |";
pub const TILE_FOOTER: &str = "+-----------------------------------------------------+";
pub const TILE_FOOTER_NOSTART: &str = "-----------------------------------------------------+";

pub fn na_start_tile(line: usize, title: String) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("|{:-^53}+", title),
            special_formatting: None,
        }],
        7 => vec![text::TextAtom {
            content: format!("|{:^53}|", "N/A"),
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

pub fn na_nostart_tile(line: usize, title: String) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("{:-^53}+", title),
            special_formatting: None,
        }],
        7 => vec![text::TextAtom {
            content: format!("{:^53}|", "N/A"),
            special_formatting: None,
        }],
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
