use rayon::prelude::*;
use std::collections::HashMap;

pub fn problem208(path_length: usize) -> usize {
    let initial_cotangent_vector = CotangentVector::start();
    let mut count = HashMap::new();
    count.insert(initial_cotangent_vector, 1);

    for i in 0..path_length {
        count = compute_path_count_iteratively(&count, path_length - i);
    }
    *count.get(&initial_cotangent_vector).unwrap_or(&0)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct CotangentVector {
    position: (i8, i8, i8, i8),
    direction: u8, // Really should be an element of Z/5Z
}

// TODO: Figure out if branch prediction can optimize away the Option wrapper
impl CotangentVector {
    fn move_forward_left(&self) -> Option<Self> {
        match self {
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 0,
            } => Some(CotangentVector {
                position: (v1 - 1, v2 - 1, v3 - 1, v4 - 1),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 1,
            } => Some(CotangentVector {
                position: (*v1, *v2, *v3, v4 + 1),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 2,
            } => Some(CotangentVector {
                position: (*v1, *v2, v3 + 1, *v4),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 3,
            } => Some(CotangentVector {
                position: (*v1, v2 + 1, *v3, *v4),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 4,
            } => Some(CotangentVector {
                position: (v1 + 1, *v2, *v3, *v4),
                direction: (c.direction + 1) % 5,
            }),
            _ => None,
        }
    }

    fn move_forward_right(&self) -> Option<Self> {
        match self {
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 1,
            } => Some(CotangentVector {
                position: (v1 - 1, v2 - 1, v3 - 1, v4 - 1),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 2,
            } => Some(CotangentVector {
                position: (*v1, *v2, *v3, v4 + 1),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 3,
            } => Some(CotangentVector {
                position: (*v1, *v2, v3 + 1, *v4),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 4,
            } => Some(CotangentVector {
                position: (*v1, v2 + 1, *v3, *v4),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 0,
            } => Some(CotangentVector {
                position: (v1 + 1, *v2, *v3, *v4),
                direction: (c.direction + 4) % 5,
            }),
            _ => None,
        }
    }

    // Inverse of move_forward_left
    fn move_backward_left(&self) -> Option<Self> {
        match self {
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 1,
            } => Some(CotangentVector {
                position: (v1 + 1, v2 + 1, v3 + 1, v4 + 1),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 2,
            } => Some(CotangentVector {
                position: (*v1, *v2, *v3, v4 - 1),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 3,
            } => Some(CotangentVector {
                position: (*v1, *v2, v3 - 1, *v4),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 4,
            } => Some(CotangentVector {
                position: (*v1, v2 - 1, *v3, *v4),
                direction: (c.direction + 4) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 0,
            } => Some(CotangentVector {
                position: (v1 - 1, *v2, *v3, *v4),
                direction: (c.direction + 4) % 5,
            }),
            _ => None,
        }
    }

    fn move_backward_right(&self) -> Option<Self> {
        match self {
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 0,
            } => Some(CotangentVector {
                position: (v1 + 1, v2 + 1, v3 + 1, v4 + 1),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 1,
            } => Some(CotangentVector {
                position: (*v1, *v2, *v3, v4 - 1),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 2,
            } => Some(CotangentVector {
                position: (*v1, *v2, v3 - 1, *v4),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 3,
            } => Some(CotangentVector {
                position: (*v1, v2 - 1, *v3, *v4),
                direction: (c.direction + 1) % 5,
            }),
            c @ CotangentVector {
                position: (v1, v2, v3, v4),
                direction: 4,
            } => Some(CotangentVector {
                position: (v1 - 1, *v2, *v3, *v4),
                direction: (c.direction + 1) % 5,
            }),
            _ => None,
        }
    }

    fn start() -> Self {
        CotangentVector {
            position: (0, 0, 0, 0),
            direction: 0,
        }
    }
}

fn compute_path_count_iteratively(
    current_path_count: &HashMap<CotangentVector, usize>,
    steps_left: usize,
) -> HashMap<CotangentVector, usize> {
    let next_left = current_path_count
        .par_iter()
        .map(|(n, _)| n.move_forward_left().unwrap());
    let next_right = current_path_count
        .par_iter()
        .map(|(n, _)| n.move_forward_right().unwrap());

    let next = next_left.chain(next_right);

    let next_count = next
        .filter(|n| filter_node(n, steps_left))
        .map(|n| (n, path_sum(n, current_path_count)));
    next_count.collect()
}

fn path_sum(n: CotangentVector, current_path_count: &HashMap<CotangentVector, usize>) -> usize {
    let back_left = n.move_backward_left().unwrap(); // Guaranteed to succeed
    let back_right = n.move_backward_right().unwrap(); // Guaranteed to succeed
    let back_left_count = current_path_count.get(&back_left).unwrap_or(&0);
    let back_right_count = current_path_count.get(&back_right).unwrap_or(&0);
    *back_left_count + *back_right_count
}

fn filter_node(n: &CotangentVector, steps_left: usize) -> bool {
    // If the L\infty norm of n.basepoint is greater than steps left, we can ignore it
    let pos_abs = [
        n.position.0.abs(),
        n.position.1.abs(),
        n.position.2.abs(),
        n.position.3.abs(),
    ];
    (*pos_abs.iter().max().unwrap() as usize) <= steps_left
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for CotangentVector {
        fn arbitrary(g: &mut Gen) -> CotangentVector {
            CotangentVector {
                position: (
                    i8::arbitrary(g) % 100,
                    i8::arbitrary(g) % 100,
                    i8::arbitrary(g) % 100,
                    i8::arbitrary(g) % 100,
                ),
                direction: (u8::arbitrary(g) % 5),
            }
        }
    }

    quickcheck! {
    fn move_forward_and_backward_left_is_identity(cotangent_vector: CotangentVector) -> bool {
        cotangent_vector
            == cotangent_vector
                .move_forward_left()
                .unwrap()
                .move_backward_left()
                .unwrap()
    }
    }

    quickcheck! {
    fn move_forward_and_backward_right_is_identity(cotangent_vector: CotangentVector) -> bool {
        cotangent_vector
            == cotangent_vector
                .move_forward_right()
                .unwrap()
                .move_backward_right()
                .unwrap()
    }
    }

    quickcheck! {
    fn move_backward_and_forward_left_is_identity(cotangent_vector: CotangentVector) -> bool {
        cotangent_vector
            == cotangent_vector
                .move_backward_left()
                .unwrap()
                .move_forward_left()
                .unwrap()
    }
    }

    quickcheck! {
    fn move_backward_and_forward_right_is_identity(cotangent_vector: CotangentVector) -> bool {
        cotangent_vector
            == cotangent_vector
                .move_backward_right()
                .unwrap()
                .move_forward_right()
                .unwrap()
    }
    }
}
