# backlight

This is a Rust library for controlling the backlight on Linux systems via
the /sys/class/backlight interface.

[`backlight`]: https://github.com/andy-sdc/backlight.git

This crate allows you to:
- Get the maximum brightness supported by the backlight. See: [`get_max_brightness()`].
- Get the current brightness level. See: [`get_brightness()`].
- Get the current brightness level as a percentage of the maximum. See: [`get_percent()`].

### Usage examples (see also examples folder)

#### Get the maximum allowable brightness level

```rust
extern crate backlight;
use backlight::{get_max_brightness};

fn main() {
	  let br = brightness::Brightness::new("backlight-lcd").unwrap();

  let max = br.get_max_brightness().unwrap();
	  println!("Maximum brightness: {}", max);
}
```

#### Get the current backlight brightness level

```rust
extern crate backlight;
use backlight::{get_brightness};

fn main() {
	  let br = brightness::Brightness::new("backlight-lcd").unwrap();

  let current = br.get_brightness().unwrap();
  println!("Current brightness: {}", current);
}
```

#### Get the current backlight brightness level as a percentage

```rust
extern crate backlight;
use backlight::{get_percent};

fn main() {
	    let br = brightness::Brightness::new("backlight-lcd").unwrap();

    let percent = br.get_percent().unwrap();
    println!("Current brightness: {} percent", percent);
}
```


License: MIT
