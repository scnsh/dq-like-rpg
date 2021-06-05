use crate::resources::*;
use bevy::{
    prelude::*
};
use std::fmt;
use std::fmt::Display;
use crate::events::GameEvent;

#[derive(Clone, Copy)]
pub enum RenderLayer {
    MapBackGround, // マップの背景
    MapForeGround, // マップの前景
    Player,
    BattleBackGround, // バトルの背景
    BattleForeGround, // バトルの背景
}
pub fn render_layer(layer: RenderLayer) -> usize {
    match layer {
        RenderLayer::MapBackGround => 0,
        RenderLayer::MapForeGround => 1,
        RenderLayer::Player => 2,
        RenderLayer::BattleBackGround => 3,
        RenderLayer::BattleForeGround => 4,
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn position_to_translation(
    map: &Res<Map>,
    position:  &Position,
    z: f32,
) -> Transform {
    Transform::from_translation(Vec3::new(
        (position.x as f32 + 1. / 2.)  * map.tile_size,
        (position.y as f32 + 1. / 2.) * map.tile_size,
        z,
    ))
}

pub fn position_to_field(
    map: &Res<Map>,
    point: &(i32, i32),
) -> MapField {
    match map.fields.get(point){
        Some(field) => field.clone(),
        _ => panic!()
    }
}

pub struct Player;

pub struct MapCamera;

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

impl CharacterStatus {
    pub fn enemy_text(&self) -> String {
        let ret = format!("{0} Lv {1:>2} HP {2:>3} / {3:>3} AT {4:>3} DF {5:>3}",
                          self.name, self.lv,
                          self.hp_current, self.hp_max,
                          self.attack, self.defence);
        ret
    }
}

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
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
    pub enemies: Vec<Handle<Texture>>,
    // pub atlas_loaded: bool,
}

// マップフィールドの属性
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MapField {
    Grass = 0,
    Forest,
    Mountain,
    Water,
    Town,
    Castle
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
                ret.push_str(&format!("> {:?}\n", s));
            }
            else{
                ret.push_str(&format!("  {:?}\n", s));
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