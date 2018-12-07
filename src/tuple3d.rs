pub use std::f64::{EPSILON};
use std::ops;
use point3d::{Point3D};
use vector3d::{Vector3D};

#[derive(Debug)]
pub struct Tuple3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple3D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple3D {
        Tuple3D{ x: x, y: y, z: z, w: w }
    }

    fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    fn normalize(&self) -> Tuple3D {
        let m = self.magnitude();
        Tuple3D::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    fn dot(&self, other: Tuple3D) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w + other.w)
    }

}

impl PartialEq for Tuple3D {
    fn eq(&self, other: &Tuple3D) -> bool {
        (self.x - other.x).abs() < EPSILON &&
            (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON &&
            (self.w - other.w).abs() < EPSILON
    }
}

impl ops::Add<Tuple3D> for Tuple3D {
    type Output = Tuple3D;
    fn add(self, other: Tuple3D) -> Tuple3D {
        Tuple3D::new(self.x + other.x,
                     self.y + other.y,
                     self.z + other.z,
                     self.w + other.w)
    }
}

impl ops::Sub<Tuple3D> for Tuple3D {
    type Output = Tuple3D;
    fn sub(self, other: Tuple3D) -> Tuple3D {
        Tuple3D::new(self.x - other.x,
                     self.y - other.y,
                     self.z - other.z,
                     self.w - other.w)
    }
}

impl ops::Neg for Tuple3D {
    type Output = Tuple3D;
    fn neg(self) -> Tuple3D {
        Tuple3D::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f64> for Tuple3D {
    type Output = Tuple3D;
    fn mul(self, f: f64) -> Tuple3D {
        Tuple3D::new(self.x * f, self.y * f, self.z * f, self.w * f)
    }
}

impl ops::Div<f64> for Tuple3D {
    type Output = Tuple3D;
    fn div(self, f: f64) -> Tuple3D {
        Tuple3D::new(self.x / f, self.y / f, self.z / f, self.w / f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuples_are_equal() {
        let t1 = Tuple3D::new(2.0, -4.0, 3.0 + EPSILON, 3.3);
        let t2 = Tuple3D::new(2.0 + EPSILON, -4.0, 3.0, 3.3);
        assert_eq!(t1, t2);
    }

    #[test]
    fn tuple_plus_tuple() {
        let t1 = Tuple3D::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple3D::new(1.0, 2.0, 3.0, 4.0);
        let new_tuple = t1 + t2;
        let nt = Tuple3D::new(2.0, 4.0, 6.0, 8.0);
        assert_eq!(nt, new_tuple);
    }

    #[test]
    fn tuple_negate() {
        let t1 = Tuple3D::new(1.0, 2.0, 3.0, -4.0);
        let negated = -t1;
        let nt = Tuple3D::new(-1.0, -2.0, -3.0, 4.0);
        assert_eq!(nt, negated);
    }

    #[test]
    fn tuple_mul_scaler() {
        let t1 = Tuple3D::new(1.0, -2.0, 3.0, -4.0);
        let scaled = t1 * 3.5;
        let ns = Tuple3D::new(3.5, -7.0, 10.5, -14.0);
        assert_eq!(scaled, ns);
    }

    #[test]
    fn tuple_mul_scaler_fraction() {
        let t1 = Tuple3D::new(1.0, -2.0, 3.0, -4.0);
        let scaled = t1 * 0.5;
        let ns = Tuple3D::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(scaled, ns);
    }

    #[test]
    fn tuple_div_scaler() {
        let t1 = Tuple3D::new(1.0, -2.0, 3.0, -4.0);
        let scaled = t1 / 2.0;
        let ns = Tuple3D::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(scaled, ns);
    }

    #[test]
    fn tuple_dot_product() {
        let t1 = Tuple3D::new(1.0, 2.0, 3.0, 0.0);
        let t2 = Tuple3D::new(2.0, 3.0, 4.0, 0.0);
        let dot = t1.dot(t2);
        assert_eq!(20.0, dot);
    }
}

