use image::RgbImage;
use ndarray::Array3;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// Names for real colors
#[allow(unused)]
pub enum Colors {
    Black,
    White,
    Red,
    Green,
    Blue,
    // Nice Colors
    DarkRed,
    NavyBlue,
    DarkBlue,
    SunYellow,
    // Grays
    FaintGray,
    LightGray,
    MediumGray,
    HeavyGray,
    DarkGray,
    // Space Grays
    SpaceFaintGray,
    SpaceLightGray,
    SpaceMediumGray,
    SpaceHeavyGray,
    SpaceDarkGray

}

impl Colors {
    // emulate a color cycler
    pub fn value(&self) -> Color {
        match *self {
            Colors::Black => Color::new(0, 0, 0),
            Colors::White => Color::new(1, 1, 1),
            Colors::Red => Color::new(1, 0, 0),
            Colors::Green => Color::new(0, 1, 0),
            Colors::Blue => Color::new(0, 0, 1),
            // beautiful colors
            Colors::DarkRed => Color::new(175, 0, 0),
            Colors::NavyBlue => Color::new(0, 159, 255),
            Colors::DarkBlue => Color::new(51, 122, 183),
            Colors::SunYellow => Color::new(255, 175, 0),
            // Grays
            Colors::FaintGray => Color::new(238,238,238),
            Colors::LightGray => Color::new(221,221,221),
            Colors::MediumGray => Color::new(204,204,204),
            Colors::HeavyGray => Color::new(187,187,187),
            Colors::DarkGray => Color::new(154,154,154),
            // Space
            Colors::SpaceFaintGray => Color::new(192,197,206),
            Colors::SpaceLightGray => Color::new(167,173,186),
            Colors::SpaceMediumGray => Color::new(101,115,126),
            Colors::SpaceHeavyGray => Color::new(79,91,102),
            Colors::SpaceDarkGray => Color::new(52,61,70),
            
        }
    }

    pub fn from_int(i: isize) -> Color {
        match i % 4 {
            0 => Colors::SpaceFaintGray.value(),
            1 => Colors::SpaceLightGray.value(),
            2 => Colors::SpaceMediumGray.value(),
            3 => Colors::SpaceDarkGray.value(),
            _ => Colors::Black.value(),
        }
    }
}

pub fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}
