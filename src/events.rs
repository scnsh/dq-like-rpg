use crate::resources::{Enemy, Item};

#[derive(Debug)]
pub enum GameEvent {
    // 戦闘開始のイベント
    EnemyEncountered(Enemy),

    // 街に到着
    TownArrived(Item, bool),

    // プレイヤーが移動
    PlayerMoved,

    // 勝利
    Win(bool),

    // 敗北
    Lose,

    // 最終戦勝利
    WinLast,

    // 攻撃
    PlayerAttack,

    // エンティティが動いた時に実行される
    // EntityMoved(EntityId),

    // // 箱がスポットに置かれた or 置かれてないの時に実行される
    // BoxPlacedOnSpot(IsCorrectSpot),
}