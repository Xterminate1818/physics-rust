use raylib::prelude::*;

pub const RADIUS: f32 = 15.0;
pub const DIAMETER: f32 = RADIUS * 2.0;
pub const DIAMETER_SQUARED: f32 = DIAMETER * DIAMETER;

#[derive(Copy, Clone)]
pub struct VerletObject {
  pub position: Vector2,
  pub last_position: Vector2,
  pub acceleration: Vector2,
  pub color: Color,
}

impl VerletObject {
  pub const fn new(position: Vector2, color: Color) -> Self {
    VerletObject {
      position,
      last_position: position,
      acceleration: Vector2::new(0.0, 0.0),
      color,
    }
  }

  pub fn update(&mut self, delta: f32) {
    let velocity = self.position - self.last_position;
    self.last_position = self.position;
    self.position =
      self.position + velocity + self.acceleration * delta.powi(2);
    self.acceleration = Vector2::zero(); // Apply bounds
  }
}
