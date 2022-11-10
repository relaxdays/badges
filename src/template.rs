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
    left_color: &'a str,
    left_text: &'a str,
    right_width: u16,
    right_color: Cow<'a, str>,
    right_text: &'a str,
}

impl<'a> BadgeTemplate<'a> {
    pub fn new(
        style: BadgeStyle,
        color: BadgeColor<'a>,
        left_text: &'a str,
        right_text: &'a str,
    ) -> Self {
        let left_width = Self::measure_text_width(left_text);
        let right_width = Self::measure_text_width(right_text);
        Self {
            style,
            left_width,
            left_color: "#555", // always choose this default as the "background" color
            left_text,
            right_width,
            right_color: color.into_cow(),
            right_text,
        }
    }

    #[cfg(not(feature = "font-shape"))]
    fn measure_text_width(text: &str) -> u16 {
        // todo: improve this heuristic somehow
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
        let template = BadgeTemplate::new(BadgeStyle::Flat, BadgeColor::Green, "badge", "rendered");
        let rendered = template.render().expect("failed to render template");
        let expected = r##"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="20">
<linearGradient id="b" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient>
<mask id="a"><rect width="136" height="20" rx="3" fill="#fff"/></mask>
<g mask="url(#a)">
	<rect width="56" height="20" fill="#555"/>
	<rect x="56" width="80" height="20" fill="#4c1"/>
	<rect width="136" height="20" fill="url(#b)"/>
</g>

<g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
	<text x="28" y="15" fill="#010101" fill-opacity=".3">badge</text>
	<text x="96" y="15" fill="#010101" fill-opacity=".3">rendered</text>
	<text x="28" y="14">badge</text>
	<text x="96" y="14">rendered</text>
</g>
</svg>"##;
        assert_eq!(rendered, expected);
    }

    #[test]
    fn test_flat_square() {
        let template = BadgeTemplate::new(
            BadgeStyle::FlatSquare,
            BadgeColor::Green,
            "badge",
            "rendered",
        );
        let rendered = template.render().expect("failed to render template");
        let expected = r##"<svg xmlns="http://www.w3.org/2000/svg" width="136" height="20">
<g shape-rendering="crispEdges">
	<rect width="56" height="20" fill="#555"/>
	<rect x="56" width="80" height="20" fill="#4c1"/>
</g>

<g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
	<text x="28" y="14">badge</text>
	<text x="96" y="14">rendered</text>
</g>
</svg>"##;
        assert_eq!(rendered, expected);
    }
}
