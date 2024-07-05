#[derive(Debug)]
pub struct Rectangle {
  a: f64,
  b: f64,
}

#[derive(Debug)]
struct Circle {
  r: f64,
}

#[derive(Debug)]
struct Triangle {
  a: f64,
  b: f64,
  c: f64,
}

#[derive(Debug)]
enum EShape {
  Rectangle(Rectangle),
  Circle(Circle),
  Triangle(Triangle),
}

#[derive(Debug)]
pub struct Shape {
    shape: EShape,
}

trait ShapeI {
  fn area(&self) -> f64;
  fn perimeter(&self) -> f64;
}

impl Shape {
  fn new(shape: EShape) -> Shape {
    Shape{ shape }
  }
}

impl ShapeI for Shape {
  fn area(&self) -> f64 {
    match &self.shape {
      EShape::Rectangle(rectangle) => rectangle.a,
      EShape::Circle(circle) => circle.r,
      EShape::Triangle(triangle) => triangle.a,
    }
  }

  fn perimeter(&self) -> f64 {
    0.0
  }
}
