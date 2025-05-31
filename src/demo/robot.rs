use bevy::prelude::*;
use bevy_vox_scene::{VoxLoaderSettings, VoxScenePlugin, VoxelInstanceReady};

use crate::{
    asset_tracking::LoadResource, demo::bipper::Bipper, demo::blink::Blink,
    demo::synchronized::Synchronized, screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(VoxScenePlugin {
        // Using global settings because Bevy's `load_with_settings` has a couple of issues:
        // https://github.com/bevyengine/bevy/issues/12320
        // https://github.com/bevyengine/bevy/issues/11111
        global_settings: Some(VoxLoaderSettings {
            supports_remeshing: true,
            ..default()
        }),
    });
    app.register_type::<RobotAssets>();
    app.load_resource::<RobotAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RobotAssets {
    #[dependency]
    robot: Handle<Scene>,
    #[dependency]
    material: Handle<StandardMaterial>,
    #[dependency]
    material_no_emission: Handle<StandardMaterial>,
}

impl FromWorld for RobotAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            robot: assets.load("models/robot.vox"),
            material: assets.load("models/robot.vox#material"),
            material_no_emission: assets.load("models/robot.vox#material-no-emission"),
        }
    }
}

pub fn spawn_robot(mut commands: Commands, robot_assets: Res<RobotAssets>) {
    commands
        .spawn((
            Name::new("Robot"),
            SceneRoot(robot_assets.robot.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            StateScoped(Screen::Gameplay),
        ))
        .observe(on_voxel_instance_ready);
}

fn on_voxel_instance_ready(
    trigger: Trigger<VoxelInstanceReady>,
    mut commands: Commands,
    robot_assets: Res<RobotAssets>,
) {
    let Some(name) = &trigger.event().model_name else {
        return;
    };
    let mut entity_commands = commands.entity(trigger.event().instance);

    // Split by spaces to get individual component specifications
    for component_spec in name.split_whitespace() {
        // Split by ":" to get key and its parameters
        let parts: Vec<&str> = component_spec.split(':').collect();
        if parts.is_empty() {
            continue;
        }

        let key = parts[0];
        let params = &parts[1..];

        match key {
            "blink" => {
                entity_commands.insert(Blink {
                    is_on: true,
                    on_material: robot_assets.material.clone(),
                    off_material: robot_assets.material_no_emission.clone(),
                });
                if let Some(track_str) = params.first() {
                    if let Ok(track) = track_str.parse::<usize>() {
                        entity_commands.insert(Synchronized::new(track));
                        break;
                    }
                }
                entity_commands.insert(Synchronized::new(0));
            }
            "bipper" => {
                entity_commands.insert((Bipper {
                    audio_hover_id: "bipper2.ogg".to_string(),
                    audio_click_id: "bipper1.ogg".to_string(),
                },));
            }
            _ => {}
        }
    }

    entity_commands.observe(|mut trigger: Trigger<Pointer<Click>>| {
        println!("{} was just clicked!", trigger.target());
        // Get the underlying pointer event data
        let _click_event: &Pointer<Click> = trigger.event();
        // Stop the event from bubbling up the entity hierarchy
        trigger.propagate(false);
    });
}
