use raylib::prelude::*;

pub const RADIUS: f32 = 30.0;
pub const DIAMETER: f32 = RADIUS * 2.0;
pub const RADIUS_SQUARED: f32 = RADIUS * RADIUS;
pub const DIAMETER_SQUARED: f32 = DIAMETER * DIAMETER;
pub const UP: Vector2 = Vector2::new(0.0, -1.0);
pub const DOWN: Vector2 = Vector2::new(0.0, -1.0);
pub const LEFT: Vector2 = Vector2::new(-1.0, 0.0);
pub const RIGHT: Vector2 = Vector2::new(1.0, 0.0);
