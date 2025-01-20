use std::cmp::min;
use std::mem::swap;

pub(crate) fn fast_gcd(mut u: usize, mut v: usize) -> usize {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}

pub(crate) fn _euclidean_gcd(mut n: usize, mut m: usize) -> usize {
    if m >= n {
        (m, n) = (n, m);
    }

    if m == 0 {
        return n;
    }

    // We are guaranteed that m is smaller
    while (n % m) != 0 {
        (n, m) = (m, n % m);
    }
    m
}

#[test]
fn test_euclidean_gcd() {
    use std::iter::zip;
    let pairs = &[(2, 1), (4, 2), (6, 9)];
    let gcds = &[1, 2, 3];

    for (p, g) in zip(pairs.into_iter(), gcds.into_iter()) {
        assert_eq!(_euclidean_gcd(p.0, p.1), *g);
    }
}

#[test]
fn test_binary_shift_gcd() {
    use std::iter::zip;
    let pairs = &[(2, 1), (4, 2), (6, 9)];
    let gcds = &[1, 2, 3];

    for (p, g) in zip(pairs.into_iter(), gcds.into_iter()) {
        assert_eq!(fast_gcd(p.0, p.1), *g);
    }
}
