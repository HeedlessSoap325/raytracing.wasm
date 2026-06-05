use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::utils::{rand_f64, rand_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

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

	pub fn random() -> Self {
		Self::new(rand_f64(), rand_f64(), rand_f64())
	}

	pub fn random_range(min: f64, max: f64) -> Self {
		Self::new(rand_range(min, max), rand_range(min, max), rand_range(min, max))
	}

	pub fn random_unit() -> Self {
		// Use the Marsaglia method directly on the unit sphere
		loop {
			let u = rand_f64() * 2.0 - 1.0;
			let v = rand_f64() * 2.0 - 1.0;
			let s = u * u + v * v;
			if s < 1.0 {
				let f = 2.0 * (1.0 - s).sqrt();
				return Self::new(u * f, v * f, 1.0 - 2.0 * s);
			}
		}
	}

	pub fn random_on_hemisphere(&self) -> Self {
		let on_unit_sphere: Self = Self::random_unit();
		if Self::dot(on_unit_sphere, *self) > 0.0 {
			return on_unit_sphere;
		} else {
			return -on_unit_sphere;
		}
	}

	pub fn near_zero(&self) -> bool {
		let s = 1e-8;
		(self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
	}

	pub fn reflect(v: Vec3, n: Vec3) -> Self {
		v - n * v.dot(n) * 2.0
	}

	pub fn refract(uv: Vec3, n: Vec3, etai_overt_etat: f64) -> Self {
		let cos_theta: f64 = f64::min(Self::dot(-uv, n), 1.0);
		let r_out_perp: Vec3 = (uv + n * cos_theta) * etai_overt_etat;
		let r_out_parallel: Vec3 = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
		r_out_perp + r_out_parallel
	}
}

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