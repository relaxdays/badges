# badges

A small library to render badges. Includes a binary for command line usage.

## Example

```rust
use badges::{BadgeBuilder, BadgeColor, BadgeStyle};

let badge_svg = BadgeBuilder::new()
	.style(BadgeStyle::Flat)
	.label("badge")
	.message("rendered")
	.message_color(BadgeColor::Green)
	.render()
	.expect("failed to render badge");
```

## Command Line Usage

Install with `cargo install badges --features cli`. Use with `badges 'label' 'message' --label-color grey --message-color green > badge.svg`.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-Apache-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The included DejaVu Sans font is licensed as described [here](https://dejavu-fonts.github.io/License.html).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted  for inclusion in the fork by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
