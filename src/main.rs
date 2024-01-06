use iced::{Application, Settings};

mod app;
mod models;
mod views;

use app::App;

fn main() {
    App::run(Settings::default()).expect("Failed to start app");
}
