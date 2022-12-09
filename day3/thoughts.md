# Day 8

## Part 1

Half in one 'compartment', half in other.
One symbol in both first half and second half
'priority' is 1..52 for a..zA..Z

Calculate sum of 'priority' of duplicated item in each pack


For 'priority' could construct a dict with each value - not so fast
Could create an array and look at each index to get the score (it will have gaps)

ord("a") == 97
ord("A") == 65
Could work out priority with 'subtract amount, check range, subtract again'
val = ord(x)
priority = val - 65 + 27 if (val - 65 < 26) else val - 96

To find duplicate item:
- create a set of each half, then find intersection

Keep running sum.


## Part 2
Groups of 3 elves; find item common to all

Exact same process should work; just do intersection of two then of third
Alternative - get ord of all items, count which one has three, then convert back
to character.
That's probably less efficient because it's basically doing what the set is doing but
dumber.
