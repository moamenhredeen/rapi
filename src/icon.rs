// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/icons.toml
// 8bdb99edeaacf06c984b1e78e900e54a87064f3d317c73867907b81eb2621cdf
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/icons.ttf");

pub fn archive<'a>() -> Text<'a> {
    icon("\u{F1C6}")
}

pub fn cancel<'a>() -> Text<'a> {
    icon("\u{2715}")
}

pub fn cog<'a>() -> Text<'a> {
    icon("\u{2699}")
}

pub fn doc<'a>() -> Text<'a> {
    icon("\u{1F4C4}")
}

pub fn down_open<'a>() -> Text<'a> {
    icon("\u{E75C}")
}

pub fn folder<'a>() -> Text<'a> {
    icon("\u{1F4C1}")
}

pub fn globe<'a>() -> Text<'a> {
    icon("\u{1F30E}")
}

pub fn paper_plane<'a>() -> Text<'a> {
    icon("\u{F1D8}")
}

pub fn plus<'a>() -> Text<'a> {
    icon("\u{2B}")
}

pub fn right_open<'a>() -> Text<'a> {
    icon("\u{E75E}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{F1F8}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("icons"))
}
