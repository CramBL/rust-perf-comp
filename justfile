BIN := "target/release/rust-perf-comp"
BRANCH_BIN := "target/release/examples/branch"
BRANCHLESS_BIN := "target/release/examples/branchless"

CORE_ONLY_EVENTS := "duration_time,cpu_core/cycles/,cpu_core/instructions/,cpu_core/branches/,cpu_core/branch-misses/"
ATOM_ONLY_EVENTS := "duration_time,cpu_atom/cycles/,cpu_atom/instructions/,cpu_atom/branches/,cpu_atom/branch-misses/"
BOTH_CORE_AND_ATOM_EVENTS := "duration_time,cycles,instructions,branches,branch-misses"

CPU_EVENTS := "BOTH"

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
perf-branch: build && (run-perf BRANCH_BIN)

# Run perf on the branchless example.
perf-branchless: build && (run-perf BRANCHLESS_BIN)

run-perf BINARY:
    #!/usr/bin/env bash
    shopt -s nocasematch
    declare cpu_events=""
    case "{{ CPU_EVENTS }}" in
    "CORE" )
            cpu_events="{{ CORE_ONLY_EVENTS }}"
            ;;
    "ATOM" )
            cpu_events="{{ ATOM_ONLY_EVENTS }}"
            ;;
    "BOTH" )
            cpu_events="{{ BOTH_CORE_AND_ATOM_EVENTS }}"
            ;;
    esac
    echo "Running perf on {{BINARY}} with events: ${cpu_events}"

    declare no_rnd_cmp=""
    if [ "{{ NO_RND_CMP }}" != "FALSE" ]; then
        no_rnd_cmp="--no-rnd-cmp"
    fi
    perf stat --repeat {{ REPEAT }} \
        --event="${cpu_events}" \
        {{ BINARY }} \
        --seed {{ SEED }} \
        --N {{ N }} \
        --ratio {{ COMP_RATIO }} \
        ${no_rnd_cmp}

perf-record-branch: build && (perf-record BRANCH_BIN "branch")
perf-record-branchless: build && (perf-record BRANCHLESS_BIN "branchless")

perf-record BINARY FILE:
    perf record \
        -o {{ FILE }}.data \
        --event="{{ BOTH_CORE_AND_ATOM_EVENTS }}" \
        {{ BINARY }} \
        --seed {{ SEED }} \
        --N {{ N }} \
        --ratio {{ COMP_RATIO }}

perf-stat-record-branch FILE="branch": (perf-stat-record BRANCH_BIN FILE)
perf-stat-record-branchless FILE="branchless": (perf-stat-record BRANCHLESS_BIN FILE)

perf-stat-record BINARY FILE: && (perf-stat-record-json-clean FILE FILE)
    perf stat --repeat {{ REPEAT }} \
        -o {{ FILE }} \
        --json-output \
        --event="{{ BOTH_CORE_AND_ATOM_EVENTS }}" \
        {{ BINARY }} \
        --seed {{ SEED }} \
        --N {{ N }} \
        --ratio {{ COMP_RATIO }}

# Clean the perf stat record json output to make it valid json
perf-stat-record-json-clean JSON_FILE OUTPUT_FILE:
    #!/usr/bin/env bash
    if [[ ! -x {{ BIN }} ]]; then
        cargo build --release
    fi
    {{ BIN }} clean-perf-stat-json --input-file {{ JSON_FILE }} --output-file {{ OUTPUT_FILE }}

line-over-x XVALS JSON_DIR BR_PRE BL_PRE SAVE_TO PLOT_TYPE:
    cargo run -r -- \
        line-over-x {{ XVALS }} \
        --json-dir {{ JSON_DIR }} \
        --branching-prefix {{ BR_PRE }} \
        --branchless-prefix {{ BL_PRE }} \
        --save-to {{ SAVE_TO }} \
        --plot-type {{ PLOT_TYPE }}
    firefox {{ SAVE_TO }}


full-run SAVE_TO:
    #!/usr/bin/env bash
    declare -r ratios="0 5 10 15 20 25 30 35 40 45 50 55 60 65 70 75 80 85 90 95 100"
    #declare -r ratios="0 20 40 50 60 80 100"
    declare -i repeat=5
    for ratio in ${ratios}; do
        just COMP_RATIO=${ratio} REPEAT=${repeat} perf-stat-record-branch "json_dir/branch${ratio}.json"
        just COMP_RATIO=${ratio} REPEAT=${repeat} perf-stat-record-branchless "json_dir/branchless${ratio}.json"
    done
    just line-over-x "${ratios}" json_dir branch branchless "{{ SAVE_TO }}/cpu_instruction.svg" cpu-instructions
    just line-over-x "${ratios}" json_dir branch branchless "{{ SAVE_TO }}/time_branch_miss.svg" time-branch-misses