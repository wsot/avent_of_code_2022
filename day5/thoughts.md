# Day 5

The most obvious approach is to represent this as a list of lists,
split the instructions into three numbers x, y and z,
and pop x items from y and push them to z

Probably could also represent them as a set of transforms
trying to collapse transforms when possible
e.g.
move 1 from 1 to 2
move 1 from 2 to 3

net result
move 1 from 1 to 3
however, if anything _else_ happened to 3 in the meantime
then that could mess it up

e.g.
move 1 from 1 to 2
move 1 from 1 to 3
move 1 from 2 to 3
can't be collapsed at all

move 1 from 1 to 2
move 1 from 4 to 3
move 1 from 2 to 3

could be collapsed, but it would need to collapse to
move 1 from 4 to 3
move 1 from 1 to 2

All up, the complexity of trying to collapse the instructions
may well not be worth it, especially if the original representation
isn't all that big


