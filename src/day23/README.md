# Notes about the optimization discarding a node

There is a point that should be considered a node during compaction (i.e. a crossroad linking more than two paths), but which is ignored by this optimization. That point towards the end is ignored because there is a single valid way to traverse it (i.e. going down, towards the end). Otherwise, because we can't visit a point more than once, the path will necessarily never be able to reach the end, thus can be pruned.

I have timed with and without the optimization, and overall it has improved from ~5s to ~2.6s.

## Other improvements

As mentioned above, the optimized solution took ~2.6s. I re-implemented the iterative DFS to a recursive DFS that no longer required to make a copy of the visited set whenever we started a new path. Rather, we insert the current node into the visited set before recursively visiting all its edges, and then remove it once all recursions are over. This is functionally equivalent, but significantly improved the runtime to now be ~1.4s.

Eventually, I replaced the visited set from a HashSet to a 2-d array, and the runtime then again improved to now be ~400ms. We could probably improve it further by moving the graph HashMap to an array as well, but I haven't tried. Though, I have changed from using the standard library's HashMap/HashSet to the hashbrown crate (which has the same API), and runtime further improved to ~290ms.