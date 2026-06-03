use std::sync::Arc;
use crate::vec::{Vec3, Point3};
use crate::ray::{Ray};
use crate::materials::{Material};
use crate::utils::{Interval};

pub trait Hittable {
	fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
	pub front_face: bool,
	pub material: Arc<dyn Material>,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
		// Sets the hit record normal vector.
		// NOTE: the parameter `outward_normal` is asumend to have unit length

		self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
		self.normal = if self.front_face { outward_normal } else { -outward_normal };
	}
}