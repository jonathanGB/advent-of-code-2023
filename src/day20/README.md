# Notes about part 2

Running the solution for part 2 will not finish in a reasonable amount of time. Module `tx` only receives signals from module `&tj`, which is a conjunction module. Therefore, for `tx` to receive a low pulse, `tj` must have received high pulses from all of its inbound modules, which are `&vt`, `&kk`, `&sk`, and `&xc`.

What I did was to add some logs everytime `tj` received a high pulses from each of these four inbound modules. What I discovered is that each had a linear reccurrence, specifically:
  * `vt` sent a high pulse every 3943 button presses
  * `kk` sent a high pulse every 3931 button presses
  * `sk` sent a high pulse every 3917 button presses
  * `xc` sent a high pulse every 4057 button presses

Therefore, I found the least common multiple of these four periods, which was the answer.