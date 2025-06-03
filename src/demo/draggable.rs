use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_audio_controller::prelude::{AudioChannel, DelayMode};

use crate::screens::Screen;

use super::bipper::{SfxChannel, SfxEvent};
use super::puzzle::PuzzleEvent;
use super::slider::Slider;

#[derive(Event, Deref)]
struct GrabEvent(bool);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Draggable {
    pub axis: String,
    pub offset: i32,
    pub base_position: Vec3,
    //    pub correct_position: Option<i32>,
    initial_drag_position: Option<Vec3>,
    snap_interval: f32,
}

impl Draggable {
    pub fn new(
        axis: String,
        offset: i32,
        base_position: Vec3,
        //    correct_position: Option<i32>,
    ) -> Self {
        Self {
            axis,
            offset,
            base_position,
            //      correct_position,
            initial_drag_position: None,
            snap_interval: 1.0, // Snap every 50 units
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Draggable>();
    app.add_event::<GrabEvent>();
    app.add_observer(on_drag_start);
    app.add_observer(on_drag);
    app.add_observer(on_drag_end);
    app.add_systems(Update, apply_grab.run_if(in_state(Screen::Gameplay)));
}

fn on_drag_start(
    drag: Trigger<Pointer<DragStart>>,
    mut transforms: Query<(&mut Transform, &mut Draggable)>,
    mut grab_events: EventWriter<GrabEvent>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }

    grab_events.write(GrabEvent(true));

    if let Ok((transform, mut draggable)) = transforms.get_mut(drag.target) {
        draggable.initial_drag_position = Some(transform.translation);
    }
}

fn on_drag_end(drag: Trigger<Pointer<DragEnd>>, mut grab_events: EventWriter<GrabEvent>) {
    if drag.button != PointerButton::Primary {
        return;
    }

    grab_events.write(GrabEvent(false));
}

fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut transforms: Query<(&mut Transform, &mut Draggable, Option<&Slider>)>,
    time: Res<Time>,
    mut puzzle_events: EventWriter<PuzzleEvent>,
    mut ew: EventWriter<SfxEvent>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }

    if let Ok((mut transform, draggable, slider)) = transforms.get_mut(drag.target) {
        let disp_value = -drag.distance.y * time.delta_secs() * 3.0;

        // Get the initial position when drag started
        let initial_position = draggable
            .initial_drag_position
            .unwrap_or(transform.translation);

        // Calculate target position relative to initial drag position
        let mut target_position = initial_position + Vec3::new(0.0, disp_value, 0.0);

        // Apply bounds
        if target_position.y > draggable.base_position.y {
            target_position.y = draggable.base_position.y;
        }
        if target_position.y < draggable.base_position.y + draggable.offset as f32 {
            target_position.y = draggable.base_position.y + draggable.offset as f32;
        }

        if let Some(slider) = slider {
            let snap_value =
                (target_position.y / draggable.snap_interval).round() * draggable.snap_interval;
            target_position.y = snap_value;
            if transform.translation.y != target_position.y {
                puzzle_events.write(PuzzleEvent {
                    puzzle_id: slider.puzzle_id,
                    slider_id: slider.slider_id,
                    slider_position: ((snap_value - draggable.base_position.y)
                        / draggable.snap_interval) as i32,
                });
                ew.write(
                    SfxChannel::play_event("bipper1.ogg".into())
                        .with_settings(PlaybackSettings::DESPAWN)
                        .with_delay_mode(DelayMode::Immediate),
                );
            }
        }

        // Smoothly interpolate to the target position
        transform.translation = target_position;
    }
}

fn apply_grab(
    mut ev: EventReader<GrabEvent>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    for grab in ev.read() {
        window.cursor_options.visible = !(**grab)
    }
}
