use std::sync::Arc;
use crate::vec::{Vec3, Point3};
use crate::materials::{Material};
use crate::hit::{Hittable, HitRecord};
use crate::ray::{Ray};
use crate::utils::{Interval};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
	pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
	fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - ray.origin;
		let a: f64 = ray.direction.length_squared();
		let h: f64 = Vec3::dot(ray.direction, oc);
		let c: f64 = oc.length_squared() - self.radius * self.radius;

		let discriminant: f64 = h * h - a * c;

		if discriminant < 0.0 {
			return None;
		}

		let sqrtd: f64 = discriminant.sqrt();
		let mut root: f64 = (h - sqrtd) / a;
		if !ray_t.surrounds(root) {
			root = (h + sqrtd) / a;
			if !ray_t.surrounds(root) {
				return None;
			}
		}

		let mut hit_record: HitRecord = HitRecord {
			t: root,
			point: ray.at(root),
			normal: (ray.at(root) - self.center) / self.radius,
			front_face: false,
			material: Arc::clone(&self.material),
		};

		let outward_normal: Vec3 = (hit_record.point - self.center) / self.radius;
		hit_record.set_face_normal(ray, outward_normal);

		Some(hit_record)
    }
}

pub struct World {
	pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
	pub fn new() -> Self {
		Self { objects: Vec::new() }
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add(&mut self, object: Box<dyn Hittable>) {
		self.objects.push(object);
	}
}

impl Hittable for World {
	fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
		let mut rec: Option<HitRecord> = None;
		let mut closest_so_far: f64 = ray_t.max;

		for object in &self.objects {
			if let Some(hit) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
				closest_so_far = hit.t;
				rec = Some(hit);
			}
		}

		rec
	}
}