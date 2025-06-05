//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

//mod animation;
pub mod level;
//mod movement;
//pub mod player;
pub mod audio;
pub mod bipper;
pub mod blink;
pub mod bumper;
pub mod draggable;
pub mod puzzle;
pub mod robot;
pub mod slider;
pub mod synchronized;
pub mod toggler;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        //animation::plugin,
        level::plugin,
        //movement::plugin,
        //player::plugin,
        robot::plugin,
        blink::plugin,
        bipper::plugin,
        bumper::plugin,
        toggler::plugin,
        synchronized::plugin,
        draggable::plugin,
        puzzle::plugin,
    ));
}
