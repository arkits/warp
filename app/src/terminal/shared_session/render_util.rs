use pathfinder_color::ColorU;
use crate::appearance::Appearance;

pub const SHARED_SESSION_AVATAR_DIAMETER: f32 = 20.0;

pub fn shared_session_indicator_color(_: &Appearance) -> ColorU {
    ColorU::new(128, 128, 128, 255)
}
