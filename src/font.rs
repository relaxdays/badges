use once_cell::sync::Lazy;
use rustybuzz::{Face, UnicodeBuffer};

/// Byte data for known fonts
const FONT_DATA: [&[u8]; 1] = [include_bytes!("../fonts/DejaVuSans.ttf")];

/// The same font size as specified in the template
const FONT_SIZE: f32 = 11.0;

/// Padding to add to the total width of the shaped text
const PADDING: i32 = 24;

/// Parsed data for known fonts
static FONTS: Lazy<[Face; 1]> = Lazy::new(|| {
    let dejavu_sans = Face::from_slice(FONT_DATA[0], 0).unwrap();
    [dejavu_sans]
});

pub fn measure_text(text: &str) -> u16 {
    let face = &FONTS[0];
    let features = [];
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);

    // calculate total x advance
    let buffer = rustybuzz::shape(face, &features, buffer);
    let mut x = 0;
    for pos in buffer.glyph_positions() {
        x += pos.x_advance;
    }

    // convert x advance from font units to pixels
    let x = (x as f32) / (face.units_per_em() as f32) * FONT_SIZE;
    let x = x as i32 + PADDING;

    if x > u16::MAX as _ {
        u16::MAX
    } else {
        x as _
    }
}
