use std::ops::Add;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
  // add fields
  x: i32,
  y: i32,
}

impl Point {
  // add methods
  pub fn new(x: i32, y: i32) -> Self {
    Point {
      x,
      y
    }
  }

  // 求勾股边
  pub fn magnitude(self) -> f64 {
    let x = self.x.pow(2) + self.y.pow(2);
    (x as f64).sqrt()
  }

  // 求两点距离
  pub fn dist(self, target: Point) -> f64 {
    let dx = self.x - target.x;
    let dy = self.y - target.y;
    let d_point = Point::new(dx, dy);
    d_point.magnitude()
  }
}

impl Add for Point {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y
    }
  }
}


#[derive(Debug, Clone)]
pub struct Polygon {
  // add fields
  points: Vec<Point>,
}

impl Polygon {
  pub fn new() -> Self {
    Polygon {
      points: Vec::new(),
    }
  }

  // add methods
  pub fn add_point(&mut self, point: Point) {
    self.points.push(point)
  }

  pub fn left_most_point(self) -> Option<Point> {
    let points = self.points;
    let mut target_point: Option<Point> = points.get(0).copied();
    for point in points {
      if point.x < target_point?.x {
        target_point = Some(point);
      }
    }
    target_point
  }

  pub fn iter(&self) -> std::slice::Iter<'_, Point> {
    self.points.iter()
  }
}

impl Iterator for Polygon {
  type Item = Point;
  fn next(&mut self) -> Option<Self::Item> {
     // 如果点向量为空，则返回None
     if self.points.is_empty() {
      return None;
    }
    // 从点向量中删除并返回第一个点
    Some(self.points.remove(0))
  }
}

impl Perimeter for Polygon {
  fn perimeter(&self) -> f64 {
    let mut result: f64 = 0.0;
    let total = self.points.len();
    let mut count = 0;
    while count < total {
      let prev = self.points[count];
      let target = self.points[(count + 1) % total];
      result = result + prev.dist(target);
      count = count + 1;
    }
    print!("{result}");
    result
  }
}

pub struct Circle {
  // add fields
  center: Point,
  radius: i32,
}

impl Circle {
  // add methods
  pub fn new(center: Point, radius: i32) -> Self {
    Circle { center, radius }
  }
}

impl Perimeter for Circle {
  fn perimeter(&self) -> f64 {
    let r = (2 * self.radius)  as f64;
    r * PI
  }
}

#[derive(Copy, Clone)]
pub enum Shape<T, U> {
  Polygon(T),
  Circle(U),
}
trait Perimeter {
  fn perimeter(&self) -> f64;
}

impl <T: Perimeter, U: Perimeter> Perimeter for Shape<T, U> {
  fn perimeter(&self) -> f64 {
    match self {
      Shape::Polygon(p) => p.perimeter(),
      Shape::Circle(c) => c.perimeter(),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  fn round_two_digits(x: f64) -> f64 {
      (x * 100.0).round() / 100.0
  }

  #[test]
  fn test_point_magnitude() {
      let p1 = Point::new(12, 13);
      assert_eq!(round_two_digits(p1.magnitude()), 17.69);
  }

  #[test]
  fn test_point_dist() {
      let p1 = Point::new(10, 10);
      let p2 = Point::new(14, 13);
      assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
  }

  #[test]
  fn test_point_add() {
      let p1 = Point::new(16, 16);
      let p2 = p1 + Point::new(-4, 3);
      assert_eq!(p2, Point::new(12, 19));
  }

  #[test]
  fn test_polygon_left_most_point() {
      let p1 = Point::new(12, 13);
      let p2 = Point::new(16, 16);

      let mut poly = Polygon::new();
      poly.add_point(p1);
      poly.add_point(p2);
      assert_eq!(poly.left_most_point(), Some(p1));
  }

  #[test]
  fn test_polygon_iter() {
      let p1 = Point::new(12, 13);
      let p2 = Point::new(16, 16);

      let mut poly = Polygon::new();
      poly.add_point(p1);
      poly.add_point(p2);

      let points = poly.iter().cloned().collect::<Vec<_>>();
      assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
  }

  #[test]
  fn test_shape_perimeters() {
      let mut poly = Polygon::new();
      poly.add_point(Point::new(12, 13));
      poly.add_point(Point::new(17, 11));
      poly.add_point(Point::new(16, 16));

      let shapes = vec![
          Shape::Polygon(poly),
          Shape::Circle(Circle::new(Point::new(10, 20), 5)),
      ];
      let perimeters = shapes
          .iter()
          .map(Shape::perimeter)
          .map(round_two_digits)
          .collect::<Vec<_>>();
      assert_eq!(perimeters, vec![15.48, 31.42]);
  }
}

// run at https://play.rust-lang.org/
#[allow(dead_code)]
fn main() {}
