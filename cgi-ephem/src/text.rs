//! Text Rendering for Both ANSI Terminal Escape Codes and HTML
//!
//! The largest unit of text is the line, lines are made up of atoms contain a string and optional special formatting

use html_escape;

/// The ANSI Terminal Colors that everything supports
#[derive(Clone, Copy)]
pub enum ANSIColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// An ANSIColor and brightness flag
#[derive(Clone, Copy)]
pub struct Color(pub ANSIColors, pub bool);

/// Specifiers for text formatting
#[derive(Clone, Copy)]
pub struct TextFormatting {
    pub color: Option<Color>,
    pub bgcolor: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// The basic unit of text, a string with optional formatting
#[derive(Clone)]
pub struct TextAtom {
    pub special_formatting: Option<TextFormatting>,
    pub content: String,
}

/// A set of functions for rendering formatted text in a certain format
pub struct Driver {
    pub render_atom: fn(TextAtom) -> String,
    pub cgi_header: &'static str,
    pub header: &'static str,
    pub footer: &'static str,
    pub eol: &'static str,
}

fn render_html_color(color: Color) -> &'static str {
    match color {
        Color(ANSIColors::Black, false) => "black",
        Color(ANSIColors::Black, true) => "grey",
        Color(ANSIColors::Blue, true) => "blue",
        Color(ANSIColors::Cyan, true) => "cyan",
        Color(ANSIColors::Green, false) => "DarkGreen",
        Color(ANSIColors::Cyan, false) => "MediumTurquoise",
        Color(ANSIColors::Magenta, false) => "Magenta",
        Color(ANSIColors::Magenta, true) => "Pink",
        Color(ANSIColors::Green, true) => "green",
        Color(ANSIColors::Red, false) => "crimson",
        Color(ANSIColors::White, _) => "white",
        Color(ANSIColors::Yellow, _) => "yellow",
        Color(ANSIColors::Red, true) => "red",
        Color(ANSIColors::Blue, false) => "navy",
    }
}

fn render_html_atom(atom: TextAtom) -> String {
    if let Some(formatters) = atom.special_formatting {
        let mut style: String = String::with_capacity(30);
        if formatters.bold {
            style.push_str("font-wieght: bold;");
        }
        if formatters.italic {
            style.push_str("font-style: italic;");
        }
        if formatters.underline {
            style.push_str("text-decoration: underline;");
        }
        if let Some(bgcol) = formatters.bgcolor {
            style.push_str(&format!("background-color: {};", render_html_color(bgcol)));
        }
        if let Some(color) = formatters.color {
            style.push_str(&format!("color: {};", render_html_color(color)));
        }

        format!(
            "<span style=\"{}\">{}</span>",
            style,
            html_escape::encode_text(&atom.content)
        )
    } else {
        html_escape::encode_text(&atom.content).to_string()
    }
}

pub const HTML_DRIVER: Driver = Driver {
    render_atom: render_html_atom,
    header: include_str!("dat/header_html"),
    footer: include_str!("dat/footer_html"),
    cgi_header: "Status: 200 OK\r\nContent-Type: text/html;charset=utf-8\r\n\r\n",
    eol: "<br>",
};

fn ansi_color_number(color: Color, is_bg: bool) -> u8 {
    (match (color.1, is_bg) {
        (false, false) => 30,
        (false, true) => 40,
        (true, false) => 90,
        (true, true) => 100,
    }) + (color.0 as u8)
}

fn render_ansi_atom(text: TextAtom) -> String {
    if let Some(formatters) = text.special_formatting {
        let mut style: Vec<String> = Vec::new();
        if formatters.bold {
            style.push("1".into());
        }
        if formatters.italic {
            style.push("3".into());
        }
        if formatters.underline {
            style.push("4".into());
        }
        if let Some(color) = formatters.color {
            style.push(ansi_color_number(color, false).to_string());
        }
        if let Some(bgcolor) = formatters.bgcolor {
            style.push(ansi_color_number(bgcolor, true).to_string());
        }

        format!("\x1b[{}m{}\x1b[0m", style.join(";"), text.content)
    } else {
        text.content
    }
}

pub const ANSI_DRIVER: Driver = Driver {
    render_atom: render_ansi_atom,
    header: "",
    cgi_header: "Status: 200 OK\r\nContent-Type: text/plain;charset=utf-8\r\n\r\n",
    footer: "",
    eol: "\n",
};

pub const TEXT_DRIVER: Driver = Driver {
    render_atom: render_text_atom,
    header: "",
    cgi_header: "Status: 200 OK\r\nContent-Type: text/plain;charset=utf-8\r\n\r\n",
    footer: "",
    eol: "\n",
};

pub fn render_text_atom(atom: TextAtom) -> String {
    atom.content
}
