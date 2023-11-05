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