// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/icons.toml
// 818f4fa9396b7ba5f0240ece6968234500d7c45e9ac0fad9b6d1792827c0a6d6
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

pub fn globe<'a>() -> Text<'a> {
    icon("\u{1F30E}")
}

pub fn paper_plane<'a>() -> Text<'a> {
    icon("\u{F1D8}")
}

pub fn plus<'a>() -> Text<'a> {
    icon("\u{2B}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{F1F8}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("icons"))
}
