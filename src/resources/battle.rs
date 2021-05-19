use bevy::ecs::entity::Entity;

#[derive(Default)]
pub struct Battle {
    pub entity: Option<Entity>,
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

