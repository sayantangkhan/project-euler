use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PossibleSum {
    Zero,
    One,
    Many,
}

pub fn problem201(sorted_numset: &[usize], subset_size: usize) -> Result<usize> {
    let max_candidate: usize = sorted_numset[..subset_size].iter().sum();
    let min_candidate: usize = sorted_numset[(sorted_numset.len() - subset_size)..]
        .iter()
        .sum();
    let mut cache = HashMap::new();

    // // Doing some dynamic programming instead of recursing
    // for target_sum in 0..(max_candidate + 1) {
    //     for right_index in 0..sorted_numset.len() {
    //         let left_index = sorted_numset.len() - right_index - 1;
    //         for k in 0..(subset_size + 1) {
    //             // dbg!((target_sum, k, &sorted_numset[left_index..].len()));
    //             count_sum_memoized(
    //                 target_sum as isize,
    //                 k,
    //                 &sorted_numset[left_index..],
    //                 &mut cache,
    //             );
    //         }
    //     }
    // }

    let res = ((min_candidate)..(max_candidate + 1))
        .filter(|s| {
            count_sum_memoized(*s as isize, subset_size, sorted_numset, &mut cache)
                == PossibleSum::One
        })
        .sum();

    // dbg!(cache.len());

    Ok(res)

    // Ok(((min_candidate)..(max_candidate + 1))
    //     .filter(|s| {
    //         count_sum_memoized(*s as isize, subset_size, sorted_numset, &mut cache)
    //             == PossibleSum::One
    //     })
    //     .sum())
}

fn count_sum_memoized<'a>(
    target_sum: isize,
    subset_size: usize,
    sorted_numset: &'a [usize],
    cache: &mut HashMap<(isize, usize, &'a [usize]), PossibleSum>,
) -> PossibleSum {
    // TODO: Figure out why this isn't working
    // dbg!(target_sum, subset_size, sorted_numset);
    // Figure out a way to speed this up
    if let Some(cached_result) = cache.get(&(target_sum, subset_size, sorted_numset)) {
        // dbg!(cache.len());
        *cached_result
    } else {
        count_sum(target_sum, subset_size, sorted_numset, cache)
    }
}

fn count_sum<'a>(
    target_sum: isize,
    subset_size: usize,
    sorted_numset: &'a [usize],
    cache: &mut HashMap<(isize, usize, &'a [usize]), PossibleSum>,
) -> PossibleSum {
    // dbg!(target_sum, subset_size, sorted_numset);
    // Dealing with some base cases
    if subset_size == 0 {
        if target_sum == 0 {
            // Storing in the cache might not be necessary
            // cache.insert((target_sum, subset_size, sorted_numset), PossibleSum::One);
            return PossibleSum::One;
        } else {
            // cache.insert((target_sum, subset_size, sorted_numset), PossibleSum::Zero);
            return PossibleSum::Zero;
        }
    }
    if target_sum < 0 {
        // cache.insert((target_sum, subset_size, sorted_numset), PossibleSum::Zero);
        return PossibleSum::Zero;
    }
    if subset_size > sorted_numset.len() {
        // cache.insert((target_sum, subset_size, sorted_numset), PossibleSum::Zero);
        return PossibleSum::Zero;
    }

    // Now the recursion part
    // First we see if there exists a unique sum decomposition with first elem of sorted_numset in sum
    // let exists_uniquely_c1 = count_sum_memoized(
    //     target_sum - (sorted_numset[0] as isize),
    //     subset_size - 1,
    //     &sorted_numset[1..],
    //     cache,
    // );

    // let exists_uniquely_c1 =
    //     count_sum_memoized(target_sum, subset_size, &sorted_numset[1..], cache);
    let exists_uniquely_c2 = count_sum_memoized(
        target_sum - (sorted_numset[0] as isize),
        subset_size - 1,
        &sorted_numset[1..],
        cache,
    );

    // Now we check how many possible sums we get in the left branch of search tree
    // If we get two, we don't need to check the second branch
    let res = match exists_uniquely_c2 {
        PossibleSum::Many => PossibleSum::Many,
        PossibleSum::One => {
            let exists_uniquely_c1 =
                count_sum_memoized(target_sum, subset_size, &sorted_numset[1..], cache);
            // let exists_unique_c2 = count_sum_memoized(
            //     target_sum - (sorted_numset[0] as isize),
            //     subset_size - 1,
            //     &sorted_numset[1..],
            //     cache,
            // );
            match exists_uniquely_c1 {
                PossibleSum::Zero => PossibleSum::One,
                _ => PossibleSum::Many,
            }
        }
        PossibleSum::Zero => {
            let exists_uniquely_c1 =
                count_sum_memoized(target_sum, subset_size, &sorted_numset[1..], cache);
            exists_uniquely_c1
        }
    };

    cache.insert((target_sum, subset_size, sorted_numset), res);
    res
}

#[test]
fn test_check_sum_exists_uniquely_easy() {
    let sorted_numset: Vec<usize> = [1, 3, 6, 8, 10, 11].into_iter().rev().collect();

    let unique_sums = &[10, 12, 14, 18, 21, 25, 27, 29];
    let non_unique_sums = &[15, 17, 19, 20, 22, 24];
    let zero_sums = &[11, 13, 16, 23, 26];
    let subset_size = 3;
    let mut cache = HashMap::new();

    dbg!(unique_sums.into_iter().sum::<isize>());

    for target_sum in unique_sums {
        assert_eq!(
            count_sum(*target_sum, subset_size, &sorted_numset, &mut cache),
            PossibleSum::One
        );
    }

    for target_sum in non_unique_sums {
        assert_eq!(
            count_sum(*target_sum, subset_size, &sorted_numset, &mut cache),
            PossibleSum::Many
        );
    }

    for target_sum in zero_sums {
        assert_eq!(
            count_sum(*target_sum, subset_size, &sorted_numset, &mut cache),
            PossibleSum::Zero
        );
    }
}

#[test]
fn test_check_sum_exists_uniquely_medium() {
    let sorted_numset: Vec<usize> = (1..31).map(|x| x * x).rev().collect();

    let subset_size = 5;

    let result = problem201(&sorted_numset, subset_size).unwrap();

    assert_eq!(result, 642963);
}
