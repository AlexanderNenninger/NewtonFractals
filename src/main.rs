use std::f64;

extern crate image;
extern crate ndarray;

use ndarray::prelude::*;
use num::complex::Complex;

mod viz;
use viz::{array_to_image, quadrant};

const MAX_IT: usize = 1000;
const TOL: f64 = 0.0000001;

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

fn newton(f: fn(C) -> C, df: fn(C) -> C, z0: C) -> Option<C> {
    // https://fse.studenttheses.ub.rug.nl/14180/1/Alida_Wiersma_2016_WB.pdf
    let mut z = z0;
    let mut z_new: C;
    let mut update: C;

    for _i in 0..MAX_IT {
        z_new = z - f(z) / df(z);
        update = z_new - z;
        z = z_new;

        if update.norm() < TOL {
            return Some(z);
        }
    }
    None
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

    // Tolerance for Newton
    const TEST_TOL: f64 = 0.0001;

    #[test]
    fn test_newton() {
        // f(x) = x²
        let f = |x: C| -> C { x * x };
        let df = |x: C| -> C { 2. * x };
        let z0 = C::new(0.2, 0.2);

        let res = newton(f, df, z0).expect("Newton for z² = 0 failed");
        assert!(res.norm() < TEST_TOL, "x²=0 failed");

        // f(x) = x³ + 1
        let f = |x: C| -> C { x.powu(3) + C::from(1.) };
        let df = |x: C| -> C { 3. * x.powu(2) };

        let solutions: (C, C, C) = (
            C::new(-1., 0.),
            C::new(0.5, 0.866025403784439),
            C::new(0.5, -0.866025403784439),
        );

        // Real line
        let z0 = C::new(-0.2, 0.);

        let res = newton(f, df, z0).expect("Newton for z³ + 1 = 0 failed");
        assert!(
            res.norm() - 1. < TEST_TOL,
            "Solution to x³+1=0 not on the unit circle. {1:.4} -> {0:.4}",
            res,
            z0
        );
        assert!(
            (res - solutions.0).norm() < TEST_TOL,
            "Solution to x³ + 1 = 0 not found. Err = {}",
            (res - solutions.0).norm()
        );

        // 1st imaginary solution
        let z0 = C::new(0.5, 0.8);

        let res = newton(f, df, z0).expect("Newton for z³ + 1 = 0 failed");
        assert!(
            res.norm() - 1. < TEST_TOL,
            "Solution to x³+1=0 not on the unit circle. {1:.4} -> {0:.4}",
            res,
            z0
        );
        assert!(
            (res - solutions.1).norm() < TEST_TOL,
            "Solution to x³ + 1=0 not found. Err = {}",
            (res - solutions.1).norm()
        );

        // 2nd imaginary solution
        let z0 = C::new(0.5, -0.8);

        let res = newton(f, df, z0).expect("Newton for z³ + 1 = 0 failed");
        assert!(
            res.norm() - 1. < TEST_TOL,
            "Solution to x³+1=0 not on the unit circle. {1:.4} -> {0:.4}",
            res,
            z0
        );
        assert!(
            (res - solutions.2).norm() < TEST_TOL,
            "Solution to x³ + 1=0 not found. Err = {}",
            (res - solutions.2).norm()
        );
    }

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
