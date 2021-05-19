// ゲームの状態
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Title,
    Loading,
    Generating,
    MapView,
    BattleView,
    // GameOver,
}

// 初期状態
impl Default for GameState {
    fn default() -> Self {
        GameState::Title
    }
}