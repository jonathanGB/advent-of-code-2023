# Some details about the solution

Given a time _T_ and a distance _D_, we define the distance traveled by the boat (_d_) as a function of the time the button was pressed (_t_). We want to find all natural values of _t_ that result in _d(t)_ greater than D in at most _T_ time. The equation goes as follows:

_d(t) = (T - t) * t = T * t - t<sup>2</sup>_.

What we more specifically want is to solve _d(t) > D_.

Which we can re-arrange as:

_d(t) = -t<sup>2</sup> + T * t - D > 0_

This is a quadratic equation, which we can solve using

_(-b ± √(b<sup>2</sup> - 4ac)) / 2a_

If we substitute the factors, we therefore get

_(T ± √(T<sup>2</sup> - 4D)) / 2_

With that, we get the two solutions where _d(t)_ crosses _D_. We can simply substract the two solutions, which gets us the number of ways to win the race.