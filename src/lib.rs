//! A small library for generating badge svg images
//! 
//! ## Example
//! 
//! ```rust
//! use badges::{BadgeBuilder, BadgeColor, BadgeStyle};
//! 
//! let badge_svg = BadgeBuilder::new()
//! 	.style(BadgeStyle::Flat)
//! 	.label("badge")
//! 	.message("rendered")
//! 	.message_color(BadgeColor::Green)
//! 	.render()
//! 	.expect("failed to render badge");
//! ```
//! 
//! ## Command Line Usage
//! 
//! Install with `cargo install badges --features cli`. 
//! 
//! Use with `badges 'label' 'message' --label-color grey --message-color green > badge.svg`.
//! 
#![warn(missing_docs)]
#![forbid(unsafe_code)]

use std::borrow::Cow;

use error::BadgeError;

pub mod error;
#[cfg(feature = "font-shape")]
mod font;
mod template;

/// Available badge colors (based on GitLab's colors)
#[derive(Clone, Debug, PartialEq, Eq)]
// unused variants are here for possible future use
#[allow(dead_code)]
pub enum BadgeColor {
    /// Green, #4c1
    Green,
    /// Light green, #a3c51c
    LightGreen,
    /// Yellow, #dfb317
    Yellow,
    /// Red, #e05d44
    Red,
    /// Light grey, #9f9f9f9
    LightGrey,
    /// Grey, #555
    Grey,
    /// A custom color
    CustomRgb(u8, u8, u8),
}

/// The style of a badge
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum BadgeStyle {
    /// flat badge
    Flat,
    /// flat badge with square corners
    FlatSquare,
}

impl BadgeColor {
    /// Get the hex value of this color
    pub fn as_hex_str(&self) -> Cow<'static, str> {
        match self {
            Self::Green => "#4c1".into(),
            Self::LightGreen => "#a3c51c".into(),
            Self::Yellow => "#dfb317".into(),
            Self::Red => "#e05d44".into(),
            Self::LightGrey => "#9f9f9f".into(),
            Self::Grey => "#555".into(),
            Self::CustomRgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b).into(),
        }
    }

    /// Helper function for parsing
    fn parse_hex_color(color: &str) -> Result<u8, ()> {
        let byte = u8::from_str_radix(color, 16).map_err(|_| ())?;
        if color.len() == 1 {
            Ok(byte * 16 + byte)
        } else if color.len() == 2 {
            Ok(byte)
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a str> for BadgeColor {
    type Error = BadgeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "grey" | "gray" => Ok(Self::Grey),
            "light-grey" | "light-gray" => Ok(Self::LightGrey),
            "red" => Ok(Self::Red),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "light-green" => Ok(Self::LightGreen),
            _ => {
                let Some(hex_color) = value.strip_prefix('#') else {
                    return Err(BadgeError::InvalidColor(value.to_string()))
                };
                let (r, g, b) = match hex_color.len() {
                    3 => {
                        let r = Self::parse_hex_color(&hex_color[0..=0])
                            .map_err(|_| BadgeError::invalid_color(value))?;
                        let g = Self::parse_hex_color(&hex_color[1..=1])
                            .map_err(|_| BadgeError::invalid_color(value))?;
                        let b = Self::parse_hex_color(&hex_color[2..=2])
                            .map_err(|_| BadgeError::invalid_color(value))?;
                        (r, g, b)
                    }
                    6 => {
                        let r = Self::parse_hex_color(&hex_color[0..=1])
                            .map_err(|_| BadgeError::invalid_color(value))?;
                        let g = Self::parse_hex_color(&hex_color[2..=3])
                            .map_err(|_| BadgeError::invalid_color(value))?;
                        let b = Self::parse_hex_color(&hex_color[4..=5])
                            .map_err(|_| BadgeError::invalid_color(value))?;
                        (r, g, b)
                    }
                    _ => return Err(BadgeError::InvalidColor(value.to_string())),
                };
                Ok(Self::CustomRgb(r, g, b))
            }
        }
    }
}

/// Builder for rendering a badge
pub struct BadgeBuilder<'a> {
    style: BadgeStyle,
    label: &'a str,
    label_color: BadgeColor,
    message: &'a str,
    message_color: BadgeColor,
}

impl<'a> Default for BadgeBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> BadgeBuilder<'a> {
    /// Create a new default [`BadgeBuilder`]
    pub fn new() -> Self {
        Self {
            style: BadgeStyle::Flat,
            label: "badge",
            label_color: BadgeColor::Grey,
            message: "example",
            message_color: BadgeColor::LightGrey,
        }
    }

    /// Change the style of the generated badge
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }

    /// Change the label text of the generated badge
    pub fn label(mut self, text: &'a str) -> Self {
        self.label = text;
        self
    }

    /// Change the label background color of the generated badge
    pub fn label_color(mut self, color: BadgeColor) -> Self {
        self.label_color = color;
        self
    }

    /// Change the message text of the generated badge
    pub fn message(mut self, text: &'a str) -> Self {
        self.message = text;
        self
    }

    /// Change the message background color of the generated badge
    pub fn message_color(mut self, color: BadgeColor) -> Self {
        self.message_color = color;
        self
    }

    /// Render the created badge
    pub fn render(self) -> Result<String, BadgeError> {
        use askama::Template;

        let badge = template::BadgeTemplate::new(
            self.style,
            self.label,
            self.label_color,
            self.message,
            self.message_color,
        );
        Ok(badge.render()?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        assert_eq!(
            BadgeColor::Green,
            BadgeColor::try_from("green").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::LightGreen,
            BadgeColor::try_from("light-green").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::Yellow,
            BadgeColor::try_from("yellow").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::Red,
            BadgeColor::try_from("red").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::Grey,
            BadgeColor::try_from("grey").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::Grey,
            BadgeColor::try_from("gray").expect("failed to parse")
        );
        BadgeColor::try_from("").expect_err("parsing should have failed");
        BadgeColor::try_from("abcdefgh").expect_err("parsing should have failed");
        BadgeColor::try_from("123").expect_err("parsing should have failed");
        BadgeColor::try_from("aaa").expect_err("parsing should have failed");
        BadgeColor::try_from("#a").expect_err("parsing should have failed");
        BadgeColor::try_from("#aag").expect_err("parsing should have failed");
        BadgeColor::try_from("#aaaaa").expect_err("parsing should have failed");
        BadgeColor::try_from("#aaaaag").expect_err("parsing should have failed");
        assert_eq!(
            BadgeColor::CustomRgb(170, 170, 170),
            BadgeColor::try_from("#aaaaaa").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::CustomRgb(170, 170, 170),
            BadgeColor::try_from("#aaa").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::CustomRgb(171, 205, 239),
            BadgeColor::try_from("#abcdef").expect("failed to parse")
        );
    }

    #[test]
    fn test_as_str() {
        assert_eq!(
            "#aaaaaa",
            BadgeColor::try_from("#aaaaaa")
                .expect("failed to parse")
                .as_hex_str(),
        );
        assert_eq!(
            "#aaaaaa",
            BadgeColor::try_from("#AAA")
                .expect("failed to parse")
                .as_hex_str(),
        );
        assert_eq!(
            "#010203",
            BadgeColor::try_from("#010203")
                .expect("failed to parse")
                .as_hex_str(),
        );
    }
}
