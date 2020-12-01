# Day 1
Day 1 asks us to find two numbers that sum to 2020 in a list and to calculate their product.  
The list is fairly short (200 entries), so I looped over the entries and checked if the list contained each number's
complement (2020 - number).

Part 2 asks for three numbers that sum to 2020.  Since the list is short, checking all of the combinations is feasible,
so I used [Itertools.combinations(3)](https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.combinations)
to find a triplet that sums to 2020.

Sorting the numbers and performing a binary search for the compliment in part 1, or the second / third number in part 2
would be more performant.
