use crate::components::{CharacterStatus, MapField};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::array::IntoIter;
use std::fmt::Display;
use core::fmt;
use crate::resources::Skill;

#[derive(Clone, Copy, Debug)]
pub enum Enemy {
    Goblin,
    Elf,
    Bird,
    Boss,
}
impl Default for Enemy {
    fn default() -> Self { Enemy::Goblin }
}
impl Display for Enemy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EnemyStatus {
    name: Enemy,
    rate: i32,
    img: usize,
    hp: i32,
    at: i32,
    df: i32,
    skl: Skill
}

pub struct EnemyData {
    pub data: HashMap<MapField, EnemyStatus>,
}
impl Default for EnemyData {
    fn default() -> Self {
        EnemyData{
            data: HashMap::<_, _>::from_iter(IntoIter::new([
                (MapField::Grass, EnemyStatus{name: Enemy::Goblin, rate: 20, img: 0, hp: 50, at: 10, df: 5, skl: Skill::Sword}),
                (MapField::Forest, EnemyStatus{name: Enemy::Elf, rate: 10, img: 1, hp: 100, at: 20, df: 10, skl: Skill::Arrow}),
                (MapField::Mountain, EnemyStatus{name: Enemy::Bird, rate: 5, img: 2, hp: 200, at: 40, df: 30, skl: Skill::Wind}),
                (MapField::Castle, EnemyStatus{name: Enemy::Boss, rate: 1, img: 3, hp: 999, at: 99, df: 99, skl: Skill::Death}),
            ]))
        }
    }
}

impl EnemyData {
    pub fn create(&self, map_field: &MapField, level: i32) -> CharacterStatus{
        if let &enemy_status = &self.data[map_field] {
            return CharacterStatus{
                name: enemy_status.name.to_string(),
                lv: level,
                exp: 0,
                hp_current: (enemy_status.hp as f32 * (0.5 + level as f32 / 2.)) as i32,
                hp_max: (enemy_status.hp as f32 * (0.5 + level as f32 / 2.)) as i32,
                mp_current: 0,
                mp_max: 0,
                attack: (enemy_status.at as f32 * (0.5 + level as f32 / 2.)) as i32,
                defence: (enemy_status.df as f32 * (0.5 + level as f32 / 2.)) as i32,
            }
        }
        panic!("unexpected map field.")
    }
    pub fn field_to_enemy(&self, map_field: &MapField) -> Enemy {
        if let &enemy_status = &self.data[map_field] {
            return enemy_status.name;
        }
        panic!("unexpected map field.")
    }
    pub fn field_to_rate(&self, map_field: &MapField) -> i32 {
        if let &enemy_status = &self.data[map_field] {
            return enemy_status.rate;
        }
        panic!("unexpected map field.")
    }
    pub fn field_to_enemy_skill(&self, map_field: &MapField) -> Skill {
        if let &enemy_status = &self.data[map_field] {
            return enemy_status.skl;
        }
        panic!("unexpected map field.")
    }
    pub fn image_index(&self, map_field: &MapField) -> usize {
        if let enemy_status = &self.data[map_field] {
            return enemy_status.img;
        }
        panic!("unexpected map field.")
    }
}
