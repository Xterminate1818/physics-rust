#[derive(Copy, Clone)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2{x, y}
    }
    
    pub fn distance(first: Vector2, second: Vector2) -> f64 {
        ((first.x - second.x).powi(2) + 
        (first.y - second.y).powi(2)).sqrt()
    }
}

#[derive(Copy, Clone)]
pub struct Circle {
    pos: Vector2,
    radius: f64
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle{pos: Vector2{x, y}, radius: r}
    }

    pub fn overlap(c1: &Circle, c2: &Circle) -> f64 {
        let distance = Vector2::distance(c1.pos, c2.pos);
        let overlap = c1.radius + c2.radius - distance;
        if overlap >= 0_f64 {overlap}
        else {0_f64}
    }
}
