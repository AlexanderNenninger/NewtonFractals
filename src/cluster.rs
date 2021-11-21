use super::Complex64;

#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    tol: f64,
    cluster: Vec<Complex64>,
    count: Vec<usize>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            tol: 0.01,
            cluster: vec![],
            count: vec![],
        }
    }

    // Update count and mean if z is close to a previous cluster. Else make a new cluster and add z.
    pub fn push(&mut self, z: Complex64) -> usize {
        let mut i = 0;
        for (cls, count) in self.cluster.iter_mut().zip(self.count.iter_mut()) {
            if (z - *cls).norm_sqr() < self.tol {
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
    pub fn push_maybe(&mut self, z: Option<Complex64>) -> isize {
        match z {
            Some(z) => self.push(z) as isize,
            None => -1,
        }
    }
}

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
