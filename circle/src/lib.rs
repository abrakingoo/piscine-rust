// Create the structures Circle and Point. 
// You'll need to create the necessary methods for the code to compile correctly.

// Methods:
    // Point:
            // distance() -> returns the distance between two coordinates.
    // Circle:
            // diameter() -> returns the diameter of the circle.
            // area() -> returns the area of the circle.
            // intersect() -> returns if two circles intersect.
// Associated Functions
    // Circle:
            // new() -> receives three 64-bit floating point numbers in the following order: 
            // x, y and radius (x and y are the coordinates of the center of the new circle). 
            // The function returns a new circle.
// hello

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    // Associated function to create a new Circle
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: r,
        }
    }

    // Method to calculate the diameter of the circle
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    // Method to calculate the area of the circle
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    // Method to check if two circles intersect
    pub fn intersect(&self, circle: Circle) -> bool {
        let distance = self.center.distance(circle.center);
        distance < (self.radius + circle.radius) // if the distance between centers is less than the sum of the radii, they intersect
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    // Method to calculate the distance between two points
    pub fn distance(&self, point: Point) -> f64 {
        // Distance formula: sqrt((x2 - x1)^2 + (y2 - y1)^2)
        ((self.0 - point.0).powi(2) + (self.1 - point.1).powi(2)).sqrt()
    }
}