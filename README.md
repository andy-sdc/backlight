# backlight

This is a Rust library for controlling the backlight on Linux systems via
the /sys/class/backlight interface.  It was originally developed by 
[Jerko Steiner](https://github.com/jeremija/backlight) and modified by
[Fabian Neumann](https://github.com/hellp/backlight) prior to being
brought up to date and submitted for full listing on [crates.io](https://crates.io/).

[`backlight`]: https://github.com/andy-sdc/backlight.git

This crate allows you to:
- Get the maximum brightness supported by the backlight. See: [`get_max_brightness()`].
- Get the current brightness level. See: [`get_brightness()`].
- Get the current brightness level as a percentage of the maximum. See: [`get_percent()`].
- Set a new brightness level. See: [`set_brightness()`].
- Set a new brightness level as a percentage of the maximum. See: [`set_percent()`].

### Usage examples (see also examples folder)

#### Get the maximum allowable brightness level

```rust
extern crate backlight;
use backlight::Brightness;

fn main() {
    let br = Brightness::new("backlight-lcd");

    let max = br.get_max_brightness().unwrap();
    println!("Maximum brightness: {}", max);
}
```

#### Get the current backlight brightness level

```rust
extern crate backlight;
use backlight::Brightness;

fn main() {
    let br = Brightness::new("backlight-lcd");

    let current = br.get_brightness().unwrap();
    println!("Current brightness: {}", current);
}
```

#### Get the current backlight brightness level as a percentage

```rust
extern crate backlight;
use backlight::Brightness;

fn main() {
    let br = Brightness::new("backlight-lcd");

    let percent = br.get_percent().unwrap();
    println!("Current brightness: {}%", percent);
}
```

#### Set a new brightness level

```rust
extern crate structopt;
use structopt::StructOpt;

extern crate backlight;
use backlight::Brightness;

#[derive(Debug, StructOpt)]
#[structopt(name = "backlight", about = "Set the backlight to a specific value")]
struct Opt {
    brightness: i32,
}

fn main() {
    let opt = Opt::from_args();

    let br = Brightness::new("backlight-lcd");
    br.set_brightness(opt.brightness).unwrap();
}
```

#### Set a new brightness level as a percentage of maximum brightness

```rust
extern crate structopt;
use structopt::StructOpt;

extern crate backlight;
use backlight::Brightness;

#[derive(Debug, StructOpt)]
#[structopt(name = "backlight", about = "Set the backlight to a percentage brightness value")]
struct Opt {
    brightness: i32,
}

fn main() {
    let opt = Opt::from_args();
    if opt.brightness < 1 || opt.brightness > 100 {
        panic!("Invalid value set.  Should be between 1 and 100");
    }

    let br = Brightness::new("backlight-lcd");
    br.set_percent(opt.brightness).unwrap();
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/sdcsystems/backlight/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

