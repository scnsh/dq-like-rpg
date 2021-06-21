use crate::resources::*;
use bevy::{
    prelude::*
};
use std::fmt;
use std::fmt::Display;
use crate::events::GameEvent;
use std::collections::HashMap;
use bevy_kira_audio::{AudioChannel, AudioSource};

#[derive(Clone, Copy)]
pub enum RenderLayer {
    MapBackGround, // マップの背景
    MapForeGround, // マップの前景
    Player,
    BattleBackGround, // バトルの背景
    BattleForeGround, // バトルの前景
    BattleEffect,     // バトルのエフェクト
}
pub fn render_layer(layer: RenderLayer) -> usize {
    match layer {
        RenderLayer::MapBackGround => 0,
        RenderLayer::MapForeGround => 1,
        RenderLayer::Player => 2,
        RenderLayer::BattleBackGround => 3,
        RenderLayer::BattleForeGround => 4,
        RenderLayer::BattleEffect => 100,
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn position_to_translation(
    map: &Res<Map>,
    position:  &Position,
    z: f32,
) -> Transform {
    Transform::from_translation(Vec3::new(
        (position.x + 1. / 2.)  * map.tile_size,
        (position.y + 1. / 2.) * map.tile_size,
        z,
    ))
}

pub fn position_to_field(
    map: &Res<Map>,
    point: &Position,
) -> MapField {
    match map.fields.get(&(point.x as i32, point.y as i32)){
        Some(field) => field.clone(),
        _ => panic!()
    }
}

// バトルの状態
#[derive(Debug)]
pub enum PlayerBattleState {
    Select, // プレイヤーの入力待ち
    Attack, // プレイヤー攻撃中
    Defense, // 相手の攻撃中
}

pub struct Player{
    pub battle_state: PlayerBattleState,
}

// システムラベル(SystemLabel)
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerMovement {
    Input,
    Movement,
    // Eating,
    // Growth,
}

#[derive(Debug)]
pub enum MapCameraState{
    Stop,
    Moving,
}

pub struct MapCamera{
    // どちらにむかう入力が入っているかを保持する
    pub direction: MoveDirection,
    pub destination: Position,
    pub state: MapCameraState,
}
impl Default for MapCamera {
    fn default() -> Self {
        MapCamera {
            direction: MoveDirection::None, // 開始時は向きなし
            destination: Position{x:0., y:0.}, // 開始時は下向き
            state: MapCameraState::Stop,
        }
    }
}


pub struct BattleCamera;

// タイトル画面UIのルート
pub struct UiTitleRoot;

// ゲーム画面のUIのルート
pub struct UiRoot;

// バトル画面のUIに使うエンティティ
pub struct UiBattle;

// マップ画面のUIに使うエンティティ
pub struct UiMap;

// イベント画面のUIに使うエンティティ
pub struct UiEvent;

// ステータス画面のテキスト
pub struct UiStatusPlayerText;
// 敵画面のテキスト
pub struct UiStatusEnemyText;
// インベントリ画面のテキスト
pub struct UiStatusInventoryText;
// イベント画面のテキスト
pub struct UiEventText;



// キャラクターのステータス
#[derive(Clone)]
pub struct CharacterStatus {
    pub name: String,
    pub lv: i32,
    pub exp: i32,
    pub hp_current: i32,
    pub hp_max: i32,
    pub mp_current: i32,
    pub mp_max: i32,
    pub attack: i32,
    pub defence: i32,
}

impl Default for CharacterStatus {
    fn default() -> Self {
        CharacterStatus {
            name: "You".to_string(),
            lv: 1,
            exp: 0,
            hp_current: 100,
            hp_max: 100,
            mp_current: 100,
            mp_max: 100,
            attack: 10,
            defence: 10
        }
    }
}

impl Display for CharacterStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Lv {0:>2} Exp {1:>3}\n\
                     HP {2:>3} / {3:>3}\n\
                     MP {4:>3} / {5:>3}\n\
                     AT {6:>3} DF {7:>3}\n",
               self.lv, self.exp, self.hp_current, self.hp_max,
               self.mp_current, self.mp_max, self.attack, self.defence)
    }
}

pub const LEVEL_LIST: [i32; 32] = [0, 10, 20, 40, 60, 80, 100, 125, 150, 175, 200, 225, 250,
    275, 300, 325, 350, 375, 400, 425, 450, 500, 550, 600, 650, 700, 750, 800, 850, 900, 950, 999];

impl CharacterStatus {
    pub fn enemy_text(&self) -> String {
        let ret = format!("{0} Lv {1:>2} HP {2:>3} / {3:>3} AT {4:>3} DF {5:>3}",
                          self.name, self.lv,
                          self.hp_current, self.hp_max,
                          self.attack, self.defence);
        ret
    }
    pub fn heal2max(&mut self) {
        self.hp_current = self.hp_max;
        self.mp_current = self.mp_max;
    }
    pub fn add_exp(&mut self, exp: i32, inventory: &Inventory) -> bool{
        self.exp = (&self.exp + exp).clamp(1, 999);
        let new_lv = LEVEL_LIST.iter().filter(|&&e| e <= self.exp).count() as i32;
        if self.lv != new_lv {
            self.level_up(new_lv, inventory); // レベルの更新
            self.heal2max(); //最大値まで回復
            return true;
        }
        return false;
    }
    pub fn level_up(&mut self, new_level: i32, inventory: &Inventory){
        // 基本値の計算
        self.lv = new_level;
        self.attack = 10 + (self.lv - 1) * 5;  // 攻撃力
        self.defence = 10 + (self.lv - 1) * 5; // 防御力
        self.hp_max = 100 + (self.lv - 1) * 25; // 最大HP
        self.mp_max = 100 + (self.lv - 1) * 25; // 最大MP

        // アイテムの効果を適用
        for item in inventory.items.iter() {
            match item {
                Item::IronBody => {self.hp_max = (self.hp_max as f32 * 1.3) as i32}
                Item::IronArm => {self.hp_max  = (self.hp_max as f32 * 1.3) as i32}
                Item::IronLeg => {self.hp_max = (self.hp_max as f32 * 1.3) as i32}
                Item::IronHead => {self.hp_max = (self.hp_max as f32 * 1.3) as i32}
                Item::HeroSword => {self.attack = (self.attack as f32 * 2.5) as i32}
                Item::WisdomRing => {self.mp_max = (self.mp_max as f32 * 2.5) as i32}
                Item::FairyShield => {self.defence = (self.defence as f32 * 2.5) as i32}
                _ => {}
            }
        }

        // 範囲に収める
        self.attack = self.attack.clamp(1, 999);
        self.defence = self.defence.clamp(1, 999);
        self.hp_max = self.hp_max.clamp(1, 999);
        self.mp_max = self.mp_max.clamp(1, 999);
        self.hp_current = self.hp_current.clamp(1, 999);
        self.mp_current = self.mp_current.clamp(1, 999);
    }
}

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}

// #[derive(Default)]
// pub struct Render {
//     pub sprite_index: usize,
//     pub z_order: usize,
// }

// スプライトのハンドル集合
// 全てのスプライトのロードが終わったかを確認する
#[derive(Default, Clone)]
pub struct AssetHandles {
    pub tilemap: Handle<Texture>,
    pub player: Handle<Texture>,
    pub battle_background: Handle<Texture>,
    pub battle_effects: HashMap<EffectKind, (Handle<Texture>, usize)>,
    pub enemies: Vec<Handle<Texture>>,
    // pub atlas_loaded: bool,
}

// マップフィールドの属性
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MapField {
    Grass,
    Forest,
    Mountain,
    Water,
    Town{item: Item, visited: bool},
    Castle
}
impl MapField {
    pub fn sprint_index(&self) -> usize {
        match &self {
            MapField::Grass => 0,
            MapField::Forest => 1,
            MapField::Mountain => 2,
            MapField::Water => 3,
            MapField::Town{ item, visited} => 4,
            MapField::Castle => 5,
        }
    }
}

// 持ち物
pub struct Inventory{
    pub items: Vec<Item>,
    pub skills: Vec<Skill>,
    pub selected_skill_index: usize
}
impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: Vec::new(),
            skills: vec![Skill::Sword],
            selected_skill_index: 0
        }
    }
}
impl Display for Inventory {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Items:\n{0}",
               self.items.iter().map(|s| format!("  {}", s)).collect::<Vec<_>>().join("\n"))
    }
}

impl Inventory {
    pub fn skill_list(&self) -> String {
        let mut ret = String::new();
        for (i, s) in self.skills.iter().enumerate(){
            if i == self.selected_skill_index {
                ret.push_str(&format!("> {}\n", s));
            }
            else{
                ret.push_str(&format!("  {}\n", s));
            }
        }
        ret
        // formart!("{0}", self.items.iter().map(|s| format!("> {}", s)).collect::<Vec<_>>().join("\n"))
    }
    pub fn add_item(&mut self, item: Item){
        // アイテムをインベントリに追加
        self.items.push(item);
        // スキルアイテムはスキルリストにも追加
        if let Some(skill) = item.can_use(){
            self.skills.push(skill);
        }
    }
    pub fn increment_index(&mut self){
        self.selected_skill_index = (&self.selected_skill_index + 1).clamp(0, self.skills.len() - 1);
    }
    pub fn decrement_index(&mut self){
        if self.selected_skill_index > 0 {
            self.selected_skill_index = (&self.selected_skill_index - 1).clamp(0, self.skills.len() - 1);;
        }
    }
    pub fn skill(&self) -> Skill{
        self.skills[self.selected_skill_index]
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum EffectKind {
    Attack,
    Heal,
    Fire,
    Ice,
    Death,
    Arrow,
    Wind,
}
pub struct EffectSpawnEvent {
    pub kind: EffectKind,
    pub damage_or_heal: i32,
    pub is_player_attack: bool,
}

pub fn skill_to_effect(skill: Skill) -> EffectKind{
    match skill {
        Skill::Sword => EffectKind::Attack,
        Skill::Wind => EffectKind::Wind,
        Skill::Arrow => EffectKind::Arrow,
        Skill::Death => EffectKind::Death,
        Skill::Spell(item) => {
            match item {
                Item::SpellHeal(lv) => EffectKind::Heal,
                Item::SpellFire(lv) => EffectKind::Fire,
                Item::SpellIce(lv) => EffectKind::Ice,
                _ => panic!("select item cannot use.")
            }
        },
    }
}

pub struct EffectString;

pub struct AudioState {
    pub audio_loaded: bool,
    pub sound_handles: HashMap<AudioKind, Handle<AudioSource>>,
    pub channels: HashMap<String, (AudioChannel, ChannelAudioState)>,
}
impl Default for AudioState {
    fn default() -> Self {
        AudioState{
            audio_loaded: false,
            sound_handles: HashMap::new(),
            channels: HashMap::new()
        }
    }
}
impl AudioState {
    pub fn get_channel(&mut self, kind: &AudioKind) -> Option<&mut (AudioChannel, ChannelAudioState)> {
        match kind {
            AudioKind::SEAttack | AudioKind::SEHeal | AudioKind::SETown => {
                self.channels.get_mut("se")
            },
            _ => {
                self.channels.get_mut("bgm")

            }
        }
    }
}

pub struct ChannelAudioState {
    pub stopped: bool,
    pub paused: bool,
    pub loop_started: bool,
    pub volume: f32,
}

impl Default for ChannelAudioState {
    fn default() -> Self {
        ChannelAudioState {
            volume: 1.0,
            stopped: true,
            loop_started: false,
            paused: false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AudioKind {
    BGMMap,
    BGMBattle,
    BGMBattleLast,
    BGMWin,
    BGMLose,
    SEAttack,
    SEHeal,
    SETown,
}

#[derive(Debug)]
pub enum AudioEvent {
    Play(AudioKind),
    Pause(AudioKind),
    Stop(AudioKind)
}

pub struct TileMap;