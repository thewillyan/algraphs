# Graph algorithms
Joint study of some basic graph algorithms.

## Benchmarks and tests
To make fair measurements, we should use the same graphs for comparisons,
therefore use the `models` module, which provides standard graphs models
with the `GRAPHS` constant, is mandatory.

To make the benchmarks, we should use the 
[criterion](https://github.com/bheisler/criterion.rs) crate, (as shown in the
`benches` diretory) to get a more precise sample of the average performance of the
algorithms.

## Performance
Performance comparisons of developed graph structures in some algorithms.

|  Graph  | Get Degree | Max Degree | Is star |   Path   |                    Author                     |
|:--------|:----------:|:----------:|:-------:|:--------:|:---------------------------------------------:|
| UTGraph |   2.5 ns   |   24.1 ns  |  2.8 ns | 679.2 ns | [@thewillyan](https://github.com/thewillyan/) |

The algorithms are tested on the `GRAPH6` (provided by the `models` module) and
has the following structure:

```
     [0]     _[5]--[6]--[7]   [11]
    / | \   /      / \
 [1] [2] [3]     [8] [9]
       \ /         \ /
       [4]--------[10]
```

This graph is not ideal for performance benchmarks at all, it's just a
placeholder for a more robust solution that is yet to be developed.

### Benchmark computer specifications

|   Spec  |                      Config                      |
| :-----: | :----------------------------------------------- |
|  Device | Thinkpad T430                                    |
|   OS    | Void Linux  x86_64 (glibc)                       |
|  Kernel | 6.1.21_1                                         |
|   CPU   | Intel i5-3320M 3.300GHz                          |
|   GPU   | Intel 3rd Gen Core processor Graphics Controller |
|   RAM   | 8G DDR3 1333MHz                                  |
| Storage | KingSpec mSATA SSD 500G                          |
