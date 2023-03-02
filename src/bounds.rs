use crate::{VerletObject, RADIUS};
use raylib::prelude::*;

pub trait Boundable {
  fn restrict(&self, circle: VerletObject) -> VerletObject;
}

impl Boundable for Rectangle {
  fn restrict(&self, mut circle: VerletObject) -> VerletObject {
    circle.position.x = circle
      .position
      .x
      .clamp(self.x + RADIUS, self.x + self.width - RADIUS);
    circle.position.y = circle
      .position
      .y
      .clamp(self.y + RADIUS, self.y + self.height - RADIUS);
    circle
  }
}

pub struct Circle {
  pub x: f32,
  pub y: f32,
  pub radius: f32,
}

impl Boundable for Circle {
  fn restrict(&self, mut circle: VerletObject) -> VerletObject {
    let pos = Vector2::new(self.x, self.y);
    let distance2 = (pos - circle.position).length_sqr();
    if distance2 > (self.radius - RADIUS).powi(2) {
      circle.position =
        pos + (pos - circle.position).normalized() * -(self.radius - RADIUS);
    }
    circle
  }
}
