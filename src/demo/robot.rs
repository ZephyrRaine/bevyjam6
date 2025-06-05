use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_vox_scene::{VoxLoaderSettings, VoxScenePlugin, VoxelInstanceReady, VoxelModelInstance};

use crate::{
    asset_tracking::LoadResource, demo::bipper::Bipper, demo::blink::Blink, demo::bumper::Bumper,
    demo::synchronized::Synchronized, screens::Screen,
};

use super::draggable::Draggable;
use super::puzzle::PuzzleSolver;
use super::slider::Slider;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(VoxScenePlugin {
        // Using global settings because Bevy's `load_with_settings` has a couple of issues:
        // https://github.com/bevyengine/bevy/issues/12320
        // https://github.com/bevyengine/bevy/issues/11111
        global_settings: Some(VoxLoaderSettings {
            supports_remeshing: true,
            voxel_size: 1.0,
            //            mesh_offset: UnitOffset::new(Vec3::new(1.0, 0.0, 0.0)),
            ..default()
        }),
    });
    app.register_type::<RobotAssets>();
    app.load_resource::<RobotAssets>();
    app.insert_resource(SliderCounter {
        slider_id: vec![0, 0],
    });
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RobotAssets {
    #[dependency]
    robot: Handle<Scene>,
    #[dependency]
    props: Handle<Scene>,
    #[dependency]
    material: Handle<StandardMaterial>,
    #[dependency]
    material_no_emission: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct SliderCounter {
    pub slider_id: Vec<usize>,
}

impl FromWorld for RobotAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            robot: assets.load("models/robot.vox"),
            props: assets.load("models/props.vox"),
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

    commands
        .spawn((
            Name::new("Props"),
            SceneRoot(robot_assets.props.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .observe(on_pros_voxel_instance_ready);

    commands.spawn((
        Name::new("PuzzleSolver 0"),
        PuzzleSolver {
            puzzle_id: 0,
            correct_positions: vec![-4, -3, 0, -1],
            current_positions: vec![0, 0, 0, 0],
        },
    ));

    commands.spawn((
        Name::new("PuzzleSolver 1"),
        PuzzleSolver {
            puzzle_id: 1,
            correct_positions: vec![-3, -1, -2, -1, -2, -2, -1, -3],
            current_positions: vec![0, 0, 0, 0, 0, 0, 0, 0],
        },
    ));
}

fn on_pros_voxel_instance_ready(trigger: Trigger<VoxelInstanceReady>, mut commands: Commands) {
    let Some(name) = &trigger.event().model_name else {
        return;
    };
    let mut entity_commands = commands.entity(trigger.event().instance);

    if name.contains("wall") {
        entity_commands.insert(NotShadowCaster);
    }
}

fn on_voxel_instance_ready(
    trigger: Trigger<VoxelInstanceReady>,
    mut commands: Commands,
    robot_assets: Res<RobotAssets>,
    instance_query: Query<&Transform, With<VoxelModelInstance>>,
    mut slider_counter: ResMut<SliderCounter>,
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

                if let Some(track_str) = params.first() {
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

                if let Some(bump_str) = params.first() {
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
                    bump_hover,
                    bump_pressed,
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
                ));
                if params.len() > 2 {
                    let puzzle_id = params[2].parse::<usize>().unwrap();
                    entity_commands.insert(Slider::new(
                        puzzle_id.try_into().unwrap(),
                        slider_counter.slider_id[puzzle_id],
                    ));
                    slider_counter.slider_id[puzzle_id] += 1;
                }
            }
            _ => {}
        }
    }
}
