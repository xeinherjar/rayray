mod point3d;
mod vector3d;

use point3d::{Point3D};
use vector3d::{Vector3D};

#[derive(Debug)]
struct Projectile {
    position: Point3D,
    velocity: Vector3D
}

struct Environment {
    gravity: Vector3D,
    wind: Vector3D
}

fn tick(p: Projectile, e: &Environment) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + e.gravity + e.wind;
    Projectile{position: position, velocity: velocity}
}

fn main() {
    let mut projectile = Projectile{position: Point3D::new(0.0, 1.0, 0.0),
    velocity: Vector3D::new(1.0, 1.0, 0.0).normalize()};

    let environment = Environment{gravity: Vector3D::new(0.0, -0.1, 0.0),
    wind: Vector3D::new(-0.01, 0.0, 0.0)};

    let mut tick_count = 0;
    while projectile.position.y > 0.0 {
        projectile = tick(projectile, &environment);
        tick_count += 1;
    }

    println!("Ticks: {:?}, Final Position: {:?}", tick_count, projectile);
}
