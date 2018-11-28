pub use std::f64::{EPSILON};
use std::ops;
use point3d::{Point3D};

#[derive(Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D{ x: x, y: y, z: z, w: 0.0 }
    }

    fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    fn normalize(&self) -> Vector3D {
        let m = self.magnitude();
        Vector3D::new(self.x / m, self.y / m, self.z / m)
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Vector3D) -> bool {
        (self.x - other.x).abs() < EPSILON &&
        (self.y - other.y).abs() < EPSILON &&
        (self.z - other.z).abs() < EPSILON
    }
}

impl ops::Add<Point3D> for Vector3D {
    type Output = Point3D;
    fn add(self, point: Point3D) -> Point3D {
        Point3D::new(self.x + point.x,
                   self.y + point.y,
                   self.z + point.z)
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self.x + vec.x,
                    self.y + vec.y,
                    self.z + vec.z)
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn sub(self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self.x - vec.x,
                    self.y - vec.y,
                    self.z - vec.z)
    }
}

impl ops::Neg for Vector3D {
    type Output = Vector3D;
    fn neg(self) -> Vector3D {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors_are_equal() {
        let v1 = Vector3D::new(2.0, -4.0 + EPSILON, 3.0);
        let v2 = Vector3D::new(2.0, -4.0, 3.0 + EPSILON);
        assert_eq!(v1, v2);
    }

    #[test]
    fn vector_plus_point() {
        let p = Point3D::new(2.0, -4.0, 3.0);
        let v = Vector3D::new(2.0, -4.0, 3.0);
        let new_point = p + v;
        let np = Point3D::new(4.0, -8.0, 6.0);
        assert_eq!(np, new_point);
    }

    #[test]
    fn vector_plus_vector() {
        let v1 = Vector3D::new(2.0, -4.0, 3.0);
        let v2 = Vector3D::new(2.0, -4.0, 3.0);
        let new_vector = v1 + v2;
        let nv = Vector3D::new(4.0, -8.0, 6.0);
        assert_eq!(nv, new_vector);
    }

    #[test]
    fn vector_minus_vector() {
        let v1 = Vector3D::new(3.0, 2.0, 1.0);
        let v2 = Vector3D::new(5.0, 6.0, 7.0);
        let new_vector = v1 - v2;
        let nv = Vector3D::new(-2.0, -4.0, -6.0);
        assert_eq!(nv, new_vector);
    }

    #[test]
    fn vector_negate() {
        let v1 = Vector3D::new(3.0, -2.0, 1.0);
        let negated = -v1;
        let nn = Vector3D::new(-3.0, 2.0, -1.0);
        assert_eq!(nn, negated);
    }

    #[test]
    fn vector_magnitude_x() {
        let v = Vector3D::new(1.0, 0.0, 0.0);
        let magnitude = v.magnitude();
        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn vector_magnitude_y() {
        let v = Vector3D::new(0.0, 1.0, 0.0);
        let magnitude = v.magnitude();
        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn vector_magnitude_z() {
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let magnitude = v.magnitude();
        assert_eq!(1.0, magnitude);
    }

    #[test]
    fn vector_magnitude() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let magnitude = v.magnitude();
        assert_eq!(14.0f64.sqrt(), magnitude);
    }

    #[test]
    fn vector_magnitude_neg() {
        let v = -Vector3D::new(1.0, 2.0, 3.0);
        let magnitude = v.magnitude();
        assert_eq!(14.0f64.sqrt(), magnitude);
    }

    #[test]
    fn vector_normalize_to_unit() {
        let v = Vector3D::new(4.0, 0.0, 0.0);
        let normal = v.normalize();
        let nv = Vector3D::new(1.0, 0.0, 0.0);
        assert_eq!(nv, normal);
    }

    #[test]
    fn vector_normalize() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let normal = v.normalize().magnitude();
        assert_eq!(1.0, normal);
    }

}
