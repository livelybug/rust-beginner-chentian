#[derive(Debug)]
struct Rectangle {
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
struct Shape {
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

fn main() {
    let triangle = Triangle{a:3.0, b: 4.0, c: 5.0};
    let shape = Shape::new(EShape::Triangle(triangle));
    println!("shape:{:#?}", shape);
    println!("perimeter: {}", shape.perimeter());
    println!("area: {}", shape.area());
}