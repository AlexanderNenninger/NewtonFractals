use image::RgbImage;
use ndarray::Array3;
use num::Complex;
use num_traits::identities::Zero;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
            (true, true) => Color {
                r: 0,
                g: 159,
                b: 255,
            },
            (false, true) => Color {
                r: 255,
                g: 175,
                b: 0,
            },
            (true, false) => Color { r: 175, g: 0, b: 0 },
            (false, false) => Color {
                r: 51,
                g: 122,
                b: 183,
            },
        }
    } else {
        Color { r: 0, g: 0, b: 0 }
    }
}
