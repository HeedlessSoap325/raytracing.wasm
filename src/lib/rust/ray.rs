use crate::vec::{Vec3, Point3, Color};
use crate::objects::{World};
use crate::hit::{Hittable};
use crate::utils::{Interval};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
	pub origin: Point3,
	pub direction: Vec3,
}

impl Ray {
	pub fn new(origin: Point3, direction: Vec3) -> Self {
		Self { origin, direction }
	}

	pub fn at(self, t: f64) -> Point3 {
		self.origin + self.direction * t
	}

	pub fn color(self, world: &World, depth: u64) -> Color {
		if depth <= 0 {
			return Color::zero();
		}

		if let Some(hit) = world.hit(self, Interval::new(0.001, f64::INFINITY)) {
			let mut scattered: Ray = Ray::new(Point3::zero(), Vec3::zero());
			let mut attenuation: Color = Color::zero();

			if hit.material.clone().scatter(self, hit, &mut attenuation, &mut scattered) {
				return attenuation * scattered.color(world, depth - 1);
			}

			return Color::zero();
		}

		let unit_direction: Vec3 = self.direction.normalize();
		let a: f64 = 0.5 * (unit_direction.y + 1.0);
		Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
	}
}