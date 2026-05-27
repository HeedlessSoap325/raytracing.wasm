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