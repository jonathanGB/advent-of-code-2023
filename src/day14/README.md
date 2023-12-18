# Regarding Part 2

The question asks for the load after going through 1 billion cycles. The solution would take a very long time (I estimate many hours), so what I did was to print the load every 100k cycles. What soon emerged is that the load was recurring every 1 900 000 cycles. For instance, notice below how cycles 100 000, 2 000 000, and 3 900 000 all have the same load: 104 325. This is also true for the other cycles, e.g. 200 000/2 100 000/4 000 000 all have the same load (104 276), and so on.

So what I did was the find 1B % 1.9M, which is 600 000. Per the table below, the load associated with cycle 600 000 is 104 409, which was the answer!

| Cycle   | Load   |
|---------|--------|
|  100000 | 104325 |
|  200000 | 104276 |
|  300000 | 104248 |
|  400000 | 104315 |
|  500000 | 104388 |
|  600000 | 104409 |
|  700000 | 104351 |
|  800000 | 104278 |
|  900000 | 104252 |
| 1000000 | 104304 |
| 1100000 | 104373 |
| 1200000 | 104407 |
| 1300000 | 104371 |
| 1400000 | 104298 |
| 1500000 | 104268 |
| 1600000 | 104267 |
| 1700000 | 104348 |
| 1800000 | 104393 |
| 1900000 | 104403 |
| 2000000 | 104325 |
| 2100000 | 104276 |
| 2200000 | 104248 |
| 2300000 | 104315 |
| 2400000 | 104388 |
| 2500000 | 104409 |
| 2600000 | 104351 |
| 2700000 | 104278 |
| 2800000 | 104252 |
| 2900000 | 104304 |
| 3000000 | 104373 |
| 3100000 | 104407 |
| 3200000 | 104371 |
| 3300000 | 104298 |
| 3400000 | 104268 |
| 3500000 | 104267 |
| 3600000 | 104348 |
| 3700000 | 104393 |
| 3800000 | 104403 |
| 3900000 | 104325 |
| 4000000 | 104276 |
| 4100000 | 104248 |
| 4200000 | 104315 |