use std::num::Wrapping;
use rand::{Rng, SeedableRng};

const N: usize = 10_000_000;

pub fn main() {
    println!("Simple branching example with N={N}");

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

    // Create the vectors and fill them with random numbers and boolean values
    let mut v1: Vec<u32> = vec![0; N];
    let mut v2: Vec<u32> = vec![0; N];
    let mut bools: Vec<bool> = vec![false; N];

    for i in 0..N {
        v1[i] = rng.gen();
        v2[i] = rng.gen();
        bools[i] = rng.gen::<u8>() < 128;
        // bools[i] = i % 2 == 0;
    }

    let a = do_branching_work(&v1, &v2, &bools);

    if a == Wrapping(0) {
        println!("Zero");
    }
}

#[inline(never)]
fn do_branching_work(v1: &[u32], v2: &[u32], bools: &[bool]) -> Wrapping<u32> {
    let mut a: Wrapping<u32> = Wrapping(0);

    for _ in 0..100 {
        for i in 0..v1.len() {
            if bools[i] {
                a += v1[i];
            } else {
                a += v2[i];
            }
        }
    }

    a
}
