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
    GameClear,
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
                    GameEvent::TownArrived(item, visited) => {
                        if *visited {
                            format!("Town\nGet healed up your HP!\n")
                        }else{
                            format!("Town\nGet healed up your HP!\nGet a {:?}!", item)
                        }
                    },
                    GameEvent::Win(levelup) => {
                        if *levelup {
                            return format!("You Win!\nLevel Up!\n");
                        }
                        return format!("You Win!\n");
                    },
                    GameEvent::Lose => {
                        format!("You Lose!\n")
                    },
                    GameEvent::WinLast => {
                        format!("You won the last battle!\nYou saved the kingdom!")
                    },
                    _ => panic!("unexpected event"),
                }
            }
        }
    }
}