use crate::components::{CharacterStatus, MapField};

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

pub fn create_enemy(enemy: Enemy) -> CharacterStatus{
    match enemy {
        Enemy::Goblin => CharacterStatus{
            name: "Goblin".to_string(),
            lv: 1,
            exp: 0,
            hp_current: 50,
            hp_max: 50,
            mp_current: 0,
            mp_max: 0,
            attack: 10,
            defence: 10,
        },
        Enemy::Elf => CharacterStatus{
            name: "Elf".to_string(),
            lv: 1,
            exp: 0,
            hp_current: 100,
            hp_max: 100,
            mp_current: 0,
            mp_max: 0,
            attack: 30,
            defence: 30,
        },
        Enemy::Bird => CharacterStatus{
            name: "Bird".to_string(),
            lv: 1,
            exp: 0,
            hp_current: 200,
            hp_max: 200,
            mp_current: 0,
            mp_max: 0,
            attack: 50,
            defence: 50,
        },
        Enemy::Boss => CharacterStatus{
            name: "Boss".to_string(),
            lv: 1,
            exp: 0,
            hp_current: 400,
            hp_max: 400,
            mp_current: 0,
            mp_max: 0,
            attack: 100,
            defence: 100,
        }
    }
}

pub fn field_to_enemy(
    field: MapField,
) -> Enemy {
    match field {
        MapField::Grass => Enemy::Goblin,
        MapField::Forest => Enemy::Elf,
        MapField::Mountain => Enemy::Bird,
        MapField::Castle => Enemy::Boss,
        _ => panic!()
    }
}
