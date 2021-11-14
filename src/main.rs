#![allow(dead_code)]

extern crate image;
extern crate ndarray;

use ndarray::prelude::*;

mod viz;
#[allow(unused)]
use viz::array_to_image;
mod newton;
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

    // Update count and mean for z is close to a previous cluster or make a new cluster.
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

    fn push_maybe(&mut self, z: Option<Complex64>) -> isize {
        match z {
            Some(z) => self.push(z) as isize,
            None => -1,
        }
    }
}

fn main() {
    let s = (10000, 10000, 3);
    let min_z = Complex64::new(-1., -1.);
    let max_z = Complex64::new(1., 1.);

    let mut roots = Cluster::new();
    let mut img_arr = Array3::<u8>::zeros(s);

    let f = |x: Complex64| -> Complex64 { x.powu(3) + 1. };
    let df = |x: Complex64| -> Complex64 { 3. * x.powu(2) };

    for i in 0..s.0 {
        for j in 0..s.1 {
            let z0 = Complex64::from_index([i, j], min_z, max_z, s.0, s.1);
            let res = newton(f, df, z0);
            let root = roots.push_maybe(res);
            let col = viz::Colors::from_int(root);          

            img_arr[[i, j, 0]] = col.r;
            img_arr[[i, j, 1]] = col.g;
            img_arr[[i, j, 2]] = col.b;
        }
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
    }

    #[test]
    fn test_rem() {
        println!("{}", -5 % 3);
        println!("{}", -2 % 3);
        println!("{}", -5 % 5);
        println!("{}", -5 % -3);
        println!("{}", -5 % -5);
        println!("{}", 5 % -3);
    }
}
