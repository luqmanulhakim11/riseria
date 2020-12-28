mod components;
mod events;
mod systems;

use bevy::ecs::RunOnce;
use bevy::prelude::*;
use components::*;
use systems::*;

fn main() {
    App::build()
        // Window descriptor needs to be before the DefaultPlugin
        .add_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "riseria".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_stage_after(
            stage::STARTUP,
            "once",
            SystemStage::serial()
                .with_run_criteria(RunOnce::default())
                .with_system(human_spawn.system()),
        )
        .add_stage_before(
            stage::UPDATE,
            "mechanics_update",
            SystemStage::parallel().with_system(grid_position_translation.system()),
        )
        .add_system(human_move.system())
        .add_stage_after(
            stage::UPDATE,
            "after_updates",
            SystemStage::parallel().with_system(human_print_position.system()),
        )
        .add_stage_after(
            stage::POST_UPDATE,
            "check event",
            SystemStage::parallel().with_system(listen_for_exit.system()),
        )
        .run();
}

/// Setup function to register needed materials
fn setup(commands: &mut Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GameColorMaterials {
        human_material: color_materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
    info!("Material added!");
    info!("Setup is done!");
}
