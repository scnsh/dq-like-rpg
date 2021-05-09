use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use rand::prelude::random; // 食べ物をランダムに配置するために使う

// グリッドサイズ
const ARENA_HEIGHT: u32 = 10;
const ARENA_WIDTH: u32 = 10;

// 方向
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

// システムラベル(SystemLabel)
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

///
/// component
///

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position{
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
// Sizeのヘルパーメソッド、全てのオブジェクトの縦横は同じ
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

struct SnakeHead {
    // どちらにむかっているかを保持する
    direction: Direction,
}


struct SnakeSegment;
// 体(セグメント)のリストを保存する
#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

// 空の構造体、タグのようなもので検索するときに使う
struct Food;

// 送信・受信可能なイベント
struct GrowthEvent;

//末尾の尻尾位置
#[derive(Default)]
struct LastTailPosition(Option<Position>);

struct GameOverEvent;


// リソース、マテリアルを保持して、頭や体、餌などにも使う
struct Materials {
    head_material: Handle<ColorMaterial>,
    segment_material: Handle<ColorMaterial>,
    food_material: Handle<ColorMaterial>,
}

///
/// system
///

// 起動時のセットアップ
fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<ColorMaterial>>)
{
    // 2D用カメラを追加する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // マテリアルを作成する
    // Materials の型で以降リソースにアクセスするとこの構造体を生成して返す
    commands.insert_resource(Materials {
        // materials.add が Handle<ColorMaterial> を返す
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into())
    });
}

// snakeの生成
fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    // Materials型を検索して、SnakeHead と SpriteBundle のコンポーネントを持つ、新しいエンティティを生成する
    // SpriteBundleを作るために、カラーマテリアルを制御するハンドルとスプライトのサイズを渡す
    segments.0 = vec![
        //1つ目のセグメントは頭
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.head_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            })
            .insert(SnakeHead{
                direction: Direction::Up, // 開始時は上向き
            })
            .insert(SnakeSegment)
            // 場所を指定する
            .insert(Position {x: 3, y: 3})
            .insert(Size::square(0.8))
            .id(),
        //2つ目は spawn_segmentから作られる
        spawn_segment(
            commands,
            &materials.segment_material,
            Position { x: 3, y: 2},
        ),
    ];
}

// segumentの生成(食べた時とヘビの初期化時に呼ばれる)
// Entityをid()を使うことで返り値として取得する、
fn spawn_segment(
    mut commands: Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

// Spriteのサイズを調整
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    // windowサイズを取得
    let window = windows.get_primary().unwrap();
    // windowサイズに合わせて、スプライトのサイズを調整
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

// 位置を移動させる
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>){
    // 原点位置を左下としているが、移動は中央から始まる
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>)
{
    //
    if let Some(mut head) = heads.iter_mut().next(){
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        }else{
            head.direction // 入力がなければそのまま
        };
        // 正反対方向の入力ではないことを確認
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

// snakeの移動
// Query型を使うと、SnakeHeadコンポーネントとTransformコンポーネントを持つエンティティをイテレーティブに使用できる
// Queryの生成はECSがやってくれる
fn snake_movement(
    segments: ResMut<SnakeSegments>, // 頭を含まない
    // SnakeHeadは使わないので、Withを使ってSnakeHeadを含む条件を追加する書き方で書くことで並列性をあげる
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    // SnakeHead が Transform を持つ理由は、SpriteBundle が Bundleコンポーネントであり、Transformを持つから
    // Transform ではなくて、Positionを使って移動を制御する
    // QueryからPositionではなく、Entityを取得する
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // positionsのQueryから SnakeSegmentsに含まれるsegments を iterative に実行して、各Positionを取得する
        let segment_positions = segments
            .0
            .iter()
            // 指定したentityのPositionを取り出す
            .map(|e| *positions.get_mut(*e).unwrap())
            // 上で取り出したPositionをVecとして集める
            .collect::<Vec<Position>>();
        // head の位置を取得する
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        // ゲームオーバーの判定(範囲外)
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }
        // ゲームオーバーの判定(体に当たる)
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        // 各segmentに位置を設定する
        // 頭の位置を更新したら、各segmentの位置をその前のsegmentの位置に合わせる
        // 例えば、最初のsegmentの位置は頭の位置、2番目のsegmentの位置は最初の頭の位置など
        segment_positions
            .iter()
            // 各位置とsegments(Entity) とペアを作る(segmentを1つずらす)
            .zip(segments.0.iter().skip(1))
            // 各要素の positions の位置を更新する
            .for_each(|(pos, segment)| {
               *positions.get_mut(*segment).unwrap() = *pos;
            });
        // 最後のsegmentの位置を割り当てる, segment_positions の更新の後で実行する
        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

// 蛇の食べる処理
fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
){
    for head_pos in head_positions.iter(){
        for (ent, food_pos) in food_positions.iter(){
            // 全ての食べ物の位置を探索して、頭の位置と一致するかを判定する
            if food_pos == head_pos {
                // 食べ物を削除
                commands.entity(ent).despawn();
                // 成長イベントを発生させる
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<Materials>
){
    if growth_reader.iter().next().is_some() {
        // 最後尾位置に segmentを追加
        segments.0.push(spawn_segment(
            commands,
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ));
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<Materials>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
){
    if reader.iter().next().is_some() {
        // 食べ物とsegmentを全部削除する
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        // 新しい蛇の生成もここで行う
        spawn_snake(commands, materials, segments_res);
    }
}

// 食べ物を追加する
fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
){
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.food_material.clone(),
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

///
/// main
///

fn main() {
    App::build()
        // 背景色を設定
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // windowサイズなどを設定する
        .insert_resource(WindowDescriptor{
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup.system())
        // startup_systemの後に実行するために add_startup_stage を選択、複数のシステムを使う時はparallelを使う
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(
            snake_movement_input
                .system()
                // .label() を使って、before(), after() などを他のシステムで使えるようにして、順番を定義する
                .label(SnakeMovement::Input)
                // snakeを動かす前に入力を取得するようにしている
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                // snake_movement に SnakeMovement::Movement をタグ付けする
                .with_system(snake_movement.system().label(SnakeMovement::Movement))
                .with_system(
                    // 移動後に判定する
                    snake_eating
                        .system()
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement))
                .with_system(
                    // 食事後に実行する
                    snake_growth
                        .system()
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating))
                // 移動後にゲームオーバー判定をする
                .with_system(game_over.system().after(SnakeMovement::Movement))
        )
        .add_system_set(
            // 特定の時間(1秒間隔)で発生するためにFixedTimestepを使う
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner.system()),
        )
        .add_system_set_to_stage(
            // 各ステージ後にコマンドが実行されるので、新しいエンティティが Updateで追加されたとして
            // position_translation と size_scaling が適応される前に実施したいため
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system())
        )
        .add_plugins(DefaultPlugins)
        .run();
}
