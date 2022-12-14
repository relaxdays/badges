//! Templating things
use std::borrow::Cow;

use askama::Template;

use crate::{BadgeColor, BadgeStyle};

/// Internal template implementation
#[derive(Template)]
#[template(path = "badge.svg", escape = "xml")]
pub(crate) struct BadgeTemplate<'a> {
    style: BadgeStyle,
    left_width: u16,
    left_color: Cow<'static, str>,
    left_text: &'a str,
    right_width: u16,
    right_color: Cow<'static, str>,
    right_text: &'a str,
}

impl<'a> BadgeTemplate<'a> {
    pub fn new(
        style: BadgeStyle,
        label: &'a str,
        label_color: BadgeColor,
        message: &'a str,
        message_color: BadgeColor,
    ) -> Self {
        let left_width = Self::measure_text_width(label);
        let right_width = Self::measure_text_width(message);
        Self {
            style,
            left_width,
            left_color: label_color.as_hex_str(),
            left_text: label,
            right_width,
            right_color: message_color.as_hex_str(),
            right_text: message,
        }
    }

    #[cfg(not(feature = "font-shape"))]
    fn measure_text_width(text: &str) -> u16 {
        let width = text.chars().count() * 8 + 16;
        if width > u16::MAX as _ {
            u16::MAX
        } else {
            width as _
        }
    }

    #[cfg(feature = "font-shape")]
    fn measure_text_width(text: &str) -> u16 {
        crate::font::measure_text(text)
    }
}

#[cfg(test)]
mod test {
    use askama::Template;
    use pretty_assertions::assert_eq;

    use super::{BadgeColor, BadgeStyle, BadgeTemplate};

    #[test]
    fn test_flat() {
        let template = BadgeTemplate::new(
            BadgeStyle::Flat,
            "badge",
            BadgeColor::Grey,
            "rendered",
            BadgeColor::Green,
        );
        let rendered = template.render().expect("failed to render template");
        let expected = r##"<svg xmlns="http://www.w3.org/2000/svg" width="131" height="20">
<linearGradient id="b" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient>
<mask id="a"><rect width="131" height="20" rx="3" fill="#fff"/></mask>
<g mask="url(#a)">
	<rect width="58" height="20" fill="#555"/>
	<rect x="58" width="73" height="20" fill="#4c1"/>
	<rect width="131" height="20" fill="url(#b)"/>
</g>

<g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
	<text x="29" y="15" fill="#010101" fill-opacity=".3">badge</text>
	<text x="94" y="15" fill="#010101" fill-opacity=".3">rendered</text>
	<text x="29" y="14">badge</text>
	<text x="94" y="14">rendered</text>
</g>
</svg>"##;
        assert_eq!(rendered, expected);
    }

    #[test]
    fn test_flat_square() {
        let template = BadgeTemplate::new(
            BadgeStyle::FlatSquare,
            "badge",
            BadgeColor::Grey,
            "rendered",
            BadgeColor::Green,
        );
        let rendered = template.render().expect("failed to render template");
        let expected = r##"<svg xmlns="http://www.w3.org/2000/svg" width="131" height="20">
<g shape-rendering="crispEdges">
	<rect width="58" height="20" fill="#555"/>
	<rect x="58" width="73" height="20" fill="#4c1"/>
</g>

<g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
	<text x="29" y="14">badge</text>
	<text x="94" y="14">rendered</text>
</g>
</svg>"##;
        assert_eq!(rendered, expected);
    }
}
