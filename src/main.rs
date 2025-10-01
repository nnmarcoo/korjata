use crate::app::Korjata;
use iced::{Font, Theme, application};
use rand::{rng, seq::IndexedRandom};

mod app;
mod comps;
mod core;

fn main() -> iced::Result {
    let theme = random_theme();

    application("korjata", Korjata::update, Korjata::view)
        .theme(move |_| theme.clone())
        .default_font(Font::MONOSPACE)
        .centered()
        .subscription(Korjata::subscription)
        .run()
}

fn random_theme() -> Theme {
    let mut rng = rng();

    let themes = vec![
        Theme::Light,
        Theme::Dark,
        Theme::Dracula,
        Theme::Nord,
        Theme::SolarizedLight,
        Theme::SolarizedDark,
        Theme::GruvboxLight,
        Theme::GruvboxDark,
        Theme::CatppuccinLatte,
        Theme::CatppuccinFrappe,
        Theme::CatppuccinMacchiato,
        Theme::CatppuccinMocha,
        Theme::TokyoNight,
        Theme::TokyoNightStorm,
        Theme::TokyoNightLight,
        Theme::KanagawaWave,
        Theme::KanagawaDragon,
        Theme::KanagawaLotus,
        Theme::Moonfly,
        Theme::Nightfly,
        Theme::Oxocarbon,
        Theme::Ferra,
    ];

    themes.choose(&mut rng).unwrap().clone()
}
