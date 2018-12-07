mod point3d;
mod vector3d;
mod canvas;
mod color;

use point3d::{Point3D};
use vector3d::{Vector3D};
use canvas::{Canvas};
use color::{Color};

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
    let velocity = Vector3D::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile{position: Point3D::new(0.0, 1.0, 0.0),
                                    velocity: velocity};

    let environment = Environment{gravity: Vector3D::new(0.0, -0.1, 0.0),
    wind: Vector3D::new(-0.01, 0.0, 0.0)};

    let mut canvas = Canvas::new(900, 550);
    let pixel = Color::new(1.0, 0.5, 0.5);

    while projectile.position.y > 0.0 {
        projectile = tick(projectile, &environment);
        let x = projectile.position.x as u64;
        let y = projectile.position.y as u64;
        if x < 0 || x > 550 {
            break;
        }
        if y < 0 || y > 900 {
            break;
        }
        canvas.put(x, y, pixel);
    }

    canvas.save_ppm("ch_02.ppm".to_string());

}
