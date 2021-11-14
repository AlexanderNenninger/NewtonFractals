use num::Complex;

const MAX_IT: usize = 1000;
const TOL: f64 = 0.0000001;

pub type Complex64 = Complex<f64>;

pub type Complex64Fun = fn(Complex64) -> Complex64;

pub fn newton(f: Complex64Fun, df: Complex64Fun, z0: Complex64) -> Option<Complex64> {
    // https://fse.studenttheses.ub.rug.nl/14180/1/Alida_Wiersma_2016_WB.pdf
    let mut z = z0;
    let mut z_new: Complex64;
    let mut update: Complex64;

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
}
