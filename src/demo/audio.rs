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
    println!("Setting up audio");
    let event =
        MusicChannel::play_event("track1bip1.ogg".into()).with_settings(PlaybackSettings::LOOP);
    ew.write(event);
}

// fn play_sfx(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
//     let event =
//         SfxChannel::play_event(AudioFiles::FireOGG).with_settings(PlaybackSettings::DESPAWN);
//     ew.write(event);
// }
