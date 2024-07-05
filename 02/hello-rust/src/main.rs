mod shape;
mod fib;

use shape::Shape as exShape;
use fib::call_fib;

struct Rectangle {
    a: f64,
    b: f64,
 }
 
 struct Circle {
   r: f64,
 }
 
 struct Triangle {
   a: f64,
   b: f64,
   c: f64,
 }

 enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
 }

 trait ShapeI {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
 }

 impl ShapeI for Rectangle {
    fn area(&self) -> f64 {
        0.0
    }

    fn circumference(&self) -> f64 {
        0.0
    }
 }

fn main() {
    println!("Hello, world!");
    call_fib();
}
