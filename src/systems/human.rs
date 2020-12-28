use crate::components::*;
use bevy::prelude::*;
//use log::*;

//pub fn setup_human(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {}
pub fn human_spawn(commands: &mut Commands, materials: Res<GameColorMaterials>) {
    //commands.spawn();
    commands
        .spawn(SpriteBundle {
            material: materials.human_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Type(EntityType::Human))
        .with(Position { x: 0.0, y: 0.0 });
    info!("A Human is spawned!");
}

/// Prints the position of all humans in the game
pub fn human_print_position(query: Query<(&Type, &Position)>) {
    for (entitytype, position) in query.iter() {
        if entitytype.get_type() == EntityType::Human {
            info!("({},{})", position.x, position.y);
        }
    }
}
pub fn human_move(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Type, &mut Position)>) {
    for (_entitytype, mut position) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::S) {
            position.y -= 0.1;
        } else if keyboard_input.pressed(KeyCode::D) {
            position.x += 0.1;
        } else if keyboard_input.pressed(KeyCode::W) {
            position.y += 0.1;
        } else if keyboard_input.pressed(KeyCode::A) {
            position.x -= 0.1;
        }
    }
}
