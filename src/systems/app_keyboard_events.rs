use bevy::prelude::*;
//use log::*;

pub fn listen_for_exit(
    keyboard_input: Res<Input<KeyCode>>,
    mut event: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event.send(bevy::app::AppExit {});
        info!("Exit event sent!");
        warn!("App is going down!");
    }
}
