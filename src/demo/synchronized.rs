use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SyncTracks {
    pub timer_tracks: [Timer; 3],
}

impl FromWorld for SyncTracks {
    fn from_world(_world: &mut World) -> Self {
        Self {
            timer_tracks: [
                Timer::from_seconds(0.1, TimerMode::Repeating),
                Timer::from_seconds(0.5, TimerMode::Repeating),
                Timer::from_seconds(1.0, TimerMode::Repeating),
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
    app.add_systems(Update, update_timers);
}

pub fn update_timers(mut blink_tracks: ResMut<SyncTracks>, time: Res<Time>) {
    for timer in blink_tracks.timer_tracks.iter_mut() {
        timer.tick(time.delta());
    }
}
