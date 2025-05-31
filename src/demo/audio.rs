use bevy::{audio::PlaybackSettings, prelude::*};
use bevy_audio_controller::prelude::*;

use crate::screens::Screen;

#[derive(Component, Default, AudioChannel, Reflect)]
#[reflect(Component)]
struct MusicChannel;

#[derive(Component, Default, AudioChannel, Reflect)]
#[reflect(Component)]
struct SfxChannel;

pub(super) fn plugin(app: &mut App) {
    app.register_audio_channel::<MusicChannel>();
    app.register_audio_channel::<SfxChannel>();
    app.add_systems(OnEnter(Screen::Gameplay), setup_audio);
}

fn setup_audio(mut ew: EventWriter<PlayEvent<MusicChannel>>) {
    let tracks = ["track1bip1.ogg", "track1bip2.ogg", "track1bip3.ogg"];

    for track in tracks {
        let event = MusicChannel::play_event(track.into()).with_settings(PlaybackSettings::LOOP);
        ew.write(event);
    }
}
