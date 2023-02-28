use std::time::Instant;

use crate::circle::*;
use raylib::prelude::*;
use rayon::prelude::*;

pub struct ImpulseSolver {
  pub buf1: Vec<Circle>,
  buf2: Vec<Circle>,
  // pub objs: Vec<Circle>,
  pub substeps: u32,
  bounds: Rectangle,
}

impl Default for ImpulseSolver {
  fn default() -> Self {
    Self {
      buf1: vec![],
      buf2: vec![],
      // objs: vec![],
      substeps: 8,
      bounds: Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 1000.0,
      },
    }
  }
}

impl ImpulseSolver {
  pub fn new() -> Self {
    ImpulseSolver::default()
  }

  pub fn add_circle(&mut self, circle: Circle) {
    let pos = &circle.position;
    if pos.x >= self.bounds.x
      && pos.x <= self.bounds.x + self.bounds.width
      && pos.y >= self.bounds.y
      && pos.y <= self.bounds.y + self.bounds.height
    {
      self.buf1.push(circle);
    }
  }

  pub fn count_circles(&self) -> usize {
    self.buf1.len()
  }

  pub fn better_collision(&mut self) {
  }

  pub fn find_collisions(&mut self) {
    let response_coefficient = 0.75;
    for circle_index in 0..self.buf1.len() {
      // Apply gravity
      self.buf1[circle_index].acceleration += Vector2::new(0.0, 1000.0);
      for other_index in circle_index..self.buf1.len() {
        let circle = &self.buf1[circle_index].position;
        let other = &self.buf1[other_index].position;
        let minimum_distance =
          self.buf1[circle_index].radius + self.buf1[other_index].radius;
        if circle.x < other.x - minimum_distance {
          break; // No further collisions possible
        }
        let dy = (circle.y - other.y).abs();
        if dy >= minimum_distance {
          continue; // Skip over obvious noncollisions
        }
        let combined = *circle - *other;
        let distance_squared = combined.length_sqr();
        if distance_squared >= minimum_distance.powi(2)
          || distance_squared == 0.0
        {
          continue;
        }
        // Finally, resort to expensive calculation
        let distance = distance_squared.sqrt();
        let normalized = combined.scale_by(1.0 / distance);
        let delta = 0.5 * response_coefficient * (distance - minimum_distance);
        self.buf1[circle_index].position -= normalized * delta * 0.5;
        self.buf1[other_index].position += normalized * delta * 0.5;
      }
    }
  }

  pub fn resolve_velocities(&mut self, delta: f32) {
    let delta_squared = (delta).powi(2);
    for o in &mut self.buf1 {
      let velocity = o.position - o.last_position;
      o.last_position = o.position;
      o.position = o.position + velocity + o.acceleration * delta_squared;
      o.acceleration = Vector2::zero();
      // Apply bounds
      o.position.x = o.position.x.clamp(
        self.bounds.x + o.radius,
        self.bounds.x + self.bounds.width - o.radius,
      );
      o.position.y = o.position.y.clamp(
        self.bounds.y + o.radius,
        self.bounds.y + self.bounds.height - o.radius,
      );
    }
  }

  pub fn apply_constraints(&mut self, circle: &mut Circle) {
    if circle.position.y + circle.radius < self.bounds.height + self.bounds.y {
      circle.position.y = self.bounds.height + self.bounds.y;
    }
  }

  pub fn sort_objects(&mut self) {
    self.buf1.par_sort_unstable_by(|a, b| {
      a.position.x.partial_cmp(&b.position.x).unwrap()
    })
  }

  pub fn set_substeps(&mut self, new: u32) {
    self.substeps = new;
  }

  pub fn do_substep(&mut self, delta: f32) {
    let t = Instant::now();
    self.find_collisions();
    // self.better_collision();
    println!("Collision detection: {} msec", t.elapsed().as_millis());

    let t = Instant::now();
    self.resolve_velocities(delta);
    println!("Movement: {} msec", t.elapsed().as_millis());

    let t = Instant::now();
    self.sort_objects();
    println!("Sorting: {} msec", t.elapsed().as_millis());
  }

  pub fn do_step(&mut self) {
    let substep_delta = (1.0 / 60.0) / (self.substeps as f32);
    if substep_delta == 0.0 {
      println!("0 DELTA!!!")
    }
    // Amongus :) ඞ
    for _ in 0..self.substeps {
      self.do_substep(substep_delta);
    }
  }
}
