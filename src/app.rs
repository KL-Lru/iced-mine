use iced::{
    widget::{column, container},
    Application, Command, Element, Theme,
};

use crate::{
    models::{Config, Game, GameState, Mode},
    views::{control_pane::control_pane, game_field::game_field},
};

pub struct App {
    pub game: Game,
    pub config: Config,
    pub mode: Mode,
}

impl App {
    pub fn new() -> Self {
        let config = Config::default();

        App {
            game: Game::new(config.clone()),
            config,
            mode: Mode::Dig,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ClickCell(usize, usize),
    InitGame,
    ChangeMode(Mode),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (App, iced::Command<Self::Message>) {
        (App::new(), iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Mine Sweeper")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::InitGame => {
                self.game = Game::new(self.config.clone());
            }

            Message::ChangeMode(mode) => {
                self.mode = mode;
            }

            Message::ClickCell(x, y) => {
                if !self.game.is_game_start {
                    self.game.start_game(x, y);
                }

                if self.game.is_game_over {
                    return Command::none();
                }

                match self.mode {
                    Mode::Dig => {
                        match self.game.open_cell(x, y) {
                            GameState::GameOver => {
                                self.game.is_game_over = true;
                                self.game.open_all_cells();
                            }
                            GameState::GameClear => {
                                self.game.is_game_clear = true;
                                self.game.open_all_cells();
                            }
                            GameState::Playing => {}
                        };
                    }
                    Mode::Flag => {
                        self.game.toggle_flag(x, y);
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let field = game_field(&self.game);
        let control = control_pane(&self.game, &self.mode);
        container(column(vec![control, field]))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }
}
