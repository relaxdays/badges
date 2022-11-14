use badges::{BadgeBuilder, BadgeColor, BadgeStyle};
use pretty_assertions::assert_eq;

const TEST_BADGE: &str = include_str!("test_badge.svg");

#[test]
fn render_badge() {
    let badge = BadgeBuilder::new()
        .message_color(BadgeColor::Green)
        .label("badge")
        .message("rendered")
        .style(BadgeStyle::Flat)
        .render()
        .expect("failed to render badge");

    assert_eq!(badge, TEST_BADGE);
}
