pub(crate) fn check_primality(num: usize) -> bool {
    let factor_bound: usize = ((num as f64).sqrt() as usize) + 1;
    for possible_factor in 2..factor_bound {
        if num % possible_factor == 0 {
            return false;
        }
    }
    true
}

pub(crate) fn create_sieve(upper_bound: usize) -> Vec<usize> {
    let mut nums = vec![true; upper_bound + 1]; // The thing we will sieve

    let mut current_prime = 1; // This is just so that we don't have the 2 case outside the loop
    while let Some(next_prime) = find_next_prime_in_sieved(current_prime, &nums) {
        let to_remove = (2..(upper_bound / next_prime + 1))
            .map(|m| next_prime * m)
            .filter(|n| *n <= upper_bound);

        for num in to_remove {
            nums[num] = false;
        }
        current_prime = next_prime;
    }

    // Compute the indices of the true entries and return those
    nums[2..]
        .iter()
        .enumerate()
        .filter(|(_o, b)| **b)
        .map(|(o, _)| o + 2)
        .collect()
}

fn find_next_prime_in_sieved(current_prime: usize, partial_sieved: &[bool]) -> Option<usize> {
    let to_check = &partial_sieved[current_prime + 1..];

    let first_true = to_check.iter().enumerate().find(|(_o, b)| **b);
    first_true.map(|(o, _)| o + 1 + current_prime)
}

#[test]
fn test_create_sieve() {
    let primes_upto_20 = vec![2, 3, 5, 7, 11, 13, 17, 19];
    assert_eq!(create_sieve(20), primes_upto_20);
}

#[test]
fn test_primality() {
    let primes = &[37, 73, 3109, 1093, 7109, 1097];

    for prime in primes {
        assert_eq!(check_primality(*prime), true);
    }
}
