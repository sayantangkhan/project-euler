use std::collections::HashSet;

type Point = (bool, bool, bool, bool, bool, bool);

pub fn problem209() -> usize {
    let orbits = orbit_decomposition();

    orbits
        .into_iter()
        .map(|o| count_assignments_fast(o.len()))
        .product()
}

fn count_assignments_fast(orbit_size: usize) -> usize {
    // The assignments satisfy the usual Fibonacci relations, but with
    // initial conditions == (1,3).
    if (orbit_size == 0) | (orbit_size == 1) {
        return 1;
    }

    let (mut a, mut b) = (1, 3);

    for _ in 0..(orbit_size - 2) {
        (a, b) = (b, a + b);
    }

    b
}

fn group_action(point: Point) -> Point {
    let (a, b, c, d, e, f) = point;
    (b, c, d, e, f, a ^ (b & c))
}

fn compute_orbit(point: Point) -> Vec<Point> {
    let mut orbit = Vec::new();
    orbit.push(point);
    let mut next_point = group_action(point);
    while next_point != point {
        orbit.push(next_point);
        next_point = group_action(next_point);
    }
    orbit
}

fn orbit_decomposition() -> Vec<Vec<Point>> {
    let mut seen = HashSet::new();
    let mut orbits = Vec::new();
    for x in 0..64 {
        let a = (x & (1 << 0)) != 0;
        let b = (x & (1 << 1)) != 0;
        let c = (x & (1 << 2)) != 0;
        let d = (x & (1 << 3)) != 0;
        let e = (x & (1 << 4)) != 0;
        let f = (x & (1 << 5)) != 0;
        let point = (a, b, c, d, e, f);
        if !seen.contains(&point) {
            let orbit = compute_orbit(point);
            for p in &orbit {
                seen.insert(p.clone());
            }
            orbits.push(orbit);
        }
    }
    orbits
}

fn _count_assignments_bootstrap(orbit_size: usize) -> usize {
    // This turns out to be F(n) + F(n-2), where F is the Fibonacci function, and n is the
    // orbit_size
    if (orbit_size == 0) | (orbit_size == 1) {
        return 1;
    } else {
        _count_assignments(orbit_size - 1, true) + _count_assignments(orbit_size - 2, false)
    }
}

fn _count_assignments(orbit_size: usize, starts_with_zero: bool) -> usize {
    if orbit_size == 0 {
        return 1;
    } else if orbit_size == 1 {
        match starts_with_zero {
            true => {
                return 2;
            }
            false => {
                return 1;
            }
        }
    } else {
        _count_assignments(orbit_size - 1, starts_with_zero)
            + _count_assignments(orbit_size - 2, starts_with_zero)
    }
}
