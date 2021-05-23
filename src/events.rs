use crate::resources::Enemy;

pub enum GameEvent {
    // 戦闘開始のイベント
    EnemyEncountered(Enemy),

    // 街に到着
    TownArrived,

    // プレイヤーが移動
    PlayerMoved,

    // エンティティが動いた時に実行される
    // EntityMoved(EntityId),

    // // 箱がスポットに置かれた or 置かれてないの時に実行される
    // BoxPlacedOnSpot(IsCorrectSpot),
}