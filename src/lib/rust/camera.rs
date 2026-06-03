use crate::vec::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::objects::{World};
use crate::utils::{linear_to_gamma, rand_f64, Interval};

#[derive(Default)]
pub struct Camera {
	pub image_width: u64,
	pub image_height: u64,
	pub samples_per_pixel: u64,
	pub max_depth: u64,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Point3,
	pixel_delta_v: Point3,
	pixels: Vec<u8>,
	pixel_samples_scale: f64,
}

impl Camera {
	pub fn new(image_width: u64, image_height: u64, samples_per_pixel: u64, max_depth: u64) -> Self {
		let mut cam: Self = Self { image_width, image_height, samples_per_pixel, max_depth, ..Default::default() };
		cam.initialize();
		cam
	}

	pub fn render(&mut self, world: &World) -> Vec<u8> {
		for j in 0..self.image_height {
			for i in 0..self.image_width {
				let mut pixel_color = Color::zero();
				for _ in 0..self.samples_per_pixel {
					let ray: Ray = self.get_ray(i, j);
					pixel_color = pixel_color + ray.color(world, self.max_depth);
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
		let r: f64 = linear_to_gamma(color.x);
		let g: f64 = linear_to_gamma(color.y);
		let b: f64 = linear_to_gamma(color.z);
		self.pixels.push((256.0 * intensity.clamp(r)) as u8);
		self.pixels.push((256.0 * intensity.clamp(g)) as u8);
		self.pixels.push((256.0 * intensity.clamp(b)) as u8);
		self.pixels.push(255u8);
	}
}