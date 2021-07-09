use crate::AppState;
use bevy::prelude::*;
use core::fmt;
use std::fmt::Display;

pub struct InventoryPlugin;

// This plugin is responsible to controll the game audio
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Inventory>();
        // .add_system_set(
        //     SystemSet::on_enter(AppState::InGameBattle)
        //         .with_system(effects_event.system())
        //         .with_system(handle_effect.system()),
        // )
        // .add_system_set(
        //     SystemSet::on_exit(AppState::InGameBattle).with_system(clean_up_effects.system()),
        // );
    }
}

// 持ち物
pub struct Inventory {
    pub items: Vec<Item>,
    pub skills: Vec<Skill>,
    pub selected_skill_index: usize,
}
impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: Vec::new(),
            skills: vec![Skill::Sword],
            selected_skill_index: 0,
        }
    }
}
impl Display for Inventory {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Items:\n{0}",
            self.items
                .iter()
                .map(|s| format!("  {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Inventory {
    pub fn skill_list(&self) -> String {
        let mut ret = String::new();
        for (i, s) in self.skills.iter().enumerate() {
            if i == self.selected_skill_index {
                ret.push_str(&format!("> {}\n", s));
            } else {
                ret.push_str(&format!("  {}\n", s));
            }
        }
        ret
        // formart!("{0}", self.items.iter().map(|s| format!("> {}", s)).collect::<Vec<_>>().join("\n"))
    }
    pub fn add_item(&mut self, item: Item) {
        // アイテムをインベントリに追加
        self.items.push(item);
        // スキルアイテムはスキルリストにも追加
        if let Some(skill) = item.can_use() {
            self.skills.push(skill);
        }
    }
    pub fn increment_index(&mut self) {
        self.selected_skill_index =
            (&self.selected_skill_index + 1).clamp(0, self.skills.len() - 1);
    }
    pub fn decrement_index(&mut self) {
        if self.selected_skill_index > 0 {
            self.selected_skill_index =
                (&self.selected_skill_index - 1).clamp(0, self.skills.len() - 1);
        }
    }
    pub fn skill(&self) -> Skill {
        self.skills[self.selected_skill_index]
    }
}
