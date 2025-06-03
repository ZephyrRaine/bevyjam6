use bevy::{audio::PlaybackSettings, prelude::*};
use bevy_audio_controller::prelude::*;

#[derive(Component, Default, AudioChannel, Reflect)]
#[reflect(Component)]
pub struct MusicChannel;

pub(super) fn plugin(app: &mut App) {
    app.register_audio_channel::<MusicChannel>();
}

pub fn setup_audio(mut ew: EventWriter<PlayEvent<MusicChannel>>) {
    // let tracks = ["track1bip1.ogg", "track1bip2.ogg", "track1bip3.ogg"];
    let tracks: [&'static str; 4] = [
        "track2bip5.ogg",
        "track2bip6.ogg",
        "track2bip7.ogg",
        "track2bip8.ogg",
    ];
    //let tracks = ["track1bip1.ogg", "track1bip2.ogg", "track1bip3.ogg"];
    for track in tracks {
        let event = MusicChannel::play_event(track.into()).with_settings(PlaybackSettings::LOOP);
        ew.write(event);
    }
}
