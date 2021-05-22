use bevy::ecs::entity::Entity;
use crate::components::CharacterStatus;

#[derive(Default)]
pub struct Battle {
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

