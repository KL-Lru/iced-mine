use iced::{
    widget::{column, row},
    Element,
};

use crate::{app::Message, models::Game};

pub fn game_field(game: &Game) -> Element<'_, Message> {
    column(
        game.field
            .iter()
            .enumerate()
            .map(|(x, r)| {
                row(r
                    .iter()
                    .enumerate()
                    .map(|(y, c)| crate::views::mine_cell::mine_cell(x, y, c, game.is_game_over))
                    .collect::<Vec<Element<'_, _, _>>>())
                .into()
            })
            .collect::<Vec<Element<'_, _, _>>>(),
    )
    .into()
}
