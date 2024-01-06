use iced::{
    alignment,
    theme::Button,
    widget::{button, svg, text},
    Color, Element, Length, Theme,
};

use crate::{app::Message, models::Cell};

const CELL_SIZE: u16 = 50;
const CELL_AROUND_MAX: u16 = 8;
static MANIFEST_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub fn mine_cell(x: usize, y: usize, c: &Cell, is_game_over: bool) -> Element<'_, Message> {
    button(mine_label(c, is_game_over))
        .height(CELL_SIZE)
        .width(CELL_SIZE)
        .on_press(Message::ClickCell(x, y))
        .style(cell_style(c))
        .into()
}

fn cell_style(c: &Cell) -> <Theme as iced::widget::button::StyleSheet>::Style {
    match c.is_open {
        true => match c.is_mine {
            true => Button::Destructive,
            false => Button::Secondary,
        },
        false => Button::Primary,
    }
}

fn mine_label(c: &Cell, is_game_over: bool) -> Element<'_, Message> {
    return match (c.is_open, c.is_mine, c.is_flag, is_game_over) {
        (true, true, _, true) => svg_cell("resources/explosion.svg"),
        (true, true, _, false) => svg_cell("resources/bomb.svg"),
        (_, _, true, _) => svg_cell("resources/flag.svg"),
        (false, _, _, _) => empty_cell(),
        (true, _, _, _) => match c.mine_around {
            0 => empty_cell(),
            _ => number_cell(c.mine_around),
        },
    };
}

fn svg_cell(path: &str) -> Element<'_, Message> {
    svg(svg::Handle::from_path(format!(
        "{}/{}",
        MANIFEST_ROOT, path
    )))
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn empty_cell<'c>() -> Element<'c, Message> {
    text("")
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center)
        .into()
}

fn number_cell<'c>(u: usize) -> Element<'c, Message> {
    let base = u + 1;
    let r = (base as f32) / (CELL_AROUND_MAX as f32);
    let b = 1.0 - r;

    text(u.to_string())
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center)
        .style(Color::from_rgb(r, 0.0, b))
        .into()
}
