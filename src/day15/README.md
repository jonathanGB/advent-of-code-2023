# Benchmarks For Part 2

The initial solution was slightly simpler in that we made copies of the labels as `Vec<u8>` across our various data structures. That way, we didn't have to annotate lifetimes. However, once the solution was implemented, I refactored it so that we made no copies: all labels passed around were references to the original string passed down as a slice `& [u8]`.


Out of curiosity, I benchmarked the solution before and after this change and got a 2x improvement:

| Implementation | Runtime per iter (ns) | Std. dev. (ns) |
|----------------|-----------------------|----------------|
| Vec\<u8\>      | 288 366               | 4 638          |
| & [u8]         | 142 181               | 4 015          |