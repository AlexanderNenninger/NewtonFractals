use std::f64;

extern crate image;
extern crate ndarray;

use ndarray::prelude::*;
use num::complex::Complex;

mod viz;
use viz::{array_to_image, quadrant};
mod newton;
use newton::newton;

trait AsIndex {
    // Convert any 2-dimensional object to an image index
    fn as_index(&self, min: Self, max: Self, width: usize, height: usize) -> [usize; 2];

    // Make any 2-dimensional object from an image index
    fn from_index(idx: [usize; 2], min: Self, max: Self, width: usize, height: usize) -> Self;
}

type C = Complex<f64>;

impl AsIndex for C {
    fn as_index(&self, min: Self, max: Self, width: usize, height: usize) -> [usize; 2] {
        let o = self - min;
        let r = max - min;

        let w = width as f64;
        let h = height as f64;
        [(o.re / r.re * w) as usize, (o.im / r.im * h) as usize]
    }

    fn from_index(idx: [usize; 2], min: Self, max: Self, width: usize, height: usize) -> Self {
        let i = (idx[0] as f64) / (width as f64);
        let j = (idx[1] as f64) / (height as f64);

        let re: f64 = i as f64 * max.re + (1. - i) as f64 * min.re;
        let im: f64 = j as f64 * max.im + (1. - j) as f64 * min.im;
        return C::new(re, im);
    }
}

fn f(z: C) -> C {
    z.powu(3) + C::from(1.)
}

fn df(z: C) -> C {
    3. * z.powu(2)
}

fn main() {
    let s = (9999, 9999, 3);
    let min_z = C::new(-1., -1.);
    let max_z = C::new(1., 1.);

    let mut out = Array3::<u8>::zeros(s);

    for i in 0..s.0 {
        for j in 0..s.1 {
            let z0 = C::from_index([i, j], min_z, max_z, s.0, s.1);
            let res = newton(f, df, z0);
            let q = quadrant(res);

            out[[i, j, 0]] = q.r;
            out[[i, j, 1]] = q.g;
            out[[i, j, 2]] = q.b;
        }
    }

    let im = array_to_image(out);
    im.save("plots/out.png").unwrap();
    println!("done")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tolerance
    const TEST_TOL: f64 = 0.0001;

    #[test]
    fn test_as_index() {
        let s = (1000, 1000);
        let min_z = C::new(-2., -2.);
        let max_z = C::new(2., 2.);

        let z = C::from(0.);
        let idx = z.as_index(min_z, max_z, s.0, s.1);
        assert_eq!(idx, [500, 500], "Index: {:?}", idx)
    }

    #[test]
    fn test_from_index() {
        let s = (1000, 1000);
        let min_z = C::new(-2., -2.);
        let max_z = C::new(2., 2.);
        let idx = [500, 500];

        let z = C::from_index(idx, min_z, max_z, s.0, s.1);
        let e = (z - C::from(0.)).norm();

        assert!(e < TEST_TOL, "z= {}", z)
    }
}
