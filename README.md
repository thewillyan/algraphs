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

