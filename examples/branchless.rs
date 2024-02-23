use rust_perf_comp::*;
use std::num::Wrapping;

pub fn main() {
    let args = parse_args();

    let (v1, v2, bools) = fill_vecs(&args);

    let a = do_branchless_work(&v1, &v2, &bools);

    if a == Wrapping(0) {
        println!("Zero");
    }
}

#[inline(never)]
fn do_branchless_work(v1: &[u32], v2: &[u32], bools: &[bool]) -> Wrapping<u32> {
    let mut a: Wrapping<u32> = Wrapping(0);

    for _ in 0..100 {
        for j in 0..v1.len() {
            let vals = [v1[j], v2[j]];
            a += vals[bools[j] as usize];
        }
    }

    a
}