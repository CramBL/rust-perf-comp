#![allow(non_snake_case)]
use num_format::{Locale, ToFormattedString};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
pub use std::num::Wrapping;

pub mod config;
pub mod plot_perf_stats;

const HELP: &str = "\
Rust performance comparitor

USAGE:
  rust-perf-comp --seed <SEED> --N <N> --ratio <RATIO> [--no-rnd-cmp]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --seed <SEED>         Seed for the random number generator
  --N <N>               Number of elements in the vectors
  --ratio <RATIO>       Ratio of true values (0-100 %)
  --no-rnd-cmp          Disable random comparison generation and use <RATIO> for modulo comparison
";

pub struct AppArgs {
    pub seed: u64,
    pub N: usize,
    pub ratio: u16,
    pub rnd_cmp: bool,
}

impl std::fmt::Display for AppArgs {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Requested ratio: {}% | N: {} | Seed={} ",
            self.ratio,
            self.N.to_formatted_string(&Locale::en),
            self.seed,
        )
    }
}

#[inline(always)]
pub fn parse_args() -> AppArgs {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{HELP}");
        std::process::exit(0);
    }

    let args = AppArgs {
        seed: pargs.value_from_str("--seed").unwrap(),
        N: pargs.value_from_str("--N").unwrap(),
        ratio: pargs.value_from_str("--ratio").unwrap(),
        rnd_cmp: !pargs.contains("--no-rnd-cmp"),
    };
    println!("{args}");
    args
}

/// Convert the ratio specified in percentage, to a u8 value for either random comparison or modulo comparison
#[inline(always)]
fn arg_ratio_to_u16(args: &AppArgs) -> u16 {
    if args.rnd_cmp {
        if args.ratio == 100 {
            256
        } else {
            (u8::MAX as f32 / 100.0 * args.ratio as f32) as u16
        }
    } else {
        // Convert the percentage to a value for use in modulo operation
        // It doesn't fit very well with the 0-100% range, but it's good enough for this purpose
        match args.ratio {
            0 => u16::MAX,
            51..=100 => 1,
            34..=50 => 2,
            26..=33 => 3,
            21..=25 => 4,
            18..=20 => 5,
            16..=17 => 6,
            14..=15 => 7,
            x => 21 - x,
        }
    }
}

/// Pretty print the stats for the vector of bools
#[inline(always)]
pub fn print_how_many_true(bools: &[bool], args: &AppArgs) {
    let total_bools: u32 = bools.iter().map(|&x| if x { 1 } else { 0 }).sum();
    let percent = (total_bools as f64 / args.N as f64) * 100.0;

    let str_total_bools = total_bools.to_formatted_string(&Locale::en);
    let str_n = args.N.to_formatted_string(&Locale::en);

    println!("True: {percent:.2}% | {str_total_bools}/{str_n}");
}

/// Fill 2 vectors with random numbers and a vector of bools with a method specified on the command line
#[inline(always)]
pub fn fill_vecs(args: &AppArgs) -> (Vec<u32>, Vec<u32>, Vec<bool>) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(args.seed);

    // Get a u16 value from the 0-100% ratio specified on the command line
    let ratio_val: u16 = arg_ratio_to_u16(args);

    // If random comparison is enabled, use a random number generator to fill the boolean vector
    // If not, use a modulo comparison to fill the boolean vector (deterministic comparison)
    let comp_fn: fn(usize, &mut ChaCha8Rng, u16) -> bool = if args.rnd_cmp {
        |_, rng: &mut ChaCha8Rng, ratio| (rng.gen::<u8>() as u16) < ratio
    } else {
        |i, _: &mut ChaCha8Rng, ratio| (i + 1) % ratio as usize == 0
    };

    // Create the vectors and fill them with random numbers and boolean values
    let mut v1: Vec<u32> = vec![0; args.N];
    let mut v2: Vec<u32> = vec![0; args.N];
    let mut bools: Vec<bool> = vec![false; args.N];

    for i in 0..args.N {
        v1[i] = rng.gen();
        v2[i] = rng.gen();
        bools[i] = comp_fn(i, &mut rng, ratio_val);
    }

    print_how_many_true(&bools, args);

    (v1, v2, bools)
}
