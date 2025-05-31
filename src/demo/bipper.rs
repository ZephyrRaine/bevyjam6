use bevy::prelude::*;

use crate::{audio::sound_effect};

use super::robot::RobotAssets;

#[derive(Component)]
pub struct Bipper {
    //pub audio_hover: Handle<AudioSource>,
    //pub audio_click: Handle<AudioSource>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_observer(bipper_play_hover_audio);
    app.add_observer(bipper_play_click_audio);
}

pub fn bipper_play_hover_audio(
    trigger: Trigger<Pointer<Over>>,
    query: Query<(), With<Bipper>>,
    mut commands: Commands,
    robot_assets: Option<Res<RobotAssets>>,
) {
    let Some(robot_assets) = robot_assets else {
        return;
    };

    if query.contains(trigger.target()) {
        commands.spawn(sound_effect(robot_assets.audio_hover.clone()));
    }
}
pub fn bipper_play_click_audio(
    trigger: Trigger<Pointer<Click>>,
    query: Query<(), With<Bipper>>,
    mut commands: Commands,
    robot_assets: Option<Res<RobotAssets>>,
) {
    let Some(robot_assets) = robot_assets else {
        return;
    };

    if query.contains(trigger.target()) {
        commands.spawn(sound_effect(robot_assets.audio_click.clone()));
    }
}
