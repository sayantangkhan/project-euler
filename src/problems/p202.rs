use crate::utils::gcd::fast_gcd;
use rayon::prelude::*;

pub fn problem202(num_reflections: usize) -> usize {
    compute_num_trajectories(num_reflections)
}

fn compute_num_trajectories(num_reflections: usize) -> usize {
    let triangles_traversed = (num_reflections + 3) / 2;
    let target_modulus = 3 - (triangles_traversed % 3);

    let iterations = triangles_traversed / 3;

    (0..iterations)
        .into_par_iter()
        .filter(|x| fast_gcd(triangles_traversed, *x * 3 + target_modulus) == 1)
        .count()
}
