use iced::{
    Element,
    Length::Fill,
    widget::{scrollable, text},
};

use crate::app::Message;

pub fn data_panel<'a>(exif: String) -> Element<'a, Message> {
    scrollable(text(exif)).height(Fill).into()
}
