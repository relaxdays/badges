use badges::{BadgeBuilder, BadgeColor, BadgeStyle};
use pretty_assertions::assert_eq;

const TEST_BADGE: &str = include_str!("test_badge.svg");

#[test]
fn render_badge() {
    let badge = BadgeBuilder::new()
        .color(BadgeColor::Green)
        .left_text("badge")
        .right_text("rendered")
        .style(BadgeStyle::Flat)
        .render()
        .expect("failed to render badge");

    assert_eq!(badge, TEST_BADGE);
}
