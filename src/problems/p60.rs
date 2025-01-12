use std::collections::HashSet;

use crate::ProgramError;

use super::super::{check_primality, create_sieve};
use anyhow::{Context, Result};
use itertools::{self, Itertools};

pub fn problem60(set_size: usize) -> Result<usize> {
    let prime_bound = 10000;

    let prime_sieve = create_sieve(prime_bound);
    let prime_sieve_hashset = HashSet::from_iter(prime_sieve.iter().cloned());
    let largest_prime = prime_sieve.iter().cloned().max().unwrap();

    let prime_pair_set =
        prime_pair_sets(set_size, &prime_sieve, &prime_sieve_hashset, largest_prime)?;
    Ok(prime_pair_set.into_iter().sum())
}

fn prime_pair_sets(
    set_size: usize,
    prime_sieve: &[usize],
    prime_sieve_hashset: &HashSet<usize>,
    largest_prime: usize,
) -> Result<Vec<usize>> {
    let combinations = prime_sieve.iter().cloned().combinations(set_size);

    let mut min_sum_candidate: Option<Vec<usize>> = None;

    for candidate in combinations {
        // dbg!(&candidate);
        let candidate_size: usize = candidate.iter().sum();
        if min_sum_candidate.is_some()
            && (candidate_size >= min_sum_candidate.as_ref().unwrap().iter().sum())
        {
            break;
        }

        let candidate_is_valid = check_concats(&candidate, prime_sieve_hashset, largest_prime)?;
        if candidate_is_valid {
            min_sum_candidate = Some(candidate);
        }
    }

    match min_sum_candidate {
        Some(c) => Ok(c),
        None => Err(ProgramError::InitialConditionTooSmall.into()),
    }
}

fn check_concats(
    candidate: &[usize],
    prime_sieve_hashset: &HashSet<usize>,
    largest_prime: usize,
) -> Result<bool> {
    let permutations = candidate.iter().permutations(2);

    for concatenataed_res in permutations.map(|pair| concat_num(*pair[0], *pair[1])) {
        let res = concatenataed_res?;
        if !lazy_primality_check(res, prime_sieve_hashset, largest_prime) {
            return Ok(false);
        }
    }
    Ok(true)
}

// Check using prime_sieve_first, and then primality test
fn lazy_primality_check(
    num: usize,
    prime_sieve_hashset: &HashSet<usize>,
    largest_prime: usize,
) -> bool {
    if num <= largest_prime {
        prime_sieve_hashset.contains(&num)
    } else {
        check_primality(num)
    }
}

fn concat_num(a: usize, b: usize) -> Result<usize> {
    (a.to_string() + &b.to_string())
        .parse()
        .context("Concatenated two usize's to get something larger.")
}

#[test]
fn test_check_concats() {
    let candidate = &[3, 7, 109];

    let prime_bound = 120;

    let prime_sieve = create_sieve(prime_bound);
    let prime_sieve_hashset = HashSet::from_iter(prime_sieve.iter().cloned());
    let largest_prime = prime_sieve.iter().cloned().max().unwrap();

    assert_eq!(
        check_concats(candidate, &prime_sieve_hashset, largest_prime).unwrap(),
        true
    );
}

#[test]
fn test_lazy_primality() {
    let primes = &[37, 73, 3109, 1093, 7109, 1097];

    let prime_bound = 120;

    let prime_sieve = create_sieve(prime_bound);
    let prime_sieve_hashset = HashSet::from_iter(prime_sieve.iter().cloned());
    let largest_prime = prime_sieve.iter().cloned().max().unwrap();

    for prime in primes {
        assert_eq!(
            lazy_primality_check(*prime, &prime_sieve_hashset, largest_prime),
            true
        );
    }
}
