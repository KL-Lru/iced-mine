use rand::{seq::IteratorRandom, Rng};
use std::cmp;

use super::{Cell, Config};

#[derive(Debug, Clone)]
pub struct Game {
    pub field: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    pub flag_count: usize,
    pub is_game_clear: bool,
    pub is_game_over: bool,
    pub is_game_start: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    GameOver,
    GameClear,
    Playing,
}

impl Game {
    // ゲーム起動時の初期化処理
    pub fn new(config: Config) -> Self {
        let Config {
            width,
            height,
            mine_count,
        } = config;
        let mut field = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Cell::default());
            }
            field.push(row);
        }

        Game {
            field,
            width,
            height,
            mine_count,
            flag_count: 0,
            is_game_clear: false,
            is_game_over: false,
            is_game_start: false,
        }
    }

    /// ゲームフィールドの初期化処理. 最初のクリック時に呼び出す
    pub fn start_game(&mut self, _: usize, _: usize) {
        self.is_game_start = true;
        self.flag_count = 0;

        // フィールドの初期化
        let rng = &mut rand::thread_rng();
        let total = self.width * self.height;
        println!("test: {}", rng.gen::<u64>());
        let mine_points = (0..total).choose_multiple(rng, self.mine_count);
        for (x, row) in self.field.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                if mine_points.contains(&(x * self.width + y)) {
                    cell.set_mine();
                }
            }
        }

        // 周囲の地雷数を計算
        for x in 0..self.height {
            for y in 0..self.width {
                let mut mine_around = 0;
                for xi in x.saturating_sub(1)..=(cmp::min(x + 1, self.height - 1)) {
                    for yi in y.saturating_sub(1)..=(cmp::min(y + 1, self.width - 1)) {
                        // 自身はカウントしない
                        if xi == x && yi == y {
                            continue;
                        }

                        if self.field[xi][yi].is_mine {
                            mine_around += 1;
                        }
                    }
                }
                self.field[x][y].save_mine_around(mine_around);
            }
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        let target_cell = &mut self.field[x][y];

        if self.is_game_over {
            return;
        }

        if self.is_game_clear {
            return;
        }

        if target_cell.is_open {
            return;
        }

        if target_cell.is_flag {
            target_cell.unset_flag();
            self.flag_count -= 1;
        } else {
            target_cell.set_flag();
            self.flag_count += 1;
        }
    }

    pub fn open_cell(&mut self, x: usize, y: usize) -> GameState {
        let target_cell = &mut self.field[x][y];

        if self.is_game_over {
            return GameState::GameOver;
        }

        if self.is_game_clear {
            return GameState::GameClear;
        }

        if target_cell.is_open || target_cell.is_flag {
            return GameState::Playing;
        }

        target_cell.open();

        if target_cell.is_mine {
            return GameState::GameOver;
        }

        // 周囲の地雷数が0の場合は再帰的に開く
        if target_cell.mine_around == 0 {
            for xi in x.saturating_sub(1)..=(cmp::min(x + 1, self.height - 1)) {
                for yi in y.saturating_sub(1)..=(cmp::min(y + 1, self.width - 1)) {
                    // 自身はスキップ
                    if xi == x && yi == y {
                        continue;
                    }

                    if self.open_cell(xi, yi) == GameState::GameClear {
                        return GameState::GameClear;
                    }
                }
            }
        }
        if self.is_clear() {
            return GameState::GameClear;
        }
        GameState::Playing
    }

    pub fn is_clear(&self) -> bool {
        if self.is_game_over {
            return false;
        }

        let mut open_count = 0;
        for row in self.field.iter() {
            for cell in row.iter() {
                if cell.is_open {
                    open_count += 1;
                }
            }
        }

        open_count == self.width * self.height - self.mine_count
    }

    pub fn open_all_cells(&mut self) {
        for row in self.field.iter_mut() {
            for cell in row.iter_mut() {
                cell.open();
            }
        }
    }
}
