use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    Theme,
    widget::{button, container, row, text},
};
use std::path::PathBuf;

use crate::app::Message;

const FONT_SIZE: f32 = 14.;
const PADDING: f32 = 3.;
const SPACING: f32 = 5.;

pub fn bottom_row<'a>(curr_file: Option<&'a PathBuf>, width: f32) -> Element<'a, Message> {
    container(
        row![
            text(truncate_str(
                curr_file.and_then(|p| p.to_str()).unwrap_or("-"),
                width
            ))
            .width(Fill)
            .size(FONT_SIZE),
            button(text("rfd").size(FONT_SIZE)).on_press(Message::FileSelect),
        ]
        .align_y(Center)
        .spacing(SPACING)
        .padding(PADDING),
    )
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();
        container::background(palette.secondary.base.color).color(palette.secondary.base.text)
    })
    .into()
}

fn truncate_str(s: &str, width: f32) -> String {
    let char_width = FONT_SIZE * 0.7;

    let usable_width = (width - 50.).max(0.);
    let max_chars = (usable_width / char_width) as usize;

    let total_chars = s.chars().count();

    if total_chars <= max_chars {
        s.to_string()
    } else {
        s.chars().skip(total_chars - max_chars).collect()
    }
}
