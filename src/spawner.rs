use crate::circle::Circle;

pub struct Spawner {
  interval: f64,
  last_spawn: f64,
  remainder: f64,
  count: i32,
}

impl Spawner {
  pub fn new(interval: f64, current_time: f64) -> Self {
    Spawner {
      interval,
      last_spawn: current_time,
      remainder: 0.0,
      count: 0,
    }
  }

  pub fn update<F>(&mut self, current_time: f64, genfunc: F) -> Vec<Circle>
  where
    F: Fn(i32) -> Circle,
  {
    let delta = current_time - self.last_spawn + self.remainder;
    let to_spawn = (delta / self.interval) as i32;
    self.remainder = delta % self.interval;
    let mut ret: Vec<Circle> = vec![];
    for _ in 0..to_spawn {
      self.count += 1;
      ret.push(genfunc(self.count));
    }
    self.last_spawn = current_time;
    ret
  }
}
