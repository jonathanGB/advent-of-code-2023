# Part 2 Benchmarks

Memoizing results was crucial to solving part 2, otherwise this performs poorly due to the state explosion. I have played with a few different configurations along 3 axes:

1. Global versus local cache. That is, is there one global cache for all records, or one local per record.
2. Owned (i.e. vector) cache key versus not-owned (i.e. slice) key. That is, do we need to allocate memory for the key, or can we use a reference to the slice.
3. Preemptively reserve versus not reserve space for the cache. If done, this is likely to save re-allocations of the cache when the cache eventually gets too big. Note that the 300k and 2k limits were chosen based on the maximum size the global/local caches got while solving part 2.

Based on this experimentation, the most efficient solution was a local cache with a slice key and a 2k reservation.


| Cache  | Cache key | Cache reserve | Runtime per iter (ns) | Std. dev. (ns) |
|--------|-----------|---------------|-----------------------|----------------|
| Global | Vector    | No            | 189,801,560           | 17,037,485     |
| Global | Vector    | Yes - 300k    | 112,777,090           | 7,914,133      |
| Global | Slice     | No            | 103,788,640           | 4,687,102      |
| Global | Slice     | Yes - 300k    | 61,736,930            | 3,533,517      |
| Local  | Vector    | No            | 127,139,090           | 3,155,575      |
| Local  | Vector    | Yes - 2k      | 100,285,110           | 2,843,110      |
| Local  | Slice     | No            | 80,314,490            | 2,009,974      |
| Local  | Slice     | Yes - 2k      | 55,533,120            | 2,460,875      |