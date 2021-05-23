use bevy::ecs::entity::Entity;
use crate::components::CharacterStatus;
use crate::resources::Enemy;

#[derive(Default)]
pub struct Battle {
    pub enemy: Enemy,
    pub entity: Option<Entity>,
    pub enemy_status: Option<CharacterStatus>,
    // pub ui_entity: Option<Entity>,
}

// // 初期状態
// impl Default for Battle {
//     fn default() -> Self {
//         Battle{
//             entity: None,
//             ui_entity: None,
//         }
//     }
// }

