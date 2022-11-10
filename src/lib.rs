//! A small library for generating badge svg images
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
pub enum BadgeColor<'a> {
    /// Green, #4c1
    Green,
    /// Light green, #a3c51c
    LightGreen,
    /// Yellow, #dfb317
    Yellow,
    /// Red, #e05d44
    Red,
    /// Grey, #9f9f9f9
    Grey,
    /// A custom color
    ///
    /// Must be a valid color in hexadecimal rgb format
    Custom(Cow<'a, str>),
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

impl<'a> BadgeColor<'a> {
    /// Get the hex value of this color
    pub fn as_str<'b: 'a>(&'b self) -> &'b str {
        match self {
            Self::Green => "#4c1",
            Self::LightGreen => "#a3c51c",
            Self::Yellow => "#dfb317",
            Self::Red => "#e05d44",
            Self::Grey => "#9f9f9f",
            Self::Custom(color) => color.as_ref(),
        }
    }

    /// Get a [`Cow`] of the contained color
    pub fn into_cow(self) -> Cow<'a, str> {
        match self {
            Self::Green => "#4c1".into(),
            Self::LightGreen => "#a3c51c".into(),
            Self::Yellow => "#dfb317".into(),
            Self::Red => "#e05d44".into(),
            Self::Grey => "#9f9f9f".into(),
            Self::Custom(color) => color,
        }
    }

    /// Convert this [`BadgeColor`] into one with a `'static` lifetime
    ///
    /// Inner data is cloned if necessary.
    pub fn into_static(self) -> BadgeColor<'static> {
        match self {
            Self::Green => BadgeColor::Green,
            Self::LightGreen => BadgeColor::LightGreen,
            Self::Yellow => BadgeColor::Yellow,
            Self::Red => BadgeColor::Red,
            Self::Grey => BadgeColor::Grey,
            Self::Custom(cow) => BadgeColor::Custom(Cow::Owned(cow.into_owned())),
        }
    }
}

impl<'a> TryFrom<&'a str> for BadgeColor<'a> {
    type Error = BadgeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "grey" | "gray" => Ok(Self::Grey),
            "red" => Ok(Self::Red),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "light-green" => Ok(Self::LightGreen),
            _ => {
                let Some(hex_color) = value.strip_prefix('#') else {
                    return Err(BadgeError::InvalidColor(value.to_string()))
                };
                if hex_color.len() != 3 && hex_color.len() != 6 {
                    return Err(BadgeError::InvalidColor(value.to_string()));
                }
                if hex_color.contains(|c: char| !c.is_ascii_hexdigit()) {
                    return Err(BadgeError::InvalidColor(value.to_string()));
                }
                Ok(Self::Custom(Cow::Borrowed(value)))
            }
        }
    }
}

/// Builder for rendering a badge
pub struct BadgeBuilder<'a> {
    style: BadgeStyle,
    left_text: &'a str,
    right_text: &'a str,
    label_color: BadgeColor<'a>,
}

impl<'a> BadgeBuilder<'a> {
    /// Create a new default [`BadgeBuilder`]
    pub fn new() -> Self {
        Self {
            style: BadgeStyle::Flat,
            left_text: "badge",
            right_text: "example",
            label_color: BadgeColor::Grey,
        }
    }

    /// Change the style of the generated badge
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }

    /// Change the text on the left side of the generated badge
    pub fn left_text(mut self, text: &'a str) -> Self {
        self.left_text = text;
        self
    }

    /// Change the text on the right side of the generated badge
    pub fn right_text(mut self, text: &'a str) -> Self {
        self.right_text = text;
        self
    }

    /// Change the background color of the right side of the generated badge
    pub fn color(mut self, color: BadgeColor<'a>) -> Self {
        self.label_color = color;
        self
    }

    /// Render the created badge
    pub fn render(self) -> Result<String, BadgeError> {
        use askama::Template;

        let badge = template::BadgeTemplate::new(
            self.style,
            self.label_color,
            self.left_text,
            self.right_text,
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
            BadgeColor::Custom(Cow::Borrowed("#aaaaaa")),
            BadgeColor::try_from("#aaaaaa").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::Custom(Cow::Borrowed("#aaa")),
            BadgeColor::try_from("#aaa").expect("failed to parse")
        );
        assert_eq!(
            BadgeColor::Custom(Cow::Borrowed("#abcdef")),
            BadgeColor::try_from("#abcdef").expect("failed to parse")
        );
    }
}
