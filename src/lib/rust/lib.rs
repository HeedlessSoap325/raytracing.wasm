mod camera;
mod vec;
mod ray;
mod objects;
mod hit;
mod materials;
mod utils;

use wasm_bindgen::prelude::*;
use std::sync::Arc;
use crate::camera::{Camera};
use crate::materials::{Lambertian, Dielectric, Metal};
use crate::objects::{World, Sphere};
use crate::utils::RNG;
use crate::vec::{Color, Point3};

#[wasm_bindgen]
pub fn render_band(width: u64, height: u64, samples_per_pixel: u64, max_depth: u64, vfov: f64, row_start: u64, row_end: u64, seed: u32) -> Vec<u8> {
	RNG.with(|r| r.set(seed)); // seed the thread-local RNG with the provided seed
	
	let material_ground: Lambertian = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center: Lambertian = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left: Dielectric   = Dielectric::new(1.5);
    let material_bubble: Dielectric = Dielectric::new(1.0 / 1.5);
    let material_right: Metal 		= Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

	let mut world: World = World::new();
	world.add(Box::new(
		Sphere {
			center: Point3::new(0.0, -100.5, -1.0),
			radius: 100.0,
			material: Arc::new(material_ground),
		}
	));
	world.add(Box::new(
		Sphere {
			center: Point3::new(0.0, 0.0, -1.2),
			radius: 0.5,
			material: Arc::new(material_center),
		}
	));
	world.add(Box::new(
		Sphere {
			center: Point3::new(-1.0, 0.0, -1.0),
			radius: 0.5,
			material: Arc::new(material_left),
		}
	));
	world.add(Box::new(
		Sphere {
			center: Point3::new(-1.0, 0.0, -1.0),
			radius: 0.4,
			material: Arc::new(material_bubble),
		}
	));
	world.add(Box::new(
		Sphere {
			center: Point3::new(1.0, 0.0, -1.0),
			radius: 0.5,
			material: Arc::new(material_right),
		}
	));

	let mut camera: Camera = Camera::new();
	camera.image_width = width;
	camera.image_height = height;
	camera.samples_per_pixel = samples_per_pixel;
	camera.max_depth = max_depth;
	camera.vfov = vfov;
	camera.render_band(&world, row_start, row_end)
}