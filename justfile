BIN := "target/release/rust-perf-comp"
BRANCH_BIN := "target/release/examples/branch"
BRANCHLESS_BIN := "target/release/examples/branchless"

COMP_RATIO :="0"
N :="10000000"
SEED :="0"
REPEAT :="1"
NO_RND_CMP := "FALSE"

build:
    cargo build --release
    cargo build --examples --release

# Run perf on the example, specifying ratio (0-255) for the branch, size of vectors, and seed.
perf-branch:
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

# Run perf on the example, specifying ratio (0-255) for the branch, size of vectors, and seed.
perf-branchless:
    #!/usr/bin/env bash
    declare no_rnd_cmp=""
    if [ "{{ NO_RND_CMP }}" != "0" ]; then
        no_rnd_cmp="--no-rnd-cmp"
    fi
    perf stat --repeat {{ REPEAT }} \
        -e cycles,instructions,branches,branch-misses \
        {{ BRANCHLESS_BIN }} \
        --seed {{ SEED }} \
        --N {{ N }} \
        --ratio {{ COMP_RATIO }} \
        ${no_rnd_cmp}

