BIN := "target/release/rust-perf-comp"
BRANCH_BIN := "target/release/examples/branch"
BRANCHLESS_BIN := "target/release/examples/branchless"

# Default values for the parameters.

# Ratio of true values in the vector of booleans. 0-100 %.
COMP_RATIO := "100"
# Number of elements in the vectors.
N := "10000000"
# Seed for the random number generator.
SEED := "0"
# Number of times to run the binary with perf.
REPEAT := "1"
# FALSE: Use a random number generator to initialize the vector of booleans.
# TRUE: Use a modulo operation to initialize the vector of booleans, creating a deterministic pattern.
NO_RND_CMP := "FALSE"

@build:
    cargo build --release
    cargo build --examples --release

# Run perf on the branching example.
perf-branch: build
    #!/usr/bin/env bash
    declare no_rnd_cmp=""
    if [ "{{ NO_RND_CMP }}" != "FALSE" ]; then
        no_rnd_cmp="--no-rnd-cmp"
    fi
    perf stat --repeat {{ REPEAT }} \
        -e cycles,instructions,branches,branch-misses \
        {{ BRANCH_BIN }} \
        --seed {{ SEED }} \
        --N {{ N }} \
        --ratio {{ COMP_RATIO }} \
        ${no_rnd_cmp}

# Run perf on the branchless example.
perf-branchless: build
    #!/usr/bin/env bash
    declare no_rnd_cmp=""
    if [ "{{ NO_RND_CMP }}" != "FALSE" ]; then
        no_rnd_cmp="--no-rnd-cmp"
    fi
    perf stat --repeat {{ REPEAT }} \
        -e cycles,instructions,branches,branch-misses \
        {{ BRANCHLESS_BIN }} \
        --seed {{ SEED }} \
        --N {{ N }} \
        --ratio {{ COMP_RATIO }} \
        ${no_rnd_cmp}

