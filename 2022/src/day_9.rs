/*
--- Day 9: Rope Bridge ---

This rope bridge creaks as you walk along it. You aren't sure how old it is, or whether it can even support your
weight.

It seems to support the Elves just fine, though. The bridge spans a gorge which was carved out by the massive
river far below you.

You step carefully; as you do, the ropes stretch and twist. You decide to distract yourself by modeling rope
physics; maybe you can even figure out where not to step.

Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. If the head moves
far enough away from the tail, the tail is pulled toward the head.

Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a
two-dimensional grid. Then, by following a hypothetical series of motions (your puzzle input) for the head, you
can determine how the tail will move.

Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must
always be touching (diagonally adjacent and even overlapping both count as touching):

....
.TH.
....

....
.H..
..T.
....

...
.H. (H covers T)
...

If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in
that direction so it remains close enough:

.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...

Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one
step diagonally to keep up:

.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the
tail both start at the same position, overlapping.

For example:

R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

This series of motions moves the head right four steps, then up four steps, then left three steps, then down one
step, and so on. After each step, you'll need to update the position of the tail if the step means the head is no
longer adjacent to the tail. Visually, these motions occur as follows (s marks the starting position as a reference
point):

== Initial State ==

......
......
......
......
H.....  (H covers T, s)

== R 4 ==

......
......
......
......
TH....  (T covers s)

......
......
......
......
sTH...

......
......
......
......
s.TH..

......
......
......
......
s..TH.

== U 4 ==

......
......
......
....H.
s..T..

......
......
....H.
....T.
s.....

......
....H.
....T.
......
s.....

....H.
....T.
......
......
s.....

== L 3 ==

...H..
....T.
......
......
s.....

..HT..
......
......
......
s.....

.HT...
......
......
......
s.....

== D 1 ==

..T...
.H....
......
......
s.....

== R 4 ==

..T...
..H...
......
......
s.....

..T...
...H..
......
......
s.....

......
...TH.
......
......
s.....

......
....TH
......
......
s.....

== D 1 ==

......
....T.
.....H
......
s.....

== L 5 ==

......
....T.
....H.
......
s.....

......
....T.
...H..
......
s.....

......
......
..HT..
......
s.....

......
......
.HT...
......
s.....

......
......
HT....
......
s.....

== R 2 ==

......
......
.H....  (H covers T)
......
s.....

......
......
.TH...
......
s.....

After simulating the rope, you can count up all of the positions the tail visited at least once. In this diagram, s
again marks the starting position (which the tail also visited) and # marks other positions the tail visited:

..##..
...##.
.####.
....#.
s###..

So, there are 13 positions the tail visited at least once.

Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at
least once?

--- Part Two ---

A rope snaps! Suddenly, the river is getting a lot closer than you remember. The bridge is still there, but some
of the ropes that broke are now whipping toward you as you fall through the air!

The ropes are moving too quickly to grab; you only have a few seconds to choose how to arch your body to avoid
being hit. Fortunately, your simulation can be extended to support longer ropes.

Rather than two knots, you now must simulate a rope consisting of ten knots. One knot is still the head of the
rope and moves according to the series of motions. Each knot further down the rope follows the knot in front of
it using the same rules as before.

Using the same series of motions as the above example, but with the knots marked H, 1, 2, ..., 9, the motions now
occur as follows:

== Initial State ==

......
......
......
......
H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)

== R 4 ==

......
......
......
......
1H....  (1 covers 2, 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
21H...  (2 covers 3, 4, 5, 6, 7, 8, 9, s)

......
......
......
......
321H..  (3 covers 4, 5, 6, 7, 8, 9, s)

......
......
......
......
4321H.  (4 covers 5, 6, 7, 8, 9, s)

== U 4 ==

......
......
......
....H.
4321..  (4 covers 5, 6, 7, 8, 9, s)

......
......
....H.
.4321.
5.....  (5 covers 6, 7, 8, 9, s)

......
....H.
....1.
.432..
5.....  (5 covers 6, 7, 8, 9, s)

....H.
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

== L 3 ==

...H..
....1.
..432.
.5....
6.....  (6 covers 7, 8, 9, s)

..H1..
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

.H1...
...2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

..1...
.H.2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== R 4 ==

..1...
..H2..
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

..1...
...H..  (H covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...1H.  (1 covers 2)
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21H
..43..
.5....
6.....  (6 covers 7, 8, 9, s)

== D 1 ==

......
...21.
..43.H
.5....
6.....  (6 covers 7, 8, 9, s)

== L 5 ==

......
...21.
..43H.
.5....
6.....  (6 covers 7, 8, 9, s)

......
...21.
..4H..  (H covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
..H1..  (H covers 4; 1 covers 3)
.5....
6.....  (6 covers 7, 8, 9, s)

......
...2..
.H13..  (1 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
H123..  (2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

== R 2 ==

......
......
.H23..  (H covers 1; 2 covers 4)
.5....
6.....  (6 covers 7, 8, 9, s)

......
......
.1H3..  (H covers 2, 4)
.5....
6.....  (6 covers 7, 8, 9, s)

Now, you need to keep track of the positions the new tail, 9, visits. In this example, the tail never moves, and so
it only visits 1 position. However, be careful: more types of motion are possible than before, so you might want
to visually compare your simulated rope to the one above.

Here's a larger example:

R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

These motions occur as follows (individual steps are not shown):

== Initial State ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........H..............  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== R 5 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........54321H.........  (5 covers 6, 7, 8, 9, s)
..........................
..........................
..........................
..........................
..........................

== U 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
................H.........
................1.........
................2.........
................3.........
...............54.........
..............6...........
.............7............
............8.............
...........9..............  (9 covers s)
..........................
..........................
..........................
..........................
..........................

== L 8 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
........H1234.............
............5.............
............6.............
............7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 3 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
.........2345.............
........1...6.............
........H...7.............
............8.............
............9.............
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== R 17 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
................987654321H
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

== D 10 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s.........98765
.........................4
.........................3
.........................2
.........................1
.........................H

== L 25 ==

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
H123456789................

== U 20 ==

H.........................
1.........................
2.........................
3.........................
4.........................
5.........................
6.........................
7.........................
8.........................
9.........................
..........................
..........................
..........................
..........................
..........................
...........s..............
..........................
..........................
..........................
..........................
..........................

Now, the tail (9) visits 36 positions (including s) at least once:

..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
..........................
#.........................
#.............###.........
#............#...#........
.#..........#.....#.......
..#..........#.....#......
...#........#.......#.....
....#......s.........#....
.....#..............#.....
......#............#......
.......#..........#.......
........#........#........
.........########.........

Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of
the rope visit at least once?
 */

#[cfg(test)]
mod test {

    fn part_1(input: &str) -> usize {
        let (mut tailpos, _) = input.trim().lines().fold(
            (Vec::<(i32, i32)>::new(), (0i32, 0i32, 0i32, 0i32)),
            |acc, line| {
                let (mut tailpos, (mut xh, mut yh, mut xt, mut yt)) = acc;
                let (dir, steps) = line.split_once(' ').unwrap();
                let steps: usize = steps.parse().unwrap();
                let (xstep, ystep): (i32, i32) = match dir {
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    "U" => (0, 1),
                    "D" => (0, -1),
                    _ => panic!("Invalid direction"),
                };
                for _ in 0..steps {
                    xh += xstep;
                    yh += ystep;
                    let (xd, yd) = (xh - xt, yh - yt);
                    if i32::max(i32::abs(xd), i32::abs(yd)) > 1 {
                        xt += i32::signum(xd);
                        yt += i32::signum(yd);
                    }
                    tailpos.push((xt, yt));
                }
                (tailpos, (xh, yh, xt, yt))
            },
        );
        tailpos.sort();
        tailpos.dedup();
        return tailpos.len();
    }

    fn part_2(input: &str) -> usize {
        let (mut tailpos, _) = input.trim().lines().fold(
            (Vec::<(i32, i32)>::new(), [(0i32, 0i32); 10]),
            |acc, line| {
                let (mut tailpos, mut pos) = acc;
                let (dir, steps) = line.split_once(' ').unwrap();
                let steps: usize = steps.parse().unwrap();
                let (xstep, ystep): (i32, i32) = match dir {
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    "U" => (0, 1),
                    "D" => (0, -1),
                    _ => panic!("Invalid direction"),
                };
                for _ in 0..steps {
                    pos[0].0 += xstep;
                    pos[0].1 += ystep;
                    for i in 1..10 {
                        let (xd, yd) = (pos[i - 1].0 - pos[i].0, pos[i - 1].1 - pos[i].1);
                        if i32::max(i32::abs(xd), i32::abs(yd)) > 1 {
                            pos[i].0 += i32::signum(xd);
                            pos[i].1 += i32::signum(yd);
                        }
                    }
                    tailpos.push(pos[9]);
                }
                (tailpos, pos)
            },
        );
        tailpos.sort();
        tailpos.dedup();
        return tailpos.len();
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 13);
        assert_eq!(part_1(INPUT), 5695);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 1);
        assert_eq!(part_2(EXAMPLE_2), 36);
        assert_eq!(part_2(INPUT), 2434);
    }

    const EXAMPLE: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const EXAMPLE_2: &str = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    const INPUT: &str = "
D 2
U 2
L 2
R 1
L 1
U 1
L 2
D 1
L 2
D 1
L 2
D 2
L 1
R 2
U 1
R 1
U 2
R 1
U 2
D 1
L 2
U 1
D 1
L 1
D 1
U 1
R 2
D 2
L 1
R 1
L 2
D 1
R 2
D 1
U 2
D 2
R 2
L 2
U 2
R 2
L 1
D 1
R 1
U 1
R 2
L 1
D 1
R 1
L 2
D 2
U 1
L 1
R 1
U 1
L 2
D 2
L 2
U 1
L 2
R 1
L 1
R 1
D 1
R 2
D 1
R 1
D 1
U 1
L 1
R 1
L 1
R 1
D 2
L 1
D 2
R 1
D 1
U 2
R 2
U 2
L 1
U 1
R 1
L 2
D 2
R 1
U 1
L 1
U 2
L 1
D 2
R 1
U 1
R 1
U 2
R 1
U 2
L 1
R 1
U 2
R 2
U 1
L 2
R 1
L 2
U 2
L 2
U 2
R 2
D 1
U 2
L 2
D 3
U 1
L 1
R 1
D 3
U 1
L 3
R 1
U 1
R 3
D 2
U 1
R 2
L 2
U 1
D 1
U 1
D 1
L 2
U 2
R 3
D 2
L 2
D 2
U 3
D 3
U 2
L 1
U 2
R 3
D 2
R 2
D 1
U 3
L 3
D 1
U 3
R 3
D 1
L 1
D 3
U 3
D 1
U 2
R 1
D 2
U 3
D 1
R 2
D 3
L 3
R 1
D 3
L 2
D 3
U 3
R 1
U 3
L 2
U 2
L 2
D 3
L 3
U 2
R 2
L 3
R 2
U 3
R 3
L 1
U 3
D 3
L 2
D 2
U 2
D 3
L 3
D 2
L 1
U 3
D 2
R 3
L 3
R 1
U 1
D 1
R 2
L 3
U 2
D 1
R 3
L 1
R 1
L 2
R 1
U 3
R 2
U 1
L 1
R 3
D 3
U 2
R 2
D 1
R 3
D 2
L 3
R 1
U 3
R 2
U 4
L 2
D 1
U 1
R 4
U 2
R 2
U 3
R 1
L 3
U 3
R 3
U 4
R 4
D 1
U 2
R 1
L 1
R 2
U 4
L 1
U 2
L 1
R 3
L 2
D 4
R 4
D 3
R 4
U 1
D 4
R 2
D 1
R 4
U 3
R 4
L 1
U 4
D 2
L 1
D 3
L 1
R 2
L 2
D 4
U 3
D 1
L 3
D 3
U 3
L 4
R 3
U 1
R 2
D 1
U 4
D 4
R 4
U 2
R 4
L 4
D 2
L 4
D 2
U 3
D 2
U 4
R 1
U 1
L 1
U 1
L 1
D 3
U 4
L 3
U 1
L 4
R 4
L 3
R 2
U 2
L 3
D 2
U 1
D 1
U 4
R 2
L 2
D 1
L 4
D 3
U 3
L 2
U 4
R 4
L 2
R 3
L 1
D 4
U 1
L 2
D 4
L 2
D 3
U 4
R 4
U 2
R 2
U 4
D 3
U 3
L 4
D 2
R 1
U 4
R 1
U 1
D 3
U 1
L 5
U 5
R 2
D 3
U 5
D 5
U 1
D 2
L 2
U 5
L 1
R 5
L 1
D 5
L 4
D 4
R 4
L 1
R 1
U 1
R 4
U 2
R 5
L 4
R 3
U 4
R 2
U 4
L 4
U 3
L 3
U 1
L 4
R 1
U 3
R 3
L 5
U 5
L 4
D 1
L 2
U 1
D 2
L 1
U 5
D 4
U 1
R 2
D 2
U 5
R 5
U 1
L 3
U 5
L 4
R 3
U 4
L 2
R 4
D 2
L 2
R 4
U 2
D 4
R 3
D 4
U 1
L 3
R 4
L 3
U 1
R 2
U 3
L 3
D 2
U 4
R 1
L 3
D 4
L 1
R 1
D 5
R 1
U 2
D 1
U 4
L 2
U 2
R 4
U 3
R 3
D 2
R 5
D 2
U 5
D 5
U 2
D 1
R 5
U 3
L 3
U 1
R 5
L 4
D 6
L 1
U 2
D 4
U 6
D 6
R 1
U 3
L 5
R 5
U 2
R 2
L 5
D 4
L 3
R 1
U 4
D 3
L 2
D 5
L 3
R 3
D 2
L 1
U 5
L 5
D 6
U 3
R 5
L 5
U 2
L 3
U 5
D 6
R 4
L 5
U 6
L 2
R 1
L 1
U 5
D 3
U 2
R 1
U 2
R 2
D 5
R 1
D 5
R 5
U 2
L 3
R 1
U 5
D 5
R 2
L 5
D 6
L 1
U 1
D 6
U 4
D 5
L 5
D 3
L 4
D 1
L 4
D 6
R 5
D 6
U 6
D 2
L 3
D 3
R 5
U 6
D 2
U 4
L 1
U 6
L 2
U 4
D 6
R 1
D 1
U 2
L 3
D 3
R 2
L 6
U 3
D 5
U 3
L 4
U 5
L 5
U 6
L 3
R 1
U 3
R 1
U 4
L 4
D 5
L 6
R 1
L 3
D 4
L 3
R 1
L 4
R 3
D 3
L 1
U 4
D 2
U 5
L 3
D 6
U 2
R 3
D 5
L 1
D 1
L 3
U 2
R 3
L 3
U 7
R 2
D 1
L 3
R 4
D 2
R 5
L 1
D 5
L 2
R 2
U 7
L 7
D 2
L 7
D 5
U 2
L 2
U 2
D 4
U 1
D 4
L 1
D 2
R 5
D 3
L 2
U 4
R 7
D 4
R 3
U 2
R 4
L 1
U 4
L 4
U 7
R 1
L 6
D 5
R 5
L 1
R 4
L 1
U 3
L 7
R 7
D 4
L 3
R 3
D 1
U 7
R 3
D 2
U 4
D 2
R 3
D 1
R 3
D 3
U 1
R 6
L 2
D 2
U 4
D 5
L 7
R 1
L 7
U 6
D 2
R 3
D 5
U 1
L 7
D 6
U 3
D 6
L 1
R 2
L 6
D 7
R 5
U 1
L 7
U 2
D 2
L 5
R 7
D 4
U 3
R 7
L 4
U 8
D 8
R 2
U 2
R 7
U 1
D 7
R 1
L 7
R 7
L 8
D 3
R 5
L 4
D 3
L 6
R 6
D 6
L 3
D 5
R 1
U 5
D 7
R 8
U 3
D 5
U 8
L 8
R 6
D 8
U 5
R 1
D 1
R 5
U 2
R 8
D 7
L 3
D 3
U 7
D 3
U 7
R 7
L 4
D 7
U 2
L 3
U 7
L 1
D 3
L 1
R 4
D 8
L 5
U 5
D 3
L 5
D 6
L 1
R 5
U 4
R 4
D 1
R 6
L 1
D 7
R 4
L 2
R 3
U 4
L 8
D 1
R 6
L 6
R 8
D 8
U 4
R 1
D 2
R 5
D 1
U 8
L 5
D 5
U 5
R 8
L 6
R 4
U 1
R 5
U 8
D 1
U 1
D 5
L 7
R 6
L 3
R 1
U 2
L 6
R 5
D 4
R 6
D 4
R 2
L 7
R 8
L 6
D 2
L 8
U 4
R 4
U 3
D 8
U 8
L 8
D 5
R 2
D 5
U 7
L 9
R 8
U 9
R 9
U 6
L 6
D 1
U 3
L 1
D 7
R 6
L 6
D 7
U 1
R 9
D 7
U 8
R 6
U 1
D 5
U 5
D 1
L 1
D 9
U 1
R 6
U 9
R 6
U 6
D 7
R 1
L 1
D 8
L 3
D 7
U 8
L 7
D 2
U 6
D 4
L 8
U 3
D 1
U 6
D 3
R 8
L 9
R 3
L 6
U 9
D 9
L 3
U 1
D 2
R 4
L 5
R 2
U 3
D 5
R 5
L 4
U 3
L 7
U 5
D 9
U 6
R 5
D 3
U 8
L 6
D 5
R 9
U 8
L 5
D 1
R 8
L 4
R 2
L 3
R 9
D 9
R 7
D 5
R 6
U 2
D 4
L 8
D 2
L 1
D 2
U 3
L 2
R 3
D 9
U 3
R 4
D 1
L 8
R 3
U 4
L 4
R 3
L 5
D 4
R 2
D 4
R 4
U 4
R 7
L 6
R 9
D 1
R 8
L 5
U 4
D 10
L 6
D 3
R 2
U 1
D 5
R 2
U 6
L 9
U 7
L 10
U 4
L 8
D 3
U 7
D 7
U 9
L 2
R 6
U 4
R 9
L 5
U 3
L 4
R 1
D 8
U 10
D 10
R 3
U 3
D 2
L 5
U 3
D 4
U 1
R 9
L 3
D 8
L 5
U 8
R 3
U 10
L 10
R 4
D 5
U 9
R 8
L 5
U 9
L 6
U 10
D 8
U 2
L 8
D 7
U 2
D 5
U 8
L 2
D 3
U 8
R 8
D 10
R 3
L 3
U 4
R 3
U 4
L 6
U 4
R 3
D 9
L 9
U 5
D 1
R 3
D 2
L 9
U 9
R 9
U 6
R 7
L 8
U 2
L 4
D 10
L 4
D 9
L 9
R 10
L 3
R 7
U 2
L 3
R 1
D 4
L 2
R 10
D 5
L 2
D 2
L 4
U 6
D 9
R 6
U 9
D 4
L 6
U 8
D 1
R 1
L 10
U 4
R 6
U 11
R 8
U 1
D 5
R 7
D 8
L 5
R 6
U 5
R 1
U 7
L 4
D 6
U 8
R 10
D 1
L 8
R 3
U 7
R 2
U 9
D 9
R 9
L 3
R 4
U 10
L 7
R 1
L 5
D 11
R 4
U 10
R 10
L 3
D 2
L 4
U 9
L 1
D 10
L 7
R 11
D 5
U 3
D 10
U 5
L 2
R 9
D 10
U 1
R 10
U 1
L 11
R 10
L 8
D 10
L 5
R 2
L 3
U 4
L 10
R 3
D 5
R 8
L 2
U 1
D 7
U 7
L 1
U 11
D 2
L 2
D 11
L 6
R 9
U 10
R 7
L 11
R 7
D 1
R 3
U 6
R 7
D 8
U 3
L 1
D 11
U 7
R 1
L 7
R 2
L 7
D 3
U 10
D 9
U 10
R 5
D 4
U 7
L 11
U 1
L 12
R 10
U 7
R 3
L 2
U 11
R 6
U 8
D 2
R 2
L 8
U 7
D 10
U 3
D 1
U 8
R 2
D 9
L 8
R 10
L 1
U 8
R 11
U 8
R 12
D 10
L 5
U 6
L 11
D 12
L 5
U 4
L 8
U 11
D 2
U 4
L 6
R 1
L 4
U 2
L 5
R 2
D 7
L 12
R 4
L 1
D 7
L 11
U 3
D 9
R 12
U 10
D 9
U 10
L 6
R 6
D 10
R 10
D 5
U 9
R 2
D 4
L 9
U 2
R 5
L 8
R 1
D 9
L 7
D 11
R 3
L 8
U 3
D 7
U 3
R 8
U 7
L 1
U 10
D 8
R 11
U 10
D 11
U 8
R 12
U 9
D 12
U 4
D 11
R 8
D 1
L 4
R 9
L 7
U 8
R 6
D 5
R 8
D 2
R 6
L 4
D 7
L 3
U 11
R 2
L 10
D 13
R 2
D 10
U 9
D 5
U 5
D 5
L 11
U 3
R 5
U 7
L 6
D 4
L 3
R 2
D 9
U 7
D 9
U 1
D 8
L 11
D 7
U 7
R 3
L 6
D 6
R 5
L 6
R 3
U 10
L 13
U 9
R 11
L 13
R 9
D 3
U 3
L 12
U 4
L 3
D 12
R 13
D 7
U 6
L 11
U 4
L 2
D 7
L 8
R 10
U 13
R 9
U 8
R 11
U 8
R 7
U 11
L 8
D 3
L 4
U 6
D 12
R 13
U 10
L 4
R 12
U 13
L 11
R 10
L 8
U 6
L 1
D 11
U 12
L 8
U 2
L 6
U 2
L 1
U 3
D 5
U 2
D 10
U 11
L 9
D 9
R 7
L 3
U 3
D 2
U 2
R 10
D 1
R 5
L 1
D 3
R 8
U 4
R 1
D 13
L 12
U 1
D 13
R 4
D 3
R 8
D 2
U 3
D 8
L 3
U 9
R 4
D 8
U 9
D 3
L 8
D 11
L 8
U 1
D 13
U 14
D 10
U 9
R 4
D 14
L 10
D 10
U 6
D 7
R 8
U 5
L 8
R 9
D 12
L 9
D 8
R 13
L 7
U 11
R 8
D 5
U 14
D 5
U 9
L 7
D 12
R 2
L 14
U 1
R 9
U 10
R 4
U 4
R 14
D 10
L 3
R 4
L 13
U 3
R 6
L 2
U 6
R 2
D 10
U 4
L 10
U 8
L 6
D 2
R 1
D 5
U 14
L 4
D 5
L 5
U 10
D 5
R 13
D 14
U 14
R 4
D 13
U 10
R 13
D 12
R 10
L 12
U 8
D 10
U 12
D 5
L 11
D 13
R 8
D 10
R 13
L 13
D 1
U 10
L 4
U 14
D 12
U 4
D 5
R 10
L 8
R 13
D 13
U 5
D 4
L 1
D 9
L 14
U 14
R 3
U 6
R 14
L 4
R 7
D 3
R 8
D 5
L 2
U 5
D 8
U 3
R 7
D 13
U 14
R 11
U 6
L 15
R 4
U 12
R 1
L 8
U 15
R 5
L 2
D 1
R 12
D 1
R 12
D 1
L 1
R 9
U 11
D 2
U 12
D 3
U 6
D 12
U 11
R 6
D 11
L 5
R 3
D 1
U 7
L 10
D 4
L 7
R 2
U 8
D 11
L 12
R 6
L 7
D 13
U 11
R 5
D 1
L 11
D 2
U 15
L 11
R 3
L 9
D 12
U 8
D 9
L 14
D 10
L 12
R 13
D 3
L 9
D 13
R 8
D 15
U 14
R 9
U 12
R 2
D 3
U 5
L 3
R 7
U 15
R 10
L 6
R 6
D 12
R 14
L 7
U 8
L 11
R 13
U 8
L 8
R 5
D 3
U 8
D 4
L 1
D 11
U 15
L 14
R 8
L 1
R 13
L 2
R 13
U 9
D 12
L 14
R 13
L 5
R 7
D 13
U 15
R 2
L 12
D 12
R 13
U 2
D 4
R 14
L 1
U 12
D 3
U 6
R 6
L 6
U 9
L 16
U 16
D 8
R 8
U 1
D 14
R 13
D 3
U 14
R 7
U 7
R 7
D 12
U 1
D 2
U 2
R 7
D 11
U 4
R 13
U 2
L 16
U 12
L 15
D 10
U 11
L 1
D 1
L 3
D 15
L 12
R 2
D 12
U 16
R 16
U 11
D 15
R 11
U 13
L 3
R 15
D 16
R 9
D 1
L 3
U 9
D 12
U 11
L 13
R 10
D 4
U 12
L 2
R 8
L 1
U 9
L 1
D 7
R 14
U 12
L 9
R 5
D 7
R 10
D 14
L 2
U 2
L 4
D 15
U 7
D 9
L 4
R 1
L 8
D 14
U 9
D 1
R 15
U 9
R 10
D 11
L 14
R 12
L 12
U 13
R 12
L 1
R 10
D 14
R 3
U 5
D 10
L 13
U 11
R 16
L 14
U 6
D 5
R 16
D 7
L 16
U 12
R 14
U 7
R 6
U 17
R 3
U 15
R 17
U 3
R 4
U 6
D 13
U 7
R 12
U 10
R 7
L 13
U 16
D 13
U 2
L 13
R 2
D 1
U 10
R 6
D 11
R 10
D 6
R 2
L 1
R 1
L 5
D 16
U 1
L 13
U 3
R 9
L 10
D 12
R 9
U 17
R 10
U 1
D 13
L 13
R 6
L 5
D 5
R 15
D 15
L 7
U 11
D 1
U 11
L 17
R 5
L 11
R 15
U 16
D 2
L 5
D 9
R 13
D 14
U 6
L 10
U 15
D 15
R 5
D 4
L 13
R 11
D 15
U 13
R 8
U 6
R 8
U 3
R 12
U 11
L 6
U 9
D 6
L 1
U 4
L 6
R 4
D 2
L 6
U 11
L 1
U 10
R 2
U 6
R 1
U 7
R 16
L 12
U 9
D 9
U 6
R 13
U 8
L 6
U 2
R 17
L 6
U 5
D 14
R 15
D 13
L 5
U 18
R 9
L 10
R 9
D 6
L 8
U 3
D 8
R 17
L 4
R 9
L 13
U 3
R 12
D 4
R 11
U 13
L 15
R 13
D 18
R 8
L 7
R 10
D 11
U 8
R 4
L 7
U 15
L 3
D 15
R 3
U 6
L 17
D 17
L 12
D 16
R 14
L 1
R 7
D 15
U 2
L 5
U 16
D 7
L 11
U 1
R 2
D 4
L 13
U 17
R 5
D 17
R 14
L 1
D 11
U 11
R 8
U 12
D 12
U 17
R 15
L 4
D 7
U 8
R 4
L 14
U 11
L 3
R 1
L 14
U 3
L 3
D 15
L 14
D 7
R 2
L 6
U 6
R 12
U 17
D 1
R 15
D 1
U 14
D 6
U 4
R 4
L 3
U 14
L 1
R 4
U 13
D 18
L 11
U 4
D 9
R 10
U 17
R 8
D 12
R 9
L 5
D 6
L 9
R 9
L 17
D 9
R 2
L 4
R 8
D 8
R 8
D 13
R 12
L 13
U 8
D 8
L 16
D 15
U 11
L 8
R 7
D 2
R 6
U 19
D 4
U 7
L 17
R 9
U 18
D 4
R 19
L 14
U 8
D 19
L 12
R 14
U 13
D 3
L 8
D 4
R 9
U 18
D 1
U 2
L 16
U 2
D 12
U 5
L 4
D 6
U 14
R 1
L 7
D 7
L 19
D 5
U 16
D 15
R 14
U 19
D 19
L 6
R 3
L 14
R 16
L 1
U 4
R 19
U 2
L 3
D 19
L 17
U 13
D 5
L 10
U 18
R 13
D 6
L 17
D 9
R 10
D 8
R 19
L 6
D 7
R 18
U 8
L 6
U 18
L 18
D 16
R 8
L 10
R 6
D 1
L 6
U 8
L 19
D 18
U 3
D 17
U 9
R 13
D 18
R 2
L 10
D 17
L 17
U 8
L 13
D 5
L 1
U 10
L 3
R 3
D 12
U 11
L 15
D 15
";
}
