use bevy::prelude::*;
use bevy_audio_controller::prelude::{AudioChannel, DelayMode};

use crate::demo::{bipper::{SfxChannel, SfxEvent}, blink::Blink, puzzle::PuzzleEvent};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct IndexTracker {
    pub current_id: usize,
}

impl FromWorld for IndexTracker {
    fn from_world(_world: &mut World) -> Self {
        Self {
            current_id: 0,
        }
    }
}

#[derive(Component)]
pub struct Toggler {
    pub unique_id: i32,
    pub puzzle_id: u32,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IndexTracker>();
    app.init_resource::<IndexTracker>();
    app.add_observer(toggle_play_pressed);
}

pub fn toggle_play_pressed(
    trigger: Trigger<Pointer<Pressed>>,
    mut query: Query<(Option<&Toggler>, &mut Blink)>,
    mut puzzle_events: EventWriter<PuzzleEvent>,
    mut index_tracker: ResMut<IndexTracker>,
    mut ew: EventWriter<SfxEvent>,
) {
    if let Ok((toggler, mut blink)) = query.get_mut(trigger.target()) {
        if !blink.is_on
        {
            blink.toggle();

            if let Some(toggler) = toggler {
                puzzle_events.write(PuzzleEvent {
                    puzzle_id: toggler.puzzle_id,
                    element_id: index_tracker.current_id,
                    element_value: toggler.unique_id,
                });

                index_tracker.current_id += 1;
            }
            ew.write(
                SfxChannel::play_event("bipper1.ogg".into())
                    .with_settings(PlaybackSettings::DESPAWN)
                    .with_delay_mode(DelayMode::Immediate),
            );
        }
    }
}
