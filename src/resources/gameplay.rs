use crate::events::GameEvent;

/// 1つの状態でのみ必要とされるエンティティにタグを付けるコンポーネント
pub struct ForState<T> {
    pub states: Vec<T>,
}

// ゲームの状態
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Title,
    Loading,
    Generating,
    Map,
    Battle,
    Event,
    // GameOver,
}

// 初期状態
impl Default for GameState {
    fn default() -> Self {
        GameState::Title
    }
}

#[derive(Debug, Default)]
pub struct RunState {
    pub event: Option<GameEvent>,
}

impl RunState {
    // pub fn new() -> RunState {
    //     RunState {
    //         event: None,
    //     }
    // }

    pub fn event_text(&self) -> String {
        match &self.event {
            None => panic!("can't convert text from None."),
            Some(event) => {
                match event {
                    GameEvent::EnemyEncountered(enemy) => {
                        format!("Battle!!!\n{0:?} appeared.\n", enemy)
                    },
                    GameEvent::TownArrived => {
                        format!("Town\nGet healed up your HP!\nGet a item\n")
                    },
                    _ => panic!("unexpected event"),
                }
            }
        }
    }
}