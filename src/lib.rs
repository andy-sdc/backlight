//! This is a Rust library for controlling the backlight on Linux systems via
//! the /sys/class/backlight interface.
//!
//! [`backlight`]: https://github.com/andy-sdc/backlight.git
//!
//! This crate allows you to:
//! - Get the maximum brightness supported by the backlight. See: [`get_max_brightness()`].
//! - Get the current brightness level. See: [`get_brightness()`].
//! - Get the current brightness level as a percentage of the maximum. See: [`get_percent()`].
//! - Set a new brightness level. See: [`set_brightness()`].
//! - Set a new brightness level as a percentage of the maximum. See: [`set_percent()`].
//!
//! ## Usage examples (see also examples folder)
//!
//! ### Get the maximum allowable brightness level
//!
//! ```no_run
//! extern crate backlight;
//! use backlight::Brightness;
//! 
//! fn main() {
//!     let br = Brightness::new("backlight-lcd");
//!
//!     let max = br.get_max_brightness().unwrap();
//!     println!("Maximum brightness: {}", max);
//! }
//! ```
//!
//! ### Get the current backlight brightness level
//!
//! ```no_run
//! extern crate backlight;
//! use backlight::Brightness;
//! 
//! fn main() {
//!     let br = Brightness::new("backlight-lcd");
//!
//!     let current = br.get_brightness().unwrap();
//!     println!("Current brightness: {}", current);
//! }
//! ```
//!
//! ### Get the current backlight brightness level as a percentage
//!
//! ```no_run
//! extern crate backlight;
//! use backlight::Brightness;
//! 
//! fn main() {
//!     let br = Brightness::new("backlight-lcd");
//!
//!     let percent = br.get_percent().unwrap();
//!     println!("Current brightness: {}%", percent);
//! } 
//! ```
//!
//! ### Set a new brightness level
//!
//! ```no_run
//! extern crate structopt;
//! use structopt::StructOpt;
//!
//! extern crate backlight;
//! use backlight::Brightness;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "backlight", about = "Set the backlight to a specific value")]
//! struct Opt {
//!     brightness: i32,
//! }
//!
//! fn main() {
//!     let opt = Opt::from_args();
//!
//!     let br = Brightness::new("backlight-lcd");
//!     br.set_brightness(opt.brightness).unwrap();
//! }
//! ```
//!
//! ### Set a new brightness level as a percentage of maximum brightness
//!
//! ```no_run
//! extern crate structopt;
//! use structopt::StructOpt;
//!
//! extern crate backlight;
//! use backlight::Brightness;
//!
//! #[derive(Debug, StructOpt)]
//! #[structopt(name = "backlight", about = "Set the backlight to a percentage brightness value")]
//! struct Opt {
//!     brightness: i32,
//! }
//!
//! fn main() {
//!     let opt = Opt::from_args();
//!     if opt.brightness < 1 || opt.brightness > 100 {
//!         panic!("Invalid value set.  Should be between 1 and 100");
//!     }
//!     
//!     let br = Brightness::new("backlight-lcd");
//!     br.set_percent(opt.brightness).unwrap();
//! }
//! ```
//!

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io;
use std::io::Write;
use std::path::{PathBuf};

pub struct Brightness {
	backend: String,
	max_brightness: i32,
}

impl Brightness {
	/// Create a new instance of the backlight device.
	pub fn new(backend_dev: &str) -> Self {
		let backend_path = format!("/sys/class/backlight/{}", backend_dev.to_string());
		Brightness {
			backend: backend_path,
			max_brightness: 0,
		}
	}

	/// Return the maximum brightness supported back the backlight.  Read
	/// it from the file system if it hasn't been got before.
	pub fn get_max_brightness(&self) -> Result<i32, io::Error> {
		if self.max_brightness > 0 {
			return Ok(self.max_brightness);
		}
		return self.get("max_brightness");
	}

	/// Return the current backlight brightness setting.
	pub fn get_brightness(&self) -> Result<i32, io::Error> {
		return self.get("brightness");
	}

	/// Return the current backlight brightness as a percentage
	/// of the maximum level.
	pub fn get_percent(&self) -> Result<i32, io::Error> {
		let value = try!(self.get_brightness()) as f32;
		let max = try!(self.get_max_brightness()) as f32;
		let result = (100 as f32) * value / max;
		return Ok(result as i32);
	}

	/// Set a new brightness level by writing to the file within
	/// the /sys/class/backlight/... structure
	pub fn set_brightness(&self, mut value: i32) -> Result<bool, io::Error> {
		let max = try!(self.get_max_brightness());
		if value > max {
			value = max;
		} else if value < 0 {
			value = 0;
		}

		let mut path_buffer = PathBuf::from(self.backend.clone());
		path_buffer.push("brightness");

		let path = path_buffer.as_path();
		let mut file = try!(OpenOptions::new().write(true).open(path));

		match file.write_all(value.to_string().as_bytes()) {
			Ok(_) => Ok(true),
			Err(err) => Err(err)
		}
	}
	
	/// Set a new backlight brightness level as a percentage of the maximum.
	pub fn set_percent(&self, value: i32) -> Result<bool, io::Error> {
		let max = try!(self.get_max_brightness());
		let value = (value as f32) / (100_f32) * (max as f32) + 0.5_f32;
		let value = value as i32;
		return self.set_brightness(value as i32);
	}
	
	/// Read the file within the /sys/class/backlight/... structure to
	/// get the corresponding value.
	fn get(&self, filename: &str) -> Result<i32, io::Error> {
		let mut path_buffer = PathBuf::from(self.backend.clone());
		path_buffer.push(filename);

		let path = path_buffer.as_path();
		let mut file = try!(File::open(path));

		let mut content = String::new();
		try!(file.read_to_string(&mut content));

		match content.trim().parse::<i32>() {
			Ok(value) => Ok(value),
			Err(_) => {
				Ok(-1)
			}
		}
	}
}
