use std::option::Option;

use rayon;
use itertools::Itertools;

extern crate image;
extern crate ndarray;
use ndarray::prelude::*;

mod newton;
mod viz;
use newton::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    tol: f64,
    cluster: Vec<Complex64>,
    count: Vec<usize>,
}

impl Cluster {
    fn new() -> Self {
        Self {
            tol: 0.01,
            cluster: vec![],
            count: vec![],
        }
    }

    // Update count and mean if z is close to a previous cluster. Else make a new cluster and add z.
    fn push(&mut self, z: Complex64) -> usize {
        let mut i = 0;
        for (cls, count) in self.cluster.iter_mut().zip(self.count.iter_mut()) {
            if (z - *cls).norm() < self.tol {
                *cls = (*cls * (*count as f64) + z) / (*count + 1) as f64;
                *count += 1;
                return i;
            }
            i += 1
        }
        self.cluster.push(z);
        self.count.push(1);
        return i;
    }

    // Utility Function to encode possible None values
    fn push_maybe(&mut self, z: Option<Complex64>) -> isize {
        match z {
            Some(z) => self.push(z) as isize,
            None => -1,
        }
    }
}

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
    let f = |x: Complex64| -> Complex64 { x.powu(3) + 1. };
    let df = |x: Complex64| -> Complex64 { 3. * x.powu(2) };

    // Type shorthand for remembering index and pixel color
    type Pixel = (usize, usize, Option<Complex64>);
    
    // Wrap newton in a closure
    let newton_from_index = move |i: usize, j: usize| -> Pixel {
        (i, j, newton(f, df, Complex64::from_index([i, j], min_z, max_z, s.0, s.1)))
    };

    // Thread pool to find our roots
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build()
        .unwrap();
    
    let (tx, rx) = std::sync::mpsc::channel();
    for (i, j) in (0..s.0).cartesian_product(0..s.1) {
        let tx = tx.clone();
        pool.spawn_fifo(move || {
            tx.send(
                newton_from_index(i, j)
            ).unwrap()
        })    
    }
    drop(tx);

    // Collect results from threads
    let results: Vec<Pixel> = rx.into_iter().collect();

    for 
    (i, j, res) in results.into_iter() {
        // Wich root did we converge to?
        let root = roots.push_maybe(res);
        let col = viz::Colors::from_int(root);

        // write color
        img_arr[[i, j, 0]] = col.r;
        img_arr[[i, j, 1]] = col.g;
        img_arr[[i, j, 2]] = col.b;
    }

    let img = viz::array_to_image(img_arr);
    img.save("plots/out.png").unwrap();
    println!("done")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cluster() {
        let mut cluster = Cluster::new();

        let numbers = vec![
            Complex64::new(0., 0.),
            Complex64::new(0., 0.001),
            Complex64::new(0.0001, 0.0001),
            Complex64::new(1., 0.),
            Complex64::new(1.0001, 0.),
        ];

        let mut cls: Vec<usize> = vec![];

        for num in numbers.iter() {
            let idx = cluster.push(*num);
            cls.push(idx);
        }

        assert_eq!(cls, vec![0, 0, 0, 1, 1]);
        println!("{:?}", cls);
        println!("{:?}", cluster);
    }

    #[test]
    fn test_rem() {
        // I didn't know precisely how % works in rust
        println!("{}", -5 % 3);
        println!("{}", -2 % 3);
        println!("{}", -5 % 5);
        println!("{}", -5 % -3);
        println!("{}", -5 % -5);
        println!("{}", 5 % -3);
    }
}
