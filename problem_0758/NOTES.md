# Pattern matching
For this problem at some point - for the lower numbers I started to puke out
some data. Mostly patterns of pouring. These were the one's I tested it for:

# [63, 485, 548]
Starts with ML
Repeating pattern is "SM-LS-SM-LS-SM-LS-SM-LS-SM-LS-SM-LS-SM-LS-SM"

# [63, 33613, 33676]
SL-MS repeats **a lot** before one LM

# [485, 6249, 6734]
ML-SM repeats in a certain pattern for sure... but how the f am I supposed
to figure out how it 'repeats'.

# [485, 33613, 34098]
ML-SM repeats

Hmmm. I feel like there are maybe N strategies that are valid and all of them
have a specific pattern. If I get all the strategies and execute them in an
efficient way, than figure out which takes the least amount of pours.

There are two strategies thusfar that I can find (unless others appear).
- It's either a strak of [SL-MS {n}, LM]{n} or [ML, SM-LS{n}]{n}

