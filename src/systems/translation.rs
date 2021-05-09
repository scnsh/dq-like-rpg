use bevy::prelude::*;
use crate::resources::*;
use crate::components::*;

// Positionを持つEntityに対してそれが更新されたら、Transoform の位置を移動させる
pub fn translation(
    map: Res<Map>,
    mut query: Query<(&mut Transform, &Position), Changed<Position>>
){
    for (mut transform, position) in query.iter_mut(){
        *transform =
            position_to_translation(&map, &(*position), transform.translation.z);
    }
}