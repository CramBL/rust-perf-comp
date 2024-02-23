# Rust performance comparison

## Example using `perf stat`

### Branching code
```shell
just COMP_RATIO=50 REPEAT=5 perf-branch
```
Output:
```
Requested ratio: 50% | N: 10,000,000 | Seed=0
True: 49.60% | 4,959,677/10,000,000

 Performance counter stats for 'target/release/examples/branch --seed 0 --N 10000000 --ratio 50' (5 runs):

    11.117.524.161      cpu_atom/cycles:u/                                                      ( +-  2,04% )  (0,37%)
    15.433.140.732      cpu_core/cycles:u/                                                      ( +-  0,22% )  (99,63%)
    12.250.330.486      cpu_atom/instructions:u/         #    1,10  insn per cycle              ( +- 19,12% )  (0,37%)
    13.148.396.527      cpu_core/instructions:u/         #    1,18  insn per cycle              ( +-  0,06% )  (99,63%)
     2.564.184.432      cpu_atom/branches:u/                                                    ( +-  1,61% )  (0,37%)
     3.574.404.585      cpu_core/branches:u/                                                    ( +-  0,04% )  (99,63%)
       298.878.195      cpu_atom/branch-misses:u/        #   11,66% of all branches             ( +- 15,93% )  (0,37%)
       500.465.785      cpu_core/branch-misses:u/        #   19,52% of all branches             ( +-  0,03% )  (99,63%)

            3,4814 +- 0,0128 seconds time elapsed  ( +-  0,37% )

```
### Branchless code

```shell
just COMP_RATIO=50 REPEAT=5 perf-branchless
```
Output:
```
Requested ratio: 50% | N: 10,000,000 | Seed=0
True: 49.60% | 4,959,677/10,000,000

 Performance counter stats for 'target/release/examples/branchless --seed 0 --N 10000000 --ratio 50' (5 runs):

     1.831.065.743      cpu_atom/cycles:u/                                                      ( +-  3,56% )  (0,37%)
     3.144.111.726      cpu_core/cycles:u/                                                      ( +-  2,77% )  (99,63%)
     4.474.920.060      cpu_atom/instructions:u/         #    2,44  insn per cycle              ( +- 10,59% )  (0,37%)
    14.690.658.311      cpu_core/instructions:u/         #    8,02  insn per cycle              ( +-  0,07% )  (99,63%)
       524.181.731      cpu_atom/branches:u/                                                    ( +- 16,11% )  (0,37%)
     2.076.262.215      cpu_core/branches:u/                                                    ( +-  0,08% )  (99,63%)
         3.084.569      cpu_atom/branch-misses:u/        #    0,59% of all branches             ( +- 12,44% )  (0,37%)
               974      cpu_core/branch-misses:u/        #    0,00% of all branches             ( +- 14,75% )  (99,63%)

            0,7247 +- 0,0200 seconds time elapsed  ( +-  2,76% )
```