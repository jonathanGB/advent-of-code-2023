# About part 2

Just like for day 20, the solution to part 2 does not complete within a reasonable amount of time. I started printing logs each 100 steps, and saw that the number of visited garden plots was almost quadratic, but not quite. I then saw online that the input had some recurrence occurring around 131 (which is the grid size), so I printed more logs every 131 steps this time, and this time I found an exact quadratic shape (see table).

| steps 	| visited garden plots 	| first degree difference 	| second degree difference 	|
|-------	|----------------------	|-------------------------	|--------------------------	|
| 65    	| 3849                 	|                         	|                          	|
| 196   	| 34331                	| 30482                   	|                          	|
| 327   	| 95175                	| 60844                   	| 30362                    	|
| 458   	| 186381               	| 91206                   	| 30362                    	|
| 589   	| 307949               	| 121568                  	| 30362                    	|
| 720   	| 459879               	| 151930                  	| 30362                    	|
| 851   	| 642171               	| 182292                  	| 30362                    	|
| 982   	| 854825               	| 212654                  	| 30362                    	|
| 1113  	| 1097841              	| 243016                  	| 30362                    	|
| 1244  	| 1371219              	| 273378                  	| 30362                    	|
| 1375  	| 1674959              	| 303740                  	| 30362                    	|
| 1506  	| 2009061              	| 334102                  	| 30362                    	|
| 1637  	| 2373525              	| 364464                  	| 30362                    	|
| 1768  	| 2768351              	| 394826                  	| 30362                    	|

With the numerous data points, I was able to figure out what the quadratic equation was. That is, I had to find _a_, _b_, and _c_ in _ax^2 + bx + c = y_. I used the _x_ and _y_ of the first 3 logs, plugged them in some matrices and solved the linear algebra problem. In the end, I found:

* _a = 15 181 / 17 161_
* _b = 30 901 / 17 161_
* _c = -95 601 / 17 161_

I validatated that subsequent number of steps resulted in the right amount of visited garden plots for the subsequent logs, and finally plugged the asked number of steps, that is 26501365. The equation gave the accepted answer.