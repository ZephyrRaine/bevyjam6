use bevy::prelude::*;

use crate::screens::Screen;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SyncTracks {
    pub timer_tracks: [Timer; 3],
}

impl FromWorld for SyncTracks {
    fn from_world(_world: &mut World) -> Self {
        Self {
            timer_tracks: [
                Timer::from_seconds((60.0 / (90.0 / 8.0)) * 0.5, TimerMode::Repeating),
                Timer::from_seconds((60.0 / (90.0 * 1.5)) * 0.5, TimerMode::Repeating),
                Timer::from_seconds((60.0 / (90.0 * 1.0)) * 0.5, TimerMode::Repeating),
            ],
        }
    }
}

#[derive(Component, Default)]
pub struct Synchronized {
    pub track: usize,
}

impl Synchronized {
    pub fn new(track: usize) -> Self {
        Self { track }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SyncTracks>();
    app.init_resource::<SyncTracks>();
    app.add_systems(Update, update_timers.run_if(in_state(Screen::Gameplay)));
}

pub fn update_timers(mut blink_tracks: ResMut<SyncTracks>, time: Res<Time>) {
    for timer in blink_tracks.timer_tracks.iter_mut() {
        timer.tick(time.delta());
    }
}
