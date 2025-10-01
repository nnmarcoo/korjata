use iced::{widget::{scrollable, text}, Element, Length::Fill};

use crate::app::Message;

pub fn data_panel<'a>(exif: String) -> Element<'a, Message> {
  scrollable(text(exif)).height(Fill).into()
}
