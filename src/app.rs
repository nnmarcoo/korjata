use std::path::PathBuf;

use iced::{
    Element,
    Length::Fill,
    Size, Subscription,
    widget::{Space, column},
    window,
};
use rfd::FileDialog;

use crate::{
    comps::{bottom_row::bottom_row, data_panel::data_panel},
    core::exif::exif,
};

#[derive(Debug, Default)]
pub struct Korjata {
    exif_text: String,
    file: Option<PathBuf>,
    window_size: Size,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileSelect,
    WindowResized(Size),
}

impl Korjata {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::FileSelect => {
                if let Some(path) = FileDialog::new()
                    .add_filter("PNG image", &["jpg"])
                    .pick_file()
                {
                    self.file = Some(path.clone());

                    self.exif_text = exif(&path);
                }
            }

            Message::WindowResized(size) => {
                self.window_size = size;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            data_panel(self.exif_text.clone()), // clone dumb?
            bottom_row(self.file.as_ref(), self.window_size.width)
        ]
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_, size)| Message::WindowResized(size))
    }
}
