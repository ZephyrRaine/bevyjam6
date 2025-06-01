use bevy::prelude::*;
use bevy_vox_scene::{VoxLoaderSettings, VoxScenePlugin, VoxelInstanceReady, VoxelModelInstance};

use crate::{
    asset_tracking::LoadResource, demo::bipper::Bipper, demo::blink::Blink, demo::bumper::Bumper,
    demo::synchronized::Synchronized, screens::Screen,
};

use super::draggable::Draggable;

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
    instance_query: Query<&Transform, With<VoxelModelInstance>>,
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
                let mut track_hover = 1;
                let mut track_click = 2;

                if let Some(track_str) = params.get(0) {
                    if let Ok(track) = track_str.parse::<usize>() {
                        track_hover = track;
                    }
                }
                if let Some(track_str) = params.get(1) {
                    if let Ok(track) = track_str.parse::<usize>() {
                        track_click = track;
                    }
                }

                entity_commands.insert((Bipper {
                    audio_hover_id: format!("bipper{}.ogg", track_hover),
                    audio_click_id: format!("bipper{}.ogg", track_click),
                },));
            }
            "bumper" => {
                let mut bump_hover = 1.1;
                let mut bump_pressed = 0.9;

                if let Some(bump_str) = params.get(0) {
                    if let Ok(bump) = bump_str.parse::<f32>() {
                        bump_hover = bump;
                    }
                }
                if let Some(bump_str) = params.get(1) {
                    if let Ok(bump) = bump_str.parse::<f32>() {
                        bump_pressed = bump;
                    }
                }

                entity_commands.insert((Bumper {
                    bump_hover: bump_hover,
                    bump_pressed: bump_pressed,
                    target_scale: 1.0,
                },));
            }
            "slider" => {
                entity_commands.insert(Draggable::new(
                    params[0].to_string(),
                    params[1].parse::<i32>().unwrap(),
                    instance_query
                        .get(trigger.event().instance)
                        .unwrap()
                        .translation,
                    if params.len() > 2 {
                        Some(params[2].parse::<i32>().unwrap())
                    } else {
                        None
                    },
                ));
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
