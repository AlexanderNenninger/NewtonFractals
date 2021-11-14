#![allow(dead_code)]

use image::RgbImage;
use ndarray::Array3;
use num::Complex;
use num_traits::identities::Zero;

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
pub enum Colors {
    Black,
    White,
    Red,
    Green,
    Blue,
    DarkRed,
    NavyBlue,
    DarkBlue,
    SunYellow,
}

impl Colors {
    pub fn value(&self) -> Color {
        match *self {
            Colors::Black => Color::new(0, 0, 0),
            Colors::White => Color::new(1, 1, 1),
            Colors::Red => Color::new(1, 0, 0),
            Colors::Green => Color::new(0, 1, 0),
            Colors::Blue => Color::new(0, 0, 1),
            // Beautyful Colors
            Colors::DarkRed => Color::new(175, 0, 0),
            Colors::NavyBlue => Color::new(0, 159, 255),
            Colors::DarkBlue => Color::new(51, 122, 183),
            Colors::SunYellow => Color::new(0, 80, 255),
        }
    }

    pub fn from_int(i: isize) -> Color {
        match i % 4 {
            0 => Colors::NavyBlue.value(),
            1 => Colors::NavyBlue.value(),
            2 => Colors::NavyBlue.value(),
            3 => Colors::NavyBlue.value(),
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

pub fn quadrant<T: Zero + PartialOrd>(z: Option<Complex<T>>) -> Color {
    // Get Quadrant of any complex number

    if z.is_some() {
        let z = z.unwrap();
        match (z.re >= T::zero(), z.im >= T::zero()) {
            (true, true) => Colors::NavyBlue.value(),
            (false, true) => Colors::SunYellow.value(),
            (true, false) => Colors::DarkRed.value(),
            (false, false) => Colors::DarkBlue.value(),
        }
    } else {
        Colors::Black.value()
    }
}
