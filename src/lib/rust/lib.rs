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

	pub fn color(self, sphere: Sphere) -> Color {
		if sphere.hit(self)  {
			return Color::new(1.0, 0.0, 0.0);
		}

		let unit_direction: Vec3 = self.direction.normalize();
		let a: f64 = 0.5 * (unit_direction.y + 1.0);
		Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
	}
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    // Returns the t value of the nearest hit, or None if no intersection.
    pub fn hit(&self, ray: Ray) -> bool {
        let oc: Vec3 = self.center - ray.origin;
		let a: f64 = Vec3::dot(ray.direction, ray.direction);
		let b: f64 = -2.0 * Vec3::dot(ray.direction, oc);
		let c: f64 = Vec3::dot(oc, oc) - self.radius * self.radius;
		let discriminant: f64 = b * b - 4.0 * a * c;
		discriminant >= 0.0
    }
}


fn write_color(pixels: &mut Vec<u8>, color: Color) {
	pixels.push((color.x * 255.999) as u8);
    pixels.push((color.y * 255.999) as u8);
    pixels.push((color.z * 255.999) as u8);
    pixels.push(255u8);
}

#[wasm_bindgen]
pub fn render(width: u64, height: u64) -> Vec<u8> {
	let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

	let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (width as f64 / height as f64);
    let camera_center: Point3 = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (width as f64);
    let pixel_delta_v = viewport_v / (height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Vec3 = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

	let sphere: Sphere = Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 };

	for j in 0..height {
		for i in 0..width {
			let pixel_center: Point3 = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction: Vec3 = pixel_center - camera_center;
            let ray: Ray = Ray::new(camera_center, ray_direction);

			write_color(&mut pixels, ray.color(sphere));
		}
	}

	pixels
}