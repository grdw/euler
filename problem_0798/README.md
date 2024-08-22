# Problem 798

- There are two players who take turns
- There's a single deck of cards with S suits where each suit has 1 to n cards.
- A set of 0 to X cards are picked from the deck and placed face up on the table

Each player will take turns, and then *choose* a card form the deck placing it
face up on top of one of the visible cards. The rules are:

- The cards must be of the same suite
- The value of the card needs to be larger
- The card that's put on top of the visible card is the new 'visible card'
- The player unable to move loses and the game is over.

# Step 1: Making a deck
A deck can be very easily made with some basic rust. Considering the amount of suits needs to support 10^7 values, and the amount of cards have 10^7 values, we'll have 10^7 * 10^7 cards in the deck or 10^14.

In the simple example of 3 cards with 2 suits, it says that the first player - considering all possible solutions will - lose 26 times. Let me write that out.

Amount of cards:
{A1, A2, A3, B1, B2, B3}

visible cards | losses
--------------|-----------
0             | 1?
1             | 4
2             | 4 (and some more)
3             |
4             |
5             | 4
6             | 1

Amount of visible cards you can pick:
## 0 (1)
In this case there's no play and player 1 automatically loses according the rules. However, I'm not sure if that's true. I'll come back to this.

## 1 (4)
There are 6 possible cards to pick. If A3 or B3 are picked, player 1 loses because there's no card higher. If A1 or B1 are picked player 1 also loses because it will have to pick either A2 or B2, player 2 will pick A3 or B3 and player 1 will lose. So in 4 out of 6 cases player 1 will lose.

## 2 (4?)
In this case there's (N/K) 6/2 = 15 possible starting positions. Each starting position comes with its own set of outcomes.

```
6! / 2!(6-2)! =
(6*5*4*3*2) / 2(4*3*2)
6*5 / 2
30 / 2 = 15
```

In which cases does player 1 loses? I have no idea, so just try it out I guess

### A1, A2
### B1, B2
In this case player 1 wins, because he plays A3. And there will be only B's left for player 2.

### A1, A3
### B1, B3
In this case player 1 wins because he plays A2, and there will be only B's left.

### A2, A3 (1)
### B2, B3 (1)
### A3, B3 (1)
Player 1 loses because he can't move

### A2, B3
### A3, B2
In this case player 2 loses because player 1 can play 1 card before locking the game.

### A3, B1
### A1, B3
In both these cases player 1 wins because player 1 plays either A3 or B3 and player 2 loses.

TODO: Figure out these cases
### A1, B1
This really depends ... player 1 could play A3 or B3 to immediately make it harder player 2, but is that 'best play'? If player 2 does the same in return it will automatically make player 1 lose. It would be _better_ for player 1 to throw up A2 or B2 instead.... in this case player 2 can choose to throw A3, B3 or A2/B2 (whichever is left).

### A1, B2
### A2, B1
Player 1 will always win this one only if he throws A2. If player 1 throws B3 he's an idiot.

### A2, B2 (1)
This one is lost by player 1 because player 1 will throw A3 or B3, which causes player 2 to throw A3 or B3 and player 1 loses.

## 3
6 over 3 which means there are 20 starting positions.

```
6! / 3!(6-3)!
6*5*4*3*2 / (3*2)(3*2)
6*5*4 / 3*2
120 / 6 = 20
```

A1,A2,A3 1
B1,B2,B3 1
A3,B3,A1 1
A3,B3,A2 1
A3,B3,B1 1
A3,B3,B2 1
A1,A2,B1
B1,B2,A1
A1,A3,B1
B1,B3,A1

B1,B3,A2 1
A1,A3,B2 1


## 4
6 over 4 means 15 starting positions.

```
6! / 4!(6-4)!
6*5*4*3*2 / (4*3*2)*2
6*5 / 2 = 15
```

## 5 (4)
6 over 5 means you have 6 starting positions.

```
6! / 5!(6-5)!
6*5*4*3*2 / (5*4*3*2)*1
6 starting positions
```

In 4 out of 6 cases player 1 will lose simply because it cannot move.
(A2, A3, B1, B2, B3) x
(A1, A3, B1, B2, B3) x
(A1, A2, B1, B2, B3)
(A1, A2, A3, B2, B3) x
(A1, A2, A3, B1, B3) x
(A1, A2, A3, B1, B2)

## 6 (1)
I'm not sure on the rules here but player 1 cannot move here.

---

It feels like quite a lot to go over for even such a simple example. What about a decision tree, or what about every possible permutation of group size 0 till N.

{A1,A2,A3,B1,B2,B3}
{A1,A2,A3,B1,B2}
{A1,A2,A3,B1,B3}
{A1,A2,A3,B2,B3}
{A1,A2,B1,B2,B3}
{A1,A3,B1,B2,B3}
{A2,A3,B1,B2,B3}
{A1,A2,A3,B1}
{A1,A2,A3,B2}
{A1,A2,A3,B3}
// and so on
{}

Let's make the example dumber. Let's say you have 1 suite and 2 cards.

{A1,A2}

The possible options are:

{A1,A2} P1 loses
{A1} P1 wins
{A2} P1 loses
{} P1 loses

3 out of 4 he loses. Easy

Let's you have 2 suites and 2 cards.

{A1,A2,B1,B2}

{A1,A2,B1,B2} P1 loses (deck empty)
{A1,A2,B1} P1 wins
{A1,A2,B2} P1 loses
{A1,B1,B2} P1 wins
{A2,B1,B2} P1 loses
{A1,A2} P1 loses
{A1,B1} P1 loses
{A1,B2} P1 wins
{A2,B1} P1 wins
{A2,B2} P1 loses
{B1,B2} P1 loses
{A1} P1 wins
{A2} P1 loses
{B1} P1 wins
{B2} P1 loses
{} P1 loses

10 out of 16 games p1 loses

*What happens if you increase the amount of suites?*
*What happens if you increase the amount of cards within the suite?*

| Suites | Cards per suite | Total cards | Losses / total games     |
|--------|-----------------|-------------|--------------------------|
| 1      | 2               | 2           | 3/4                      |
| 2      | 2               | 4           | 10/16                    |
| 2      | 3               | 6           | 26/64                    |
| 3      | 2               | 6           | ?/64                     |
| 4      | 13              | 52                                     |

3 suits 2 cards
{A1,A2,B1,B2,C1,C2}

6 over 6 | 1
6 over 5 | 6
6 over 4 | 15
6 over 3 | 20
6 over 2 | 15
6 over 1 | 6
6 over 0 | 1
Totals to: 64

52 over 19 | 76360380541900
52 over 20 | 125994627894135
52 over 21 | 191991813933920
52 over 22 | 270533919634160
52 over 23 | 352870329957600
52 over 24 | 426384982032100
52 over 25 | 477551179875952
52 over 26 | 495918532948104

Loads of cards. How do you even do this with 10^7 cards?

You have 10^14 amount of cards. And an insane of amount of opening positions. I mean there must be just some silly math trick to this.

The rules are:

- V equals the amount of visible cards being on the table.

- if V = 0 P1 loses
- if V = MAX(N), where the suits are 1 till S. P1 always loses because there's no possible play.
- if V = N x S then P1 also always loses because all the cards are on the table

Definite losses = 2 + ((1 till S) over S)

if V != MAX(N), and V = 1 card. I'm assuming in this case that P1 will always throw the highest card so it wins.

V = 2 cards P1.

3 cards 2 suits

2 + 3 = 5

- For 1 card P1 only loses if the highest card isn't immediately visible. So the amount of loses for 1 card == S (2)
- For 2 cards:

{3A, 3B} = P1 loss
{3A, 2B}, {2A, 3B} = P1 wins
{2A, 2B} = P1 loss
{1A, 1B} =
  Either P1 throws 3A, then P2 can throw 3B (L)
  Either P1 throws 2A, then P2 can throw 3B (or 3A), P1 throws 3A or 3B (W)
  Either P1 throws 2A, then P2 can throw 2B, P1 throws 3A, P2 throws 3B (L)
  What is best play in this situation? The one where P1 wins or am I to
  assume that P2 also engages in best play? If that's the case than P1 always
  loses this one.

How many situations do you have. With uneven and even I mean, 'an even' amount of cards removed from the highest card or 'an uneven' amount of cards removed from the highest card. So:

if A17 is the highest, and A4 is on the table, than there are 13 cards left.

Even   | Uneven
Even   | Even
Uneven | Uneven

Or could you also argue you have an uneven amount of cards exclusively consisting of uneven numbers?
