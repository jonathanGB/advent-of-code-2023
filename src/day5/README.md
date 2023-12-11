# Part 2 Benchmarks

The initial solution I came up with was quite slow to solve part 2. Therefore, I attempted alternative implementations, and came up with these runtimes. There are essentially two variants that I experimented with:
1. Try to find a matching `ConversationMap` using linear- versus binary-search.
2. Process all seed ranges in a single thread, or process each seed range in its own thread.

These numbers were measured by running `time cargo run --release day5 part2`. I ran each implementation a couple of times, and for each the runtime was within 1-2s.

| Implementation                   | Runtime |
|----------------------------------|---------|
| Linear search                    | 1m35s   |
| Linear search w/ multi-threading | 35s     |
| Binary search                    | 32s     |
| Binary search w/ multi-threading | 14s     |