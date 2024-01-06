use crate::{
    app::Message,
    models::{Game, Mode},
};
use iced::{
    widget::{button, column, row, svg, text},
    Element,
};
static MANIFEST_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub fn control_pane<'c>(game: &'c Game, mode: &'c Mode) -> Element<'c, Message> {
    let current_state = match (game.is_game_over, game.is_game_clear) {
        (_, true) => "Clear",
        (true, _) => "Game Over",
        (_, _) => "Playing",
    };

    let current_mode = match mode {
        Mode::Dig => svg_cell("resources/pickle.svg"),
        Mode::Flag => svg_cell("resources/flag.svg"),
    };
    row(vec![
        column(vec![
            text(format!("Mines: {}", game.mine_count)).into(),
            text(format!("Flags: {}", game.flag_count)).into(),
            text(format!("Remain: {}", game.mine_count - game.flag_count)).into(),
        ])
        .align_items(iced::Alignment::Start)
        .width(100)
        .into(),
        column(vec![
            text(format!("Game State: {:}", current_state)).into(),
            current_mode,
        ])
        .align_items(iced::Alignment::Center)
        .width(180)
        .into(),
        row(vec![reset_button(), dig_button(), flag_button()])
            .align_items(iced::Alignment::End)
            .spacing(10)
            .width(180)
            .into(),
    ])
    .align_items(iced::Alignment::Center)
    .spacing(20)
    .into()
}

fn reset_button<'c>() -> Element<'c, Message> {
    button("New Game").on_press(Message::InitGame).into()
}

fn dig_button<'c>() -> Element<'c, Message> {
    button(svg_cell("resources/pickle.svg"))
        .on_press(Message::ChangeMode(Mode::Dig))
        .height(30)
        .width(30)
        .into()
}

fn flag_button<'c>() -> Element<'c, Message> {
    button(svg_cell("resources/flag.svg"))
        .on_press(Message::ChangeMode(Mode::Flag))
        .height(30)
        .width(30)
        .into()
}

fn svg_cell(path: &str) -> Element<'_, Message> {
    svg(svg::Handle::from_path(format!(
        "{}/{}",
        MANIFEST_ROOT, path
    )))
    .width(30)
    .height(30)
    .into()
}
