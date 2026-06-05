use std::cell::Cell;

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

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * 3.1415926535897932385 / 180.0
}

thread_local! {
    pub static RNG: Cell<u32> = Cell::new(12345);
}

pub fn rand_f64() -> f64 {
    RNG.with(|r| {
        let mut x = r.get();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        r.set(x);
        (x as f64) / (u32::MAX as f64)
    })
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
	if linear_component > 0.0 {
		return linear_component.sqrt();
	}
	0.0
}