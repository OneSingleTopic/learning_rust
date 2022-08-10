use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

// By defaut Right Hand Side (RHS) type is Self for Add -> Here : Point
impl Add for Point {
    type Output = Point;
    fn add(self, another_point: Point) -> Point {
        Point {
            x: self.x + another_point.x,
            y: self.y + another_point.y,
        }
    }
}

pub struct Millimeters(f32);
pub struct Meters(f32);

// We can specify Right Hand Side (RHS) type for Add -> Here : Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000.0))
    }
}

// We can specify Right Hand Side (RHS) type for Add -> Here : Millimeters
impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, millimiters: Millimeters) -> Meters {
        Meters(0.001 * millimiters.0 + self.0)
    }
}
