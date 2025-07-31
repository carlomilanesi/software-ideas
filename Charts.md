# Charts

## Linear and non-linear charts

### When linear charts are appropriate

Let's think of a chart which maps the temperatures of a place in a weather bullettin.
Let's assume that in the range of values of that chart is [5, 36] °C.

It is quite reasonable that a temperature change from 5 to 6, a temperature change from 20 to 21, and a temperature change from 35 to 36 have the same importance.

In such a case, the best kind of scale for such a chart is the linear scale, i.e. the scale in which the ratio of any difference in temperature to the distance of the represented points for such temperatures is constant.

### When logarithmic charts are appropriate

Now, let's consider instead the yearly income of a population of persons or of business companies.

It is quite reasonable that the difference between the income of 10,000 and the income of 20,000 is much more important than the difference between the income of 210,000 and the income of 220,000.

In the first difference, one person has an income which is the double of the income of another person. Such a difference can really change the standard of living of these needy persons.

Instead, in the second difference, the two standard of living of these affluent people can be considered quite the same.

Therefore, a logarithmic chart would be more appropriate for such a case, than a linear chart.

Though, there are two kinds of logarithmic chart:
* **Real logarithimic**. The ones for which the zero value should be represented as minus infinite.
* **Adjusted logarithimic**. The ones for which the zero value should be represented as zero.

The real logarithmic is appropriate when the zero value cannot be really reached, and a value near zero, like 0.0001 is to be considered hugely different from a value not so near zero, like 0.1.
Instead, the adjusted logarithimic is appropriate when the zero value can actually be reached, and a value near zero, like 0.0001 is to be considered not hugely different from a value of zero, and not so hugely different from a value not so near zero, like 0.1.

To create a real logarithmic chart, it is enough to compute the logarithm of the value to represent.
To create an adjusted logarithmic chart, it is enough to add 1 to the value to represent, before computing its logarithm.

### When neither linear nor logarithmic charts are appropriate

In addition to linear charts and two kinds of logarithmic charts another kind of charts can be useful.

Consider the level of usage of a limited resource, like the percentage of occupied seats in a restaurant or in a theater, or the number of vehicle currently running in a road.

Such numbers have of course zero as their minimum value, meaning that such a resource is not used at all: the restaurant or theater of road are empty.
There is quite a big difference between an empty restaurant or theater or road and one with just one person or vehicle.

Though such numbers have also a maximum number: the seats in the restaurant or theater, the maximum number of running vehicles in the road.
For a reastaurant or a theater, such maximum number is well defined; but also for a road there is a number of vehicle which shouldn't be exceeded to avoid having the traffic to jam.

And there is quite a big difference between a full restaurant or theater or road and one with just one seat or car slot available.
Such available seat or car slot means that the resource is not saturated, and one other person or vehicle is allowed to enter.

An adjusted logarithmic chart can display with good detail the situation of almost unused resource, but it is inappropriate to display with enough detail the situation of almost full resource.
To display appropriately such a case, the best chart is made computing the difference between two adjusted logarithmic charts; one, positive, is translated to have its zero at the zero point, and the other, negative, is translated to have its zero at the maximum point.
