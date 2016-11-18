Algorithm for generating a random self-avoiding path from one point to another on a 2D-grid.
The algorithm will randomly walk until it reaches a dead end. Then, it will retreat from the dead end point,
as well as all previous points that had only one possible direction. It will blacklist all points it
retreats from (they are no longer walkable). If path length decreases to 0, the algorithm will restart itself.