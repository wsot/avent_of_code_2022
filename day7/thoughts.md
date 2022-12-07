# Thoughts
Size direct and indirect - so calculating with all nested children
Files can be counted more than once

The names of directories is irrelevant - important thing is that `$ cd ..`
means `going up a level` and `$ cd <anything else>` means going down a level

If going up a level, then the total should be added to the parent

Nodes need a size, and children? Parent-child relationship
is not actually important except the list of _current_ parents when
traversing.

Input appears to be a depth-first traversing of the tree. Should validate
that is the case (i.e. do a check to make sure that if a listing contains
dirs, then each of those dirs is traversed in order from first to last)
