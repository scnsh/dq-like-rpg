use crate::components::CharacterStatus;
use crate::resources::{Enemy, Item};
use bevy::ecs::entity::Entity;
use bevy::math::Vec2;
use bevy::text::Text;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct Battle {
    // pub state: BattleState,
    pub enemy: Enemy,
    pub entity: Option<Entity>,
    pub enemy_status: Option<CharacterStatus>,
    pub ui_status_text: Option<Text>,
    pub enemy_root_offset: Vec2, // pub ui_entity: Option<Entity>,
}

#[derive(Clone, Copy, Debug)]
pub enum Skill {
    Sword,
    Spell(Item),
    Arrow,
    Wind,
    Death,
}

impl Display for Skill {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Skill::Spell(item) => {
                write!(f, "{}", item)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}
