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
#[derive(Debug, Clone, Copy)]
use std::f64::consts::PI;

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {

    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point { x: x, y: y },
            radius: r,
        }
    }

    // Method to calculate the diameter based on the Circle's radius
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    // Method to calculate the area of the circle using the accurate value of PI
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    // Check if two circles intersect based on their center and radius
    pub fn intersect(&self, other: &Circle) -> bool {
        let distance = (self.center - other.center).abs(); // Calculate the distance between centers (1D for now)
        let sum_of_radii = self.radius + other.radius; // Sum of the radii of both circles

        // The circles intersect if the distance between centers is less than or equal to the sum of the radii
        distance <= sum_of_radii
    }
}


// returns the distance between two coordinates.
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, point: &Point) -> f64 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }
}