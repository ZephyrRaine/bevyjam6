use bevy::prelude::*;
use bevy_audio_controller::prelude::{AudioChannel, ChannelRegistration, DelayMode, PlayEvent};

#[derive(Component)]
pub struct Bipper {
    pub audio_hover_id: String,
    pub audio_click_id: String,
}

#[derive(Component, Default, AudioChannel, Reflect)]
#[reflect(Component)]
pub struct SfxChannel;

pub type SfxEvent = PlayEvent<SfxChannel>;

pub(super) fn plugin(app: &mut App) {
    app.register_audio_channel::<SfxChannel>();
    app.add_observer(bipper_play_hover_audio);
    app.add_observer(bipper_play_click_audio);
}

pub fn bipper_play_hover_audio(
    trigger: Trigger<Pointer<Over>>,
    query: Query<&Bipper>,
    mut ew: EventWriter<SfxEvent>,
) {
    if query.contains(trigger.target()) {
        let bipper = query.get(trigger.target()).unwrap();
        ew.write(SfxChannel::play_event(bipper.audio_hover_id.clone().into())
            .with_settings(PlaybackSettings::DESPAWN)
            .with_delay_mode(DelayMode::Immediate));
    }
}
pub fn bipper_play_click_audio(
    trigger: Trigger<Pointer<Click>>,
    query: Query<&Bipper>,
    mut ew: EventWriter<SfxEvent>,
) {
    if query.contains(trigger.target()) {
        let bipper = query.get(trigger.target()).unwrap();
        ew.write(SfxChannel::play_event(bipper.audio_click_id.clone().into())
            .with_settings(PlaybackSettings::DESPAWN)
            .with_delay_mode(DelayMode::Immediate));
    }
}
