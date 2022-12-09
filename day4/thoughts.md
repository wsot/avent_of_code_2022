# Day 4
Given two ranges, in how many cases does one fully contain the other

for a-b, c-d
if a<=c and d<=b
OR
if c<=a and b<=d
then it is fully enclosed

Alt approach:
create (a, b), (c, d)
sort them
if d <= b, then a-b fully encloses c, d
