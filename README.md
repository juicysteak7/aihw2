# AI HW2
Pedro Gonzalez

Professor Bart Massey

CS441

## Background

There are many Tic-Tac-Toe variants out there, including some that are played on a 4×4 board. We will build AI for Fourtic, a 4×4 game invented for the class.

## Fourtic Rules

Fourtic is a two-player game for children or adults. The expected playtime is around 10 minutes.

Fourtic is played on a 4×4 grid. The cells are named as in chess. The players are named X and O as in Tic-Tac-Toe: X is first to move.

Here is an empty Fourtic board.

1 ....

2 ....

3 ....

4 ....

  abcd

One possible starting move is b2. This would lead to the board

1 ....

2 .X..

3 ....

4 ....

  abcd

The position might then continue like

.O..  .O..  XOO.  XOO.

.X..  .X..  .X..  .X..

....  ..X.  ..X.  ..X.

....  ....  ....  ...O

Players alternate turns until the board is completely filled (eight moves by each side). When the game is over, the player with the most points wins.

Point scoring is in two forms:

    Every row of three squares owned by a player counts three points for that player. Note that a row of four, which contains a row of three, is thus worth six points.

    Every side or corner cell owned by a player is worth one bonus point to that player.

For example, consider this end position.

OOXO

XXXX

OXOX

OOXO

In this position:

    X has one row of four, one row of three and five side cells. X's score is 14.

    O has no rows and seven side cells. O's score is 7.

Thus, X wins (by 7).

## Assignment

Write a solver for Fourtic instances. Call your program "fourtic".

Your program should read an Fourtic position in the ASCII format described above (without row and column numbers) from a file whose name is supplied on the command line. Your program should infer whose turn it is from the position (number of Xs and Os).

Your program should evaluate the position using negamax search. It should then print a line indicating the side on move and the value of the position for the side on move.

For example, given a file random-5.txt containing this position:

.XOX

.O.X

..OX

OXXO

your program should print

O 5

Your program must solve all the instances present in the Git repo https://github.com/pdx-cs-ai/hw-fourtic

Links to an external site.. Each instance should be solved in less than five seconds. A negamax search written in Python should have no problem with this even on a slow machine.

In addition, your program should solve other instances we might try during grading.

## Extra Credit

10 points: add a "player" program that allows a human to play Fourtic as X against your program. This will require depth-limited search (at least near the start of the game) to play at reasonable speed. Use a depth 4 search.

Show a game trace from a human game in your README.

20 points: add αβ (alpha-beta) pruning to your search, as well as some move-ordering heuristic.

Report on the performance improvement from αβ in your README.

Solve Fourtic. Choose one of these:

    40 points: Weakly solve Fourtic by good alpha-beta search from the starting position.

    Your program should produce a table giving the best move for every main line Fourtic position. A main line position is a position encountered in your search.

    50 points: Strongly solve Fourtic by complete Retrograde Analysis 

Links to an external site. from all possible ending positions.

Your program should produce a table giving the best move for every Fourtic position .

Include a link to a solution table stored somewhere online (probably Github) in your submission. Compress the table so that it is reasonably sized. Report the value of Fourtic in your README.

## Project Notes

I started this assignment ambitious about creating an a.i. player, I wanted to be able to have an A.I. that can beat me every time.

I also didn't want long runtimes to make the game unplayable. I managed to keep run times down but I can still beat my A.I. even when it is tuned to be very difficult. The A.I. player can beat me sometimes.

All in all this assignment was a lot of fun and gave me a lot of foundational ideas for my course project in this class and Bart's other Rust class.

I completed two extra credits for this assignment, adding an A.I. player, and adding alpha beta pruning.

The alpha beta pruning, and transposition tables are the main factor to keeping run times low even when looking deep into the game tree.