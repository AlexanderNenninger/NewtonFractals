use num::Complex;
use num::Float;

const MAX_IT: usize = 1000;

/// Newton's method in the complex plane
/// https://fse.studenttheses.ub.rug.nl/14180/1/Alida_Wiersma_2016_WB.pdf
pub fn newton<T>(
    f: fn(Complex<T>) -> Complex<T>,
    df: fn(Complex<T>) -> Complex<T>,
    z0: Complex<T>,
) -> Option<Complex<T>>
where
    T: Float,
{
    let mut z = z0;
    let mut z_new: Complex<T>;
    let mut update: Complex<T>;

    for _i in 0..MAX_IT {
        z_new = z - f(z) / df(z);
        update = z_new - z;
        z = z_new;

        if update.norm_sqr() < T::epsilon() {
            return Some(z);
        }
    }
    None
}

pub trait AsIndex {
    /// Convert any 2-dimensional object to an image index
    fn as_index(&self, min: Self, max: Self, width: usize, height: usize) -> [usize; 2];

    // Make any 2-dimensional object from an image index
    fn from_index(idx: [usize; 2], min: Self, max: Self, width: usize, height: usize) -> Self;
}

pub type Complex64 = Complex<f64>;
impl AsIndex for Complex64 {
    /// given a complex number z, find the array index closest to where z would be in the complex plane
    fn as_index(&self, min: Self, max: Self, width: usize, height: usize) -> [usize; 2] {
        let o = self - min;
        let r = max - min;

        let w = width as f64;
        let h = height as f64;
        [(o.re / r.re * w) as usize, (o.im / r.im * h) as usize]
    }

    /// given an array index, return the corresponding Complex64
    fn from_index(idx: [usize; 2], min: Self, max: Self, width: usize, height: usize) -> Self {
        let i = (idx[0] as f64) / (width as f64);
        let j = (idx[1] as f64) / (height as f64);

        let re: f64 = i as f64 * max.re + (1. - i) as f64 * min.re;
        let im: f64 = j as f64 * max.im + (1. - j) as f64 * min.im;
        return Complex64::new(re, im);
    }
}

#[cfg(test)]
mod tests {
    const TEST_TOL: f64 = 0.0001;
    use super::*;
    #[test]
    fn test_newton() {
        // f(x) = x²
        let f = |x: Complex64| -> Complex64 { x * x };
        let df = |x: Complex64| -> Complex64 { 2. * x };
        let z0 = Complex64::new(0.2, 0.2);

        let res = newton(f, df, z0).expect("Newton for z² = 0 failed");
        assert!(res.norm() < TEST_TOL, "x²=0 failed");

        // f(x) = x³ + 1
        let f = |x: Complex64| -> Complex64 { x.powu(3) + Complex64::from(1.) };
        let df = |x: Complex64| -> Complex64 { 3. * x.powu(2) };

        let solutions: (Complex64, Complex64, Complex64) = (
            Complex64::new(-1., 0.),
            Complex64::new(0.5, 0.866025403784439),
            Complex64::new(0.5, -0.866025403784439),
        );

        // Real line
        let z0 = Complex64::new(-0.2, 0.);

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
        let z0 = Complex64::new(0.5, 0.8);

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
        let z0 = Complex64::new(0.5, -0.8);

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
        let min_z = Complex64::new(-2., -2.);
        let max_z = Complex64::new(2., 2.);

        let z = Complex64::from(0.);
        let idx = z.as_index(min_z, max_z, s.0, s.1);
        assert_eq!(idx, [500, 500], "Index: {:?}", idx)
    }

    #[test]
    fn test_from_index() {
        let s = (1000, 1000);
        let min_z = Complex64::new(-2., -2.);
        let max_z = Complex64::new(2., 2.);
        let idx = [500, 500];

        let z = Complex64::from_index(idx, min_z, max_z, s.0, s.1);
        let e = (z - Complex64::from(0.)).norm();

        assert!(e < TEST_TOL, "z= {}", z)
    }
}
