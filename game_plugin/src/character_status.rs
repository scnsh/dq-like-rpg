use crate::inventory::Inventory;
use crate::AppState;
use bevy::prelude::*;
use core::fmt;
use std::fmt::Display;

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
            defence: 10,
        }
    }
}

impl Display for CharacterStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Lv {0:>2} Exp {1:>3}\n\
                     HP {2:>3} / {3:>3}\n\
                     MP {4:>3} / {5:>3}\n\
                     AT {6:>3} DF {7:>3}\n",
            self.lv,
            self.exp,
            self.hp_current,
            self.hp_max,
            self.mp_current,
            self.mp_max,
            self.attack,
            self.defence
        )
    }
}

pub const LEVEL_LIST: [i32; 32] = [
    0, 10, 20, 40, 60, 80, 100, 125, 150, 175, 200, 225, 250, 275, 300, 325, 350, 375, 400, 425,
    450, 500, 550, 600, 650, 700, 750, 800, 850, 900, 950, 999,
];

impl CharacterStatus {
    pub fn enemy_text(&self) -> String {
        let ret = format!(
            "{0} Lv {1:>2} HP {2:>3} / {3:>3} AT {4:>3} DF {5:>3}",
            self.name, self.lv, self.hp_current, self.hp_max, self.attack, self.defence
        );
        ret
    }
    pub fn heal2max(&mut self) {
        self.hp_current = self.hp_max;
        self.mp_current = self.mp_max;
    }
    pub fn add_exp(&mut self, exp: i32, inventory: &Inventory) -> bool {
        self.exp = (&self.exp + exp).clamp(1, 999);
        let new_lv = LEVEL_LIST.iter().filter(|&&e| e <= self.exp).count() as i32;
        if self.lv != new_lv {
            self.level_up(new_lv, inventory); // レベルの更新
            self.heal2max(); //最大値まで回復
            return true;
        }
        return false;
    }
    pub fn level_up(&mut self, new_level: i32, inventory: &Inventory) {
        // 基本値の計算
        self.lv = new_level;
        self.attack = 10 + (self.lv - 1) * 5; // 攻撃力
        self.defence = 10 + (self.lv - 1) * 5; // 防御力
        self.hp_max = 100 + (self.lv - 1) * 25; // 最大HP
        self.mp_max = 100 + (self.lv - 1) * 25; // 最大MP

        // アイテムの効果を適用
        for item in inventory.items.iter() {
            match item {
                Item::IronBody => self.hp_max = (self.hp_max as f32 * 1.3) as i32,
                Item::IronArm => self.hp_max = (self.hp_max as f32 * 1.3) as i32,
                Item::IronLeg => self.hp_max = (self.hp_max as f32 * 1.3) as i32,
                Item::IronHead => self.hp_max = (self.hp_max as f32 * 1.3) as i32,
                Item::HeroSword => self.attack = (self.attack as f32 * 2.5) as i32,
                Item::WisdomRing => self.mp_max = (self.mp_max as f32 * 2.5) as i32,
                Item::FairyShield => self.defence = (self.defence as f32 * 2.5) as i32,
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
