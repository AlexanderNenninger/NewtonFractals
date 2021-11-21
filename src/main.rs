extern crate image;
extern crate ndarray;

use ndarray::prelude::*;

mod newton;
use newton::*;

mod cluster;
use cluster::*;

mod viz;



fn main() {
    // Output size
    let s = (10000, 10000, 3);

    // Plot Limits
    let min_z = Complex64::new(-1., -1.);
    let max_z = Complex64::new(1., 1.);

    // loop setup
    // Stash of roots
    let mut roots = Cluster::new();
    // output array
    let mut img_arr = Array3::<u8>::zeros(s);

    // We search for roots of this function
    let f = |x: Complex64| -> Complex64 { x.powu(4) + 1. };
    let df = |x: Complex64| -> Complex64 { 4. * x.powu(3) };

    // Assign a color to each pixel in img_arr
    for i in 0..s.0 {
        for j in 0..s.1 {
            // starting pos
            let z0 = Complex64::from_index([i, j], min_z, max_z, s.0, s.1);

            //root
            let res = newton(f, df, z0);

            // Wihch root did we converge to?
            let root = roots.push_maybe(res);
            let col = viz::Colors::from_int(root);

            // write color
            img_arr[[i, j, 0]] = col.r;
            img_arr[[i, j, 1]] = col.g;
            img_arr[[i, j, 2]] = col.b;
        }
    }

    let img = viz::array_to_image(img_arr);
    img.save("plots/out.png").unwrap();
    println!("done")
}
