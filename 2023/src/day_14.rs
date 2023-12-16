/*
 --- Day 14: Parabolic Reflector Dish ---

You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the
side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a
parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is
meant to focus light, all it's doing right now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go
where it's pointing and use the light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and
pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes.
Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform
controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that
lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the
cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your
puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....

Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....

You notice that the support beams along the north side of the platform are damaged; to ensure the platform
doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the
south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.)
So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1

The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support
beams?

 */

#[cfg(test)]
mod test {
    fn part_1(input: &str) -> usize {
        let input = input.trim();
        let cols = input.find('\n').unwrap();
        let rows = input.lines().count();
        input
            .lines()
            .enumerate()
            .fold(
                (0usize, vec![0usize; cols]),
                |(sum, mut prev), (ri, line)| {
                    let load: usize = line
                        .chars()
                        .enumerate()
                        .map(|(ci, c)| match c {
                            'O' => {
                                let out = rows - prev[ci];
                                prev[ci] += 1;
                                out
                            }
                            '#' => {
                                prev[ci] = ri + 1;
                                0usize
                            }
                            _ => 0usize,
                        })
                        .sum();
                    (sum + load, prev)
                },
            )
            .0
    }

    fn part_2(input: &str) -> usize {
        let (rows, cols, mut input) = {
            let input = input.trim();
            let cols = input.find('\n').unwrap();
            let rows = input.lines().count();
            let mut bytes: Vec<u8> = Vec::with_capacity(input.len());
            bytes.extend(input.lines().map(|l| l.as_bytes().iter()).flatten());
            (rows, cols, bytes)
        };
        let mut stops: Vec<usize> = Vec::with_capacity(usize::max(rows, cols));
        let mut history: Vec<u8> = Vec::with_capacity(input.len() * 100);
        for iteration in 0..1000000000 {
            // North
            stops.clear();
            stops.extend(0..cols);
            for i in 0..input.len() {
                match input[i] {
                    b'O' => {
                        let ci = i % cols;
                        let other = input[stops[ci]];
                        input[stops[ci]] = std::mem::replace(&mut input[i], other);
                        stops[ci] += cols;
                    }
                    b'#' => {
                        let ci = i % cols;
                        stops[ci] = i + cols;
                    }
                    _ => {}
                }
            }
            // West
            stops.clear();
            stops.extend((0..rows).map(|r| r * cols));
            for i in 0..input.len() {
                match input[i] {
                    b'O' => {
                        let ri = i / cols;
                        let other = input[stops[ri]];
                        input[stops[ri]] = std::mem::replace(&mut input[i], other);
                        stops[ri] += 1;
                    }
                    b'#' => {
                        let ri = i / cols;
                        stops[ri] = i + 1;
                    }
                    _ => {}
                }
            }
            // South
            stops.clear();
            stops.extend((0..cols).map(|c| c + cols * (rows - 1)));
            for i in (0..input.len()).rev() {
                match input[i] {
                    b'O' => {
                        let ci = i % cols;
                        let other = input[stops[ci]];
                        input[stops[ci]] = std::mem::replace(&mut input[i], other);
                        if stops[ci] > cols {
                            stops[ci] -= cols;
                        }
                    }
                    b'#' => {
                        let ci = i % cols;
                        if i > cols {
                            stops[ci] = i - cols;
                        }
                    }
                    _ => {}
                }
            }
            // East
            stops.clear();
            stops.extend((0..rows).map(|r| r * cols + cols - 1));
            for i in (0..input.len()).rev() {
                match input[i] {
                    b'O' => {
                        let ri = i / cols;
                        let other = input[stops[ri]];
                        input[stops[ri]] = std::mem::replace(&mut input[i], other);
                        if stops[ri] > 1 {
                            stops[ri] -= 1;
                        }
                    }
                    b'#' => {
                        let ri = i / cols;
                        if i > 1 {
                            stops[ri] = i - 1;
                        }
                    }
                    _ => {}
                }
            }
            match history
                .chunks(input.len())
                .enumerate()
                .find(|(_i, prev)| prev == &input)
            {
                Some((start, _prev)) => {
                    // We detected a cycle that means the final state
                    // after 1 billion cycles is already in the
                    // history and can be accessed through modula
                    // arithmetic.
                    let begin =
                        input.len() * (start - 1 + (1000000000 - start) % (iteration - start));
                    let end = begin + input.len();
                    let state = &history[begin..end];
                    return state
                        .iter()
                        .enumerate()
                        .map(|(i, c)| {
                            if *c == b'O' {
                                rows - (i / cols)
                            } else {
                                0usize
                            }
                        })
                        .sum();
                }
                None => {}
            }
            history.extend(input.iter());
        }
        panic!("Expected to detect a cycle and return early.");
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 136);
        assert_eq!(part_1(INPUT), 110779);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 64);
        assert_eq!(part_2(INPUT), 86069);
    }

    const EXAMPLE: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    const INPUT: &str = "
O........O..#...O...#OO..O.##..#.O....O#..#..O..#.O....#.....O.O.O..#OO#.....O#O..#.#.#O....O...#.O.
...#....O.O...#..O....#O.....#..#O.O.....O.......#.O...O.O.#O.#..O#.#O..O##...#.OO.##...#O#.O..O###.
O.......O...OO.....O...#..O#.##......O.O.O...O..##..#.O...O...O###O.#.OO.#....#O.........O.##...O#..
.##.O.........O###...#.#......#.#...O.....OO#..#......#..OO.O.OO.O#OOO....O..............O.......O#O
#......#OO#O...O#.O..#.#O..........O.............OOO.##..#....O..OO......OO.....OO.OO...O.......###.
.O...#O#......OOO.....#OO.O.OO..O....#.#..#.O.......#O..O....OO....O..O.#.O.####.O...O..O....OOO#.OO
...#.O......#...##O.......O.#.#..##......#O.O.OO.OO.....#OO#....OOO.....O...#..OOO..O.....O.O.#..O##
##.....O##.O#.O###O#O..........#...O.O.O.#..OO....O#O.##.O......O.OO.O..#......#...#.#O..O#....#...O
.OO........#..O.#O.O..O.....O...#O..OO.#O...O.#....O.O....#O.#O..O......O....#..##.O....##O..#O.O.O.
.O.O.OO..........OOO##O....#.#.##OO#O.#...O...#.#...OO.#O.#..O.O#..O....OO.#.OO.#...####.O.##..O.O##
.....OO#.#O......#.#O#.#...##...O.O.#OO#.....O#.O#.OO.O#O..O##.O#..O.OO#.O...O.#...........O.O.#....
.........O.......OO...#..O.#...O.OO#.O...OO#...O.O#....O......O..O.O.......#.#...#.#.#...#....O.....
..O...O....OO...#..OO.O...#O..O#O#.###..O##OO.....OO.......O...#..#O.....##...OOO##.#....O...#.....O
OO..OO...##.O..#.O.OO..O..O..#....O..O#.#.....O...#...O.............O.#...#O..O..OO#.OO...##.#...OO.
..#.O..#O.......OO..#.#.#.#O..O..O#.O...OO.O.....##....#.#O.O#..O.#....O#...OO.....OO.OO....O.#..O..
.#..O...OOO.......O.....O.####OO.OO..O..#...#O....###.#OOO.O.....#...##.O..O.#OO....##....O.#.#O...#
#O#.O.O#O.O....#.O#.#.....#O......O..O........#O.O.#O..O.#O........O.#.#O.O....O.#O##.......O.O...##
...O#O.##.O##.....#.....O...O..OO..#.....O..OO.O...#.OOO.O...O#.O...O........#O#..O.O#O.O..O..OOO#.#
O..O.....O....O..O...OO.O..O...OO##..#.##....#O......OOO......O#.O.#.....##OO...O..#..#.....#.......
....#O#.......#..#..O.O.......#..O.....OO...............##..#....O...O...O.O.####...#.O##..O.#.#...O
...O#..O#.OO.O#OOO.....O..........##O#.##.O.O....#..OO....#....O#....#...O#........O.......O.O.#OO..
#......#.O.............#.OO.##.###...#..#.......OO.#..#...O..#.#.....#....O..#.....O....#..O.......O
.O...O.#O.OO#.#...O.....O.O...O...O....##....#.O..#.#O...OOO.O.#.O.#.O#..O.#..O#O#.##...#..O.#O.O...
O#.O...O#...O.O..O.O#.O##..#...#...O#.O.#.#......#.........#O.#.OO.O..#.#...#.OO.OO##.......O.O.###.
O..O..O..O.#..#......##.#O#....O.OO...OO.........O...O...O..O#.O#OO.....#..##...#.#O.#...##.#.#.#...
O#O....O..O..#..#.#.....#.......OO..O....O....O.OO...OOO#..O........O.....O..#....O#..O#....OO.#O.#O
...O...O.O..O..##......#...#.O##..O..OO..#.#O#..O.#OO.#..#.O....O....#OOO.O......#O...#.#..#..O..O.O
...O..O#.O.O.#O..O.#.O...OOO.#.....#....##O.....O..O......OOO.#.O..O.#..O#....OO#.....#O.O.....#.#..
....#O..##O..O.O.O...#..OO.........#.###.O..#.O.O.......#.O.O#O..#....#.#..#.......#........#.......
.O.O#...O.OO......O.OO..#........#...#O.....O#O...O.#.O.#O.#.#.O#..O#O#.#..###....O......O...#.O..#.
.O.O.....#..O#..O.O..#..#...#..O....#..#O#...O.....#....OO.....#.#......#.#OOO..O.O..O....##........
..#...#O#.O...#O.###OO.O.#O...#...O....##..#.O...#.#...O....#.O#..O..O......O#..O.#O..O......#......
O.....O..##.....OO#.O.O.O..OO.OO..O..#......O.#....O..O##.OO..OO#.O#O#..OO.O....OOO..O..OO..OOO.O#..
.#....O........OO#O..O...#..O...O.......O.#O...O.OO...O..O#....O#OO.##.O.#.....O.O..OO......#O...O#.
..O.#OO.O..##...O.#...#..O...OO.O..O..O..O#..#.OO..OO.....O...#...#....#.O...OO....##..O........O...
OOO...O#......#.#....O.O.#.#.......#.O.....#.O..OOO#O.#......##O.O..#O#.#...O.O...............OO..O#
..#O#O.........OO....O..##.O.......O#..O....O....O..........O.......OOO.O....#..#O#..##..#OO...#..OO
..#..#...O...#O...##O.OO........#..#...OO..O.O#..O....O..#.O.#..OOO...O.O.##......O#O...........#...
...O..#O......#.....O.#.#...##...###.....#.O#.#......#.#O#O..O...##O.....OO.O.....O....O#O.#OO......
.O.#...OO#.#O..O.....O...O.......#O.#O....O...O...O.O....O.O.....OOO#O...#.OO....#..O#....O....O...O
...O.#.#.O#......OOO..............#.O....#.O..O........O...O..#OOOO.....O.#....O......##..O..O.#.O.#
O#....##.#......O.......O..#..OOO..O.....#O...........O.O.....O...O.#O.O.....#..#..#.....OO#O.#...OO
O..OO.O#....#O#.O.#.....OOO.....O.O.O.O...OOO.O.....#...##..#...O#...##...O....O.O.OO..#....O.O.....
O....#....O.........#..#..O....#OO#....O..#.....#OO..#..O.#..O..#..#...#O.#O...........O...........#
.O.#..O.#..O.#.....O..O#.#OO.#OOOOOO..O.O#..OOO....O.O.O#...O...O....O...O.##.....O..O.O.OO.OO#....O
O#.O..OO..O...#.....O...O.O.O...OO.OO..O..O............#.O....O..O.........OO.#......#O...#O..O..O..
.#O.....#.OO....O.#OO.#O....OO#OO.#.OO....O#..OO#..#O....##.#..O.O..#OO..O...O#O.O#......O...#.##O.#
........#O.....O.OO....O.#O...#.O.O..........O#..O...##..#..O..OO..####O........#O#..O...O..O##....#
O...##.O....OO....O#........O..O....O#.##O..O#.#.O....#O...#....O.#...O.OO...OO..O...........#.#....
...O#O...O....#...O..#.#..#.......O.OO#OO.....O....O..#.#.....#..O#......O.O.OO#.##....O.....O......
..#.#...#O#..O.#.O.#.#.#.O#.#OO..O..#.#.#O.O...O.......#...#O..#.#.O#O#..O......OO##..OO.....#O..#..
.......O.OO....#...O.OOO##...#O..O.....#.#.#..##O.O.............#.....#..#O....#....O##....O......#O
...#.O.#...##.O.....#.#O..O#.O.#......O.O#..#.##OO.......O........#.OO.OO.#.#OO..#O....O#..#..#...OO
#O.....#......O..##.#O..#.O..#..##....#.#.O....#....O#.O.O.#....#O..##.O....O..O..OO.#.O.#O....O..#O
...OO.O.......#.##.O#....O.O..#.O.O..O..O...#..O.OOOO.O#.O.O..O#O#..#....#.....O.....#.O....#..O.#.O
#......#OOO....#......O##.....O..#....#......#.O...O#..O.O#...#O.........#..##..O.OO....O...#...#O..
#O....O...O...O..#.....O#.....O.#O...O.O..##.O...O.#...#..O.OOOOO.O#..#.OO##.O.O.##...OO..O..O#OO.#.
...#.........#..#O.#O..###.....OO.#O....#..O.....#....OO.O.####.O.O.....#O..#..#..O...O.#..#.#.O.O#O
....O.#........OO..#...............##.....#.#.#O#.......#OO.....OOO...O..O..O.OO....##..OO#.O.OO....
..O.O.O.O##...O##.#.O#...O.OOO#.O##..O..O.#O#....#...#.......O.O...OO.O.OO....O...#.........O.#...O.
..O......O.OO.O.#.....O...#.O...O#.#...O.#..O#...O.O#.O#........#.O.O#..#..#........OO#.##.#.#O....O
..#......####.....O.O.O......O#...O.O.....O.O..#......O.........#O.O...#..####..#....#.O.#..#..#..#.
.......###OOO....O#OO.O##....#...O....#.#..O.....#......#OO#O.O..O....O.#.OO.....#.#................
#....O.#OO....O..O..O.....O#...#...O#........O#.O.#.O#O#.#.O......O.#O.#.O..O.O..#.........O.....#..
.O.O.##...O..OO...O.....O.O#..##...O..O.#..##O..O#....#.......O.O.#.......#....O#O..##....OO...##O.O
..#.....#...###........O.O..#O.......#..#..OO#O##..#...O.#...#....O...O..#..##.......#....O#.O.#O..#
#.#.OO.##.OO#....##..#..O.O...O...O....O#..O#..#O#O.#.....O#..#.........O#..O##.#..O....OO#..#O..O..
O..........O..#..#..O...#.O#...#OO..##.O..OO..OO....##.O.O....O...O.O....O.....O.....#O...O...O....O
O..OO.O..O.....#.OOO#.......O....O..O......#.O.##.OO...O...O.O.O#O.#..#.#O......#OOO..OO.O...##O..#.
...#O.O.#O#..#...#.....O....#.......#...O.####..#....#..#..O##.O##...O..O.O.....#.#.#....O#.O......O
..OO....#O..O............OOO.O....#....OOO.OO....#.....O##O.O.#O.O...#.#.O.....#..O........OO.#O..O.
....OO.#.OO#O..#.OO##OO#.#..#..O..O...O...#O.O.O.....OO.#.O....O.O.O.#O#O#....O##........#..##O##...
#.#..O.#.O......O...##....O.O.O#O#...O.O.O...O...#..OO...........#.#.....#..........#O#....O.OO#O.##
#...#......#O...........#O.OOO#OO#..#..O....##......#...O#.OO...#....O..O#...#.#.#O...#..O.........#
##O..........OO.#..#..O.......#O.#..O.#.#..O...O#O..O....O.OO...O..O....O.....#....O#O.#...O##.....#
OO..O..#O.#OOO.#.......#.....#OO....O........O.#..#.O...#........OO..#..#.....#O...#....#....O....OO
.OO....O......#O#..O.OO..OO.O..#......................O...#...#.O........#..##.#..O.#O#.#.O#O..#....
..#...O.#....#.O.....O.O...#....O#......OO.......OO.##....OO......#....##OO....O.........O.OO.#..O..
..O.O...O#...##....O##O.....#..#.......##...#.....#O#....O........O..#O.O.#........O....O.O..O#...O.
.O.......#..#.O.OO...O.....OO.#..........O...O......O..O.##.#.....O.O.........OO#.......O......O..OO
..O........O.#O..#..OO...O.....O.##O....OO#.OO#.O.O.#O...##......O..O.....O..OOO#.O##....O.O...##O..
.O#....#O.##.#..O#.O..#.O.###.#....#O#O#.OOO#.#......O....#.O#......#.........###O#.O.OO....O..#O##.
.O..O.........#.O#O..#.......OOO#OOOO......#..#..#.O##.O#...##.....O...#.#...#.#..O...O#.#.....#....
O.....#O....O.#.#...OOO##..#.....OO...O#..#..O#.#..O..O...#...#.O#....#..O.....O..O.O#.#.O....O...O.
....O.....O..O...OO......OO.#.#..O...O#...#.#O.................O..O.O.....##..#.##.#O..O.#.#..#O##OO
...##.O..#....#O#.#..O..#....#O.O.#......O.O..O#OOOOOO....O.#..#..O..#..........O#....#...#O#..OO.O.
#.#......O.#O#.....O.O...O.#.OOO..#.O.....#O.#.........O#.#.OO.O.....O..##..#O.OO..O...O....O#.##O.#
.O.........#O#..O...##....#.#.#O..O..O.#..O..O#OOO..O.....##.........O#.#O.#...#O....#..O.....##.#O.
#.O......O.O..O...O.#O.#..O.#..#O###..#..O..O....OO....O.#......O..#.OO.O#.#O....OO#O#...OOO.#.....O
..O.O.#.#O#.......#..O.......#.O..OO...OO...O#..#....OOO....OOO..#.O#.O.O.....#.O........O.#........
..O.#.......O..#.#.#.#O........#O....#.#O##.....O.O#.##O.#..OO........#...O......O...O....#...#.#..#
.#...##..O.....#O#O..O.#.#.O...O.O#O....#........#....O...OO.#OO.....O#..O#O........#.....O.....O..#
....O.O..O...OO.......O..##..##.O....O.............O.O.O....O....#......O..O........OO.O.#.O....O...
O..#......O.#.O.O.#.OOO.......O.O.#.O...OO......##...O....O.O#O..O.O.....O..#..#...O.O###O...O#...##
#.#..#..##.OO#...O..#O#..O..O..O.O...#.#..OO.OO.....O.O.O#O......O.O.O......##O..O.#....#.#.O......#
.....#OO..OOO..O...O..O..O............O#.#.#O#..O#.#......#.O.....O..........#..O....O..O.....#.O#..
#..#O.O...#.O.O.........O.O..#......OO...#...#..#.##O...#.O...........O.#.#OO.##..O..O...O.#OO...#OO
..##.##..O....OO...OOO.........#.O.......#.O#..O#....OOO.#....O#.#....OO...#.....#.O...O.........#.O
OO##..O#.OO##.......O...OOO.#..O........#O.O.#.O..O..O..#.....O.O..OO...O##....O.#..OO#O....O..##...
..O..O..#.O..........O..O..OO.....#....O#...O#........##.O.#....OO.O.....O...O####...#O.#O.O....#O..";
}
