use iced::{Element, Size, Subscription, widget::column, window};
use rfd::FileDialog;

use crate::{
    comps::{bottom_row::bottom_row, data_panel::data_panel},
    core::media::Media,
};

#[derive(Debug, Default)]
pub struct Korjata {
    exif_text: String,
    media: Media,
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
                    .add_filter("JPG image", &["jpg"])
                    .pick_file()
                {
                    if let Ok(media) = Media::from_file(path) {
                        self.media = media;
                    }

                    println!("{:#?}", self.media.segments());
                    self.exif_text = String::new(); //exif(&path);
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
            bottom_row(self.media.path_string(), self.window_size.width)
        ]
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_, size)| Message::WindowResized(size))
    }
}
