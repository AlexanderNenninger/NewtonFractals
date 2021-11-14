extern crate image;
extern crate ndarray;

use ndarray::prelude::*;

mod viz;
use viz::{array_to_image, quadrant};
mod newton;
use newton::*;

fn main() {
    let s = (9999, 9999, 3);
    let min_z = Complex64::new(-1., -1.);
    let max_z = Complex64::new(1., 1.);

    let mut out = Array3::<u8>::zeros(s);

    let f = |x: Complex64| -> Complex64 { x * x };
    let df = |x: Complex64| -> Complex64 { 2. * x };

    for i in 0..s.0 {
        for j in 0..s.1 {
            let z0 = Complex64::from_index([i, j], min_z, max_z, s.0, s.1);
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
