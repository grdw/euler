# Facts
- You can only use blocks with the width of 1 till `w`.
- A castle doesn't have to be max `h` high.

Let's imagine 10.000 `w` and 2 `h`.

How many ways can I stack blocks on the second layer?

## For 1 block:
1 block of width = 1 I can put in `w` spaces.
1 block of width = 2 I can put in `w` - 1 spaces
1 block of width = 3 I can put in `w` - 2 spaces
... and so on.

# For 3 blocks:


Imagine a 4 by 4, the bottom is already placed
[x x x x]
[x x x x]
[x x x x]

Assume you can just put a block anywhere and pin it to the grid? How many ways can you this? Let's break the rule of '1 space' between each block, and the stacking rule.


