use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

type Color = Vec3;
type Point3 = Vec3;

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self { 
		Self { x, y, z } 
	}

	pub fn zero() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}

	pub fn length_squared(self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn length(self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn normalize(self) -> Self {
		self / self.length()
	}

	pub fn dot(self, other: Self) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn cross(self, other: Self) -> Self {
		Self::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x- self.x * other.z,
			self.x * other.y - self.y * other.x
		)
	}
}

use std::ops::{Add, Sub, Mul, Div, Neg};

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Default for Vec3 {
	fn default() -> Self {
		Self::zero()
	}
}

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

	pub fn color(self, world: &World) -> Color {
		if let Some(hit) = world.hit(self, Interval::new(0.0, f64::INFINITY)) {
			return (hit.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
		}

		let unit_direction: Vec3 = self.direction.normalize();
		let a: f64 = 0.5 * (unit_direction.y + 1.0);
		Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
	}
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
		// Sets the hit record normal vector.
		// NOTE: the parameter `outward_normal` is asumend to have unit length

		self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
		self.normal = if self.front_face { outward_normal } else { -outward_normal };
	}
}

pub trait Hittable {
	fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
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

pub struct Interval {
	pub min: f64,
	pub max: f64,
}

impl Interval {
	pub fn empty() -> Self {
		Self { min: f64::INFINITY, max: -f64::INFINITY }
	}

	pub fn universe() -> Self {
		Self {min: -f64::INFINITY, max: f64::INFINITY }
	}

	pub fn new(min: f64, max: f64) -> Self {
		Self { min, max }
	}

	pub fn size(&self) -> f64 {
		self.max - self.min
	}

	pub fn contains(&self, x: f64) -> bool {
		(self.min <= x) && (x <= self.max)
	}

	pub fn surrounds(&self, x: f64) -> bool {
		(self.min < x) && (x < self.max)
	}

	pub fn clamp(&self, x: f64) -> f64 {
		if x < self.min { return self.min };
		if x > self.max { return self.max };
		x
	}
}

#[derive(Default)]
pub struct Camera {
	pub image_width: u64,
	pub image_height: u64,
	pub samples_per_pixel: u64,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Point3,
	pixel_delta_v: Point3,
	pixels: Vec<u8>,
	pixel_samples_scale: f64,
}

impl Camera {
	pub fn new(image_width: u64, image_height: u64, samples_per_pixel: u64) -> Self {
		let mut cam: Self = Self { image_width, image_height, samples_per_pixel, ..Default::default() };
		cam.initialize();
		cam
	}

	pub fn render(&mut self, world: &World) -> Vec<u8> {
		for j in 0..self.image_height {
			for i in 0..self.image_width {
				let mut pixel_color = Color::zero();
				for _ in 0..self.samples_per_pixel {
					let ray: Ray = self.get_ray(i, j);
					pixel_color = pixel_color + ray.color(world);
				}
	
				self.write_color(pixel_color * self.pixel_samples_scale);
			}
		}
	
		std::mem::take(&mut self.pixels)
	}

	fn initialize(&mut self) {
		self.pixels = Vec::with_capacity((self.image_width * self.image_height * 4) as usize);

		self.center = Point3::zero();

		self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

		let focal_length: f64 = 1.0;
		let viewport_height: f64 = 2.0;
		let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

		// Calculate the vectors across the horizontal and down the vertical viewport edges.
		let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
		let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

		// Calculate the horizontal and vertical delta vectors from pixel to pixel.
		self.pixel_delta_u = viewport_u / (self.image_width as f64);
		self.pixel_delta_v = viewport_v / (self.image_height as f64);

		// Calculate the location of the upper left pixel.
		let viewport_upper_left: Vec3 = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
		self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
	}

	fn get_ray(&self, i: u64, j: u64) -> Ray {
		let offset: Vec3 = self.sample_square();
		let pixel_sample: Vec3 = self.pixel00_loc 
			+ (self.pixel_delta_u * (i as f64 + offset.x)) 
			+ (self.pixel_delta_v * (j as f64 + offset.y));

		let ray_origin: Point3 = self.center;
		let ray_direction: Vec3 = pixel_sample - ray_origin;

		Ray::new(ray_origin, ray_direction)
	}

	fn sample_square(&self) -> Vec3 {
		Vec3::new(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
	}

	fn write_color(&mut self, color: Color) {
		let intensity: Interval = Interval::new(0.000, 0.999);
		self.pixels.push((256.0 * intensity.clamp(color.x)) as u8);
		self.pixels.push((256.0 * intensity.clamp(color.y)) as u8);
		self.pixels.push((256.0 * intensity.clamp(color.z)) as u8);
		self.pixels.push(255u8);
	}
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * 3.1415926535897932385 / 180.0
}

use std::cell::Cell;
thread_local! {
    static RNG: Cell<u32> = Cell::new(12345);
}

fn rand_f64() -> f64 {
    RNG.with(|r| {
        let mut x = r.get();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        r.set(x);
        (x as f64) / (u32::MAX as f64)
    })
}

fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}

#[wasm_bindgen]
pub fn render(width: u64, height: u64, samples_per_pixel: u64) -> Vec<u8> {
	let mut world: World = World::new();
	world.add(Box::new(
		Sphere {
			center: Point3::new(0.0, 0.0, -1.0),
			radius: 0.5,
		}
	));
	world.add(Box::new(
		Sphere {
			center: Point3::new(0.0, -100.5, -1.0),
			radius: 100.0,
		}
	));

	let mut camera: Camera = Camera::new(width, height, samples_per_pixel);
	camera.render(&world)
}