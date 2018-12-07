pub use std::f64::{EPSILON};
use std::ops;
use vector3d::{Vector3D};

#[derive(Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D{ x: x, y: y, z: z, w: 1.0 }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Point3D) -> bool {
        (self.x - other.x).abs() < EPSILON &&
            (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON
    }
}

impl ops::Add<Vector3D> for Point3D {
    type Output = Point3D;
    fn add(self, vec: Vector3D) -> Point3D {
        Point3D::new(self.x + vec.x,
                     self.y + vec.y,
                     self.z + vec.z)
    }
}

impl ops::Sub<Vector3D> for Point3D {
    type Output = Point3D;
    fn sub(self, vec: Vector3D) -> Point3D {
        Point3D::new(self.x - vec.x,
                     self.y - vec.y,
                     self.z - vec.z)
    }
}

impl ops::Sub<Point3D> for Point3D {
    type Output = Vector3D;
    fn sub(self, point: Point3D) -> Vector3D {
        Vector3D::new(self.x - point.x,
                      self.y - point.y,
                      self.z - point.z)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_are_equal() {
        let p1 = Point3D::new(2.0, -4.0, 3.0 + EPSILON);
        let p2 = Point3D::new(2.0 + EPSILON, -4.0, 3.0);
        assert_eq!(p1, p2);
    }

    #[test]
    fn point_plus_vector() {
        let p = Point3D::new(2.0, -4.0, 3.0);
        let v = Vector3D::new(2.0, -4.0, 3.0);
        let new_point = p + v;
        let np = Point3D::new(4.0, -8.0, 6.0);
        assert_eq!(np, new_point);
    }

    #[test]
    fn point_minus_point() {
        let p1 = Point3D::new(3.0, 2.0, 1.0);
        let p2 = Point3D::new(5.0, 6.0, 7.0);
        let new_vector = p1 - p2;
        let nv = Vector3D::new(-2.0, -4.0, -6.0);
        assert_eq!(nv, new_vector);
    }

    #[test]
    fn point_minus_vector() {
        let p = Point3D::new(3.0, 2.0, 1.0);
        let v = Vector3D::new(5.0, 6.0, 7.0);
        let new_point = p - v;
        let np = Point3D::new(-2.0, -4.0, -6.0);
        assert_eq!(np, new_point);
    }

}
