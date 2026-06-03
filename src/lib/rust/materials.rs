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
}

impl Material for Dielectric {
	fn scatter(&self, ray_in: Ray, rec: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		*attenuation = Color::new(1.0, 1.0, 1.0);
		let ri: f64 = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

		let unit_dir: Vec3 = ray_in.direction.normalize();
		let refracted: Vec3 = Vec3::refract(unit_dir, rec.normal, ri);

		*scattered = Ray::new(rec.point, refracted);
		true
	}
}