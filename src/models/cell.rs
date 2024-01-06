#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub is_open: bool,
    pub is_mine: bool,
    pub is_flag: bool,
    pub mine_around: usize,
}

impl Cell {
    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn set_mine(&mut self) {
        self.is_mine = true;
    }

    pub fn set_flag(&mut self) {
        self.is_flag = true;
    }

    pub fn unset_flag(&mut self) {
        self.is_flag = false;
    }

    pub fn save_mine_around(&mut self, mine_around: usize) {
        self.mine_around = mine_around;
    }
}
