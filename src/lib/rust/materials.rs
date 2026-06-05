use crate::utils::rand_f64;
use crate::vec::{Vec3, Color};
use crate::ray::{Ray};
use crate::hit::{HitRecord};

pub trait Material {
    fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
	pub fn new(albedo: Color) -> Self {
		Self { albedo }
	}
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let _ = ray_in;
		let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit();
		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}

		*scattered = Ray::new(rec.point, scatter_direction);
		*attenuation = self.albedo;
		true
    }
}

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}

impl Metal {
	pub fn new(albedo: Color, fuzz: f64) -> Self {
		Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
	}
}

impl Material for Metal {
	fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let mut reflected: Vec3 = Vec3::reflect(ray_in.direction, rec.normal);
		reflected = reflected.normalize() + (Vec3::random_unit() * self.fuzz);
		*scattered = Ray::new(rec.point, reflected);
		*attenuation = self.albedo;
		
		scattered.direction.dot(rec.normal) > 0.0
	}
}

pub struct Dielectric {
	pub refraction_index: f64,
}

impl Dielectric {
	pub fn new(refraction_index: f64) -> Self {
		Self { refraction_index }
	}

	fn reflectance(cos: f64, refraction_index: f64) -> f64 {
		// Use Schlick's approximation for reflectance.
		let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
		r0 = r0 * r0;
		r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
	}
}

impl Material for Dielectric {
	fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		*attenuation = Color::new(1.0, 1.0, 1.0);
		let ri: f64 = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

		let unit_dir: Vec3 = ray_in.direction.normalize();
		let cos_theta: f64 = f64::min(Vec3::dot(-unit_dir, rec.normal), 1.0);
		let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

		let cannot_refract: bool = ri * sin_theta > 1.0;
		let direction: Vec3;

		if cannot_refract || (Dielectric::reflectance(cos_theta, ri) > rand_f64()) {
			direction = Vec3::reflect(unit_dir, rec.normal);
		} else {
			direction = Vec3::refract(unit_dir, rec.normal, ri);
		}

		*scattered = Ray::new(rec.point, direction);
		true
	}
}