Algorithm for generating a random self-avoiding path from one point to another on a 2D-grid.
The algorithm will randomly walk until it reaches a dead end. Then, it will retreat from the dead end point,
as well as all previous points that had only one possible direction. It will blacklist all points it
retreats from (they are no longer walkable). If path length decreases to 0, the algorithm will restart itself.

Text output looks like this:

cycle: 219

    ░░░░░██████░░░███░░░░░░░░░░░░░██████░░░░xxx░░░░xxx
    ░░░░██░░░░█░███░█░░░░░░░░░░████░░░░██░░xx░xx░░░x░x
    ░░░░█░░░░░███░░░███░░░░░░░░█░░░░░░░░██░x░░x░xxxxx░
    ░░░░███░░░░░░xxxx░█████░░░░██░░░░░░░░██░xxxxx░x░xx
    ░░░░░░███░░xxx░x░xx░░░██E░░░█░░░░░░░░░██░░░x░xxx░x
    ░░░░░░░░██░x░░░░░░░░░░░░S░░░█░░░░░░░░░░█xx░░xx░░░x
    ░░░░░░░░░██░░██████░░░░░█████░░░░░░░░░██░xxx░xxx░x
    ░░░░░░░░░░████░░░░█░░░░░░░░░░░███░░░░░█░x░░xx░░x░x
    ░░░░░░░░░░░░░░░░░░██░███████░██░█░░░███░x░x░xx░x░x
    ░░░░░░░░░░░░░░░░░░░███░░░░░███░░█████░xxxxx░░xxx░x