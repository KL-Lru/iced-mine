#[derive(Debug, Clone)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 10,
            height: 10,
            mine_count: 10,
        }
    }
}
