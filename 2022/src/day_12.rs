/*
--- Day 12: Hill Climbing Algorithm ---

You try contacting the Elves using your handheld device, but the river you're following must be too low to get a
decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local
area from above broken into a grid; the elevation of each square of the grid is given by a single lowercase
letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

Also included on the heightmap are marks for your current position (S) and the location that should get the best
signal (E). Your current position (S) has elevation a, and the location that should get the best signal (E) has
elevation z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can
move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation
of the destination square can be at most one higher than the elevation of your current square; that is, if your
current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation
of the destination square can be much lower than the elevation of your current square.)

For example:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

Here, you start in the top-left corner; your goal is near the middle. You could start by moving down or right, but
eventually you'll need to head toward the e at the bottom. From there, you can spiral around to the goal:

v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^

In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<),
or right (>). The location that should get the best signal is still E, and . marks unvisited squares.

This path reaches the goal in 31 steps, the fewest possible.

What is the fewest steps required to move from your current position to the location that should get the best
signal?

--- Part Two ---

As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The beginning isn't
very scenic, though; perhaps you can find a better starting point.

To maximize exercise while hiking, the trail should start as low as possible: elevation a. The goal is still the
square marked E. However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll
need to find the shortest path from any square at elevation a to the square marked E.

Again consider the example from above:

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at
elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

...v<<<<
...vv<<^
...v>E^^
.>v>>>^^
>^>>>>>^

This path reaches the goal in only 29 steps, the fewest possible.

What is the fewest steps required to move starting from any square with elevation a to the location that should
get the best signal?

 */

#[cfg(test)]
mod test {
    use std::collections::BinaryHeap;

    use itertools::Itertools;

    fn parse_grid(input: &str) -> (Vec<u8>, usize, usize, usize, usize) {
        let input = input.trim();
        let cols = input.find('\n').unwrap();
        let rows = input.lines().count();
        let input: String = input.lines().flat_map(|l| l.chars()).collect();
        let mut grid = input
            .chars()
            .filter_map(|c| match LETTERS.find(c) {
                Some(pos) => Some(pos as u8),
                None => None,
            })
            .collect_vec();
        let start = grid.iter().position(|h| *h == 0).unwrap();
        let end = grid.iter().position(|h| *h == 27).unwrap();
        const LETTERS: &str = "SabcdefghijklmnopqrstuvwxyzE";
        grid[start] = 1;
        grid[end] = 26;
        (grid, rows, cols, start, end)
    }

    fn part_1(input: &str) -> usize {
        let (grid, rows, cols, start, end) = parse_grid(input);
        let mut distances = vec![usize::MAX; grid.len()];
        let mut prev = vec![usize::MAX; grid.len()];
        let mut heap = BinaryHeap::new();
        heap.push((usize::MAX, usize::MAX, start));
        while let Some((cost, from, pos)) = heap.pop() {
            let dist = usize::MAX - cost;
            if distances[pos] <= dist {
                continue;
            }
            distances[pos] = dist;
            prev[pos] = from;
            let (x, y) = (pos % cols, pos / cols);
            heap.extend(
                [
                    if x > 0 { Some(pos - 1) } else { None },
                    if x < cols - 1 { Some(pos + 1) } else { None },
                    if y > 0 { Some(pos - cols) } else { None },
                    if y < rows - 1 { Some(pos + cols) } else { None },
                ]
                .iter()
                .filter_map(|n| match n {
                    Some(n) => {
                        if *n == from || grid[*n] > 1 + grid[pos] {
                            None
                        } else {
                            Some((cost - 1, pos, *n))
                        }
                    }
                    None => None,
                }),
            );
        }
        return distances[end];
    }

    fn part_2(input: &str) -> usize {
        let (grid, rows, cols, _start, end) = parse_grid(input);
        let mut distances = vec![usize::MAX; grid.len()];
        let mut prev = vec![usize::MAX; grid.len()];
        let mut heap = BinaryHeap::new();
        heap.push((usize::MAX, usize::MAX, end));
        while let Some((cost, from, pos)) = heap.pop() {
            let dist = usize::MAX - cost;
            if distances[pos] <= dist {
                continue;
            }
            distances[pos] = dist;
            prev[pos] = from;
            let (x, y) = (pos % cols, pos / cols);
            heap.extend(
                [
                    if x > 0 { Some(pos - 1) } else { None },
                    if x < cols - 1 { Some(pos + 1) } else { None },
                    if y > 0 { Some(pos - cols) } else { None },
                    if y < rows - 1 { Some(pos + cols) } else { None },
                ]
                .iter()
                .filter_map(|n| match n {
                    Some(n) => {
                        if *n == from || grid[*n] < grid[pos] - 1 {
                            None
                        } else {
                            Some((cost - 1, pos, *n))
                        }
                    }
                    None => None,
                }),
            );
        }
        let newstart = (0..grid.len())
            .filter(|i| grid[*i] == 1)
            .min_by_key(|i| distances[*i])
            .unwrap();
        return distances[newstart];
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 31);
        assert_eq!(part_1(INPUT), 504);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 29);
        assert_eq!(part_2(INPUT), 500);
    }

    const EXAMPLE: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    const INPUT: &str = "
abaaaaaaaaccccccccccccccccccaaaaaccccaaaaaaccccccccccccccccccccccaaaaaaaaaacccccccccccccccccccccccccccccccaaaaaccccccccccccccccccccccccccccccccccccccccccaaaaaa
abaaaaaaaacccccccccccccccccccaaaaaccccaaaacccccaaaacccccccccccccccaaaaaaaaaacccccccccccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccaaaa
abccaaaaaaccccccccccccccccccaaaaaaccccaaaaccccaaaaaccccccccccaaaaaaaaaaaaaaacccccccccccccccccccccccccccccaaaacccccccccccccccccccccccccccccaaaccccccccccccccaaaa
abcaaaaaaaccccccccccccccccccaaaaccccccaccaccccaaaaaacccccccccaaaaaaaaaaaaaaacccccccccccccccccccccacccccccccaacccccccccccccccccccccccccccccaaaccccccccccccccaaaa
abccaacccaccccccccccccccccccccaaacccccccccccccaaaaaaccccccccccaaaaaaaaacaaacccccccccccccccccccaaaacccccccccccccccccccccccccaacccccccaaccccaaacccccccccccccaaaaa
abcaaaaaacccccccccccccccccccccccccccccccccccccaaaaaccccccccccaaaaaaaaaaccccaacaaccccccccccccccaaaaaacccccccccccccccccccccccaacccccccaaaacaaaaccccccccccccccaccc
abccaaaaacccccccccccccccccccccccccccccccccccaaccaaacccccccccaaaaaaaaaaaacccaaaaccccccccccccccccaaaaacccccccccccccccaacaaaaaaacccccccaaaaaaaaacccccccccccccccccc
abccaaaaaacccccccccccccccccccccccccccccaaacaaaccccccccccccccaaaaaaaaaaacccaaaaacccccccccccccccaaaaacccccccccccccaaaaaccaaaaaaaaccccccaaaaaalllllllcccaacccccccc
abccaaaaaaccccccaaaaacccccccccaaaccccccaaaaaaaccccccccccccccaaacaaacaaacccaaaaaaccccccccccccccaccaaccccccccccccccaaaaacaaaaaaaaajkkkkkkkkkklllllllccccaaaaacccc
abccaaaaacccccccaaaaacccccccccaaaaccccccaaaaaaccccccccaacaacccccaaacccccccacaaaaccccccccaaaccccccccccccccccccccccaaaaaccaaaaaaajjkkkkkkkkkkllssllllcccaaaaacccc
abcccaaaaccccccaaaaaacccccccccaaaaccccccaaaaaaaaccccccaaaaacccccaaccccccccccaacccccccccaaaacccccccccccccccaaccccaaaaaccaaaaaacjjjjkkkkkkkkssssssslllccaaaaccccc
abcccccccccccccaaaaaacccccccccaaaccccccaaaaaaaaacaaccccaaaaacccccccccccccccaaccccccccccaaaaccccccccccccccaaacccccccaaccaaaaaajjjjrrrrrrsssssssssslllcccaaaccccc
abcccccccccccccaaaaaacccccccccccccccccaaaaaaaaaaaaaaacaaaaaacccccccccccaaacaacccccccccccaaaccccaaacccccaaaaaaaaccccccccaacaaajjjrrrrrrrsssssuusssslmcccaaaacccc
abcccccccccccccccaacccccccccccccccaacaaaacaaaccaaaaaacaaaaccccccccccccccaaaaaccccccccccccccccccaaaaacccaaaaaaaaccccccccccccaajjjrrrruuurssuuuuvsqqmmcddaaaacccc
abccccccccccccccccccccccccccccccccaaaaacccaaacccaaaaccccaaccccccccccccccaaaaaaacccccccccccccccaaaaaaccccaaaaaacccccccccccccccjjrrruuuuuuuuuuuuvvqqmmmdddccccccc
abcccccccccccccccccccccccacccccccccaaaaaccaaacccaaaaccccccccccccccccccccaaaaaaacccccccccccccccaaaaaaccccaaaaaacccccccccaaccccjjjrrtuuuuuuuuyyvvvqqmmmddddcccccc
abccccccccccccccccccccaaaaccccccccaaaaaacccccaacaccacccccccccccccccccccaaaaaaccccccccccccccccccaaaaaccccaaaaaaccccccccaaaccccjjjrrttuxxxuuxyyyvvqqmmmmdddcccccc
abcccccccccaacccccccccaaaaaaccccccaaaaccccccaaaccccccccccccccccccccccccaacaaaccccccccccccccccccaacaaccccaaccaaccccaaaaaaaccccjjjrrtttxxxxxyyyyvvqqqmmmddddccccc
abccccccccaaaacccccccccaaaacccccccccaaccccccaaacaaaccccccccccccccccccaaccccaacccccccccccccccccccccccccccccccccccccaaaaaaaaaacijjqrtttxxxxxyyyvvvqqqqmmmdddccccc
abcccccacaaaaaccccccccaaaaaccccccccccccccaaaaaaaaaacccccccccccccccccaaaccccccccccccccccccccccccccccccccccccccccccccaaaaaaaaaciiiqqqttxxxxxyyyvvvvqqqqmmmdddcccc
SbcccccaaaaaaaaaacccccaacaaccccccccccccccaaaaaaaaaccccccccccccccaaacaaacccccccccccccccccccccccccccccccccccccccccccccaaaaaaaciiiqqqtttxxxEzzyyyyvvvqqqmmmdddcccc
abcccccaaaaaaaaaaccccccccccccaaccccccccccccaaaaaccccccccccccccccaaaaaaaaaacccccccaacccccccccccccaacccccccccccccccccaaaaaaccciiiqqqttxxxxyyyyyyyyvvvqqqmmmeddccc
abcccccccaaaaaacccccccccccaaaaccccccccccaaaaaaaaacccccccaaaacccccaaaaaaaaacccccaaaaccccccccccaacaaaccccccccccccccccaaaaaaaciiiqqqtttxxyyyyyyyyyvvvvqqqnnneeeccc
abcccccccaaaaaacccccccccccaaaaaaccccccccaaaaaaaaaaccccccaaaaccccccaaaaaaaccccccaaaaaaccccccccaaaaacccccccccccccccccaaccaaaciiiqqtttxxxxwwyyywwvvvvrrrnnnneeeccc
abcccccccaaaaaaccccccccccccaaaaacccccccaaaaaaacaaaccccccaaaacccccaaaaaacccccccccaaaaccccccccccaaaaaaccccaaccccccccccccccaaciiqqqtttxxxwwwyywwwwvvrrrrnnneeecccc
abccccccaaaaaaaaccccccccccaaaaaccccccccaaaaaaccccccccccccaaacccccaaaaaaacccccccaaaaaccccccccaaaaaaaaacccaaccccccccccccccccciiqqqtttttwwswwyywwrrrrrrnnnneeecccc
abccccccccccccacccccccccccaccaaccccaaccaaaaaacccccccccccaccccccccaaacaaacccccccaacaaccccccccaaaaacaaaaaaaacccccccccaacccccciiqqqqttssssswwwwwrrrrnnnnnneeeecccc
abcccccccccccccccccccccccccccccaaaaaaccccaacccccccaaacaaacccccccccccccaacaaacccccccccccccccccccaaaccaaaaaaaaccccaacaacccccciiiqqpppsssssswwwwrrrnnnnneeeeeccccc
abcccccccccccccccccccccccccccccaaaaaaaccccccccccccaaaaaaaccccccccccccccccaaacccccccccccccccccccaaaccaaaaaaaaacccaaaaacccccchhhhppppppppssswwwrroonnfeeeeacccccc
abccccccccccccccccccccaaaaaccccaaaaaaaaccccccccccccaaaaaaccccccccccccccaaaaaaaacccccccccccccccccccccaaaaaaaaaccccaaaaaaccccchhhhhpppppppsssssrroonfffeeaaaacccc
abccccccccccccccccccccaaaaacccccaaaaaaaccccccccccccaaaaaaaaccccccccccccaaaaaaaacccccccccccccccccccccaaaaaacccccaaaaaaaacccccchhhhhhhppppsssssrooofffffaaaaacccc
abcccccaacaaacccccccccaaaaaacccaaaaaacccccccccccccaaaaaaaaacccccccccccccaaaaacccccccccccccccccccccccaaaaaaaccccaaaaaccaccccccchhhhhhhhpppssssrooofffcaaaaaccccc
abcccccaaaaaacccccccccaaaaaacccaaaaaaccccccccccccaaaaaaaaaacccccccccccccaaaaaaccccccccccccccccccccccaccaaaccccccacaaaccaacccccccchhhhhgppooooooofffcaaaaacccccc
abcccccaaaaaacccccccccaaaaaaccccccaaacaacccccccccaaacaaaccccccccccaaacccaaaaaaccccccccccccccccccccccccccaaacccccccaaacaaaccccccccccchgggoooooooffffcaaaaaaccccc
abaccccaaaaaaaccccccccccaaccccccccaaaaaacccccccccccccaaaccccccccccaaaaccaaaccacaacaacccccccccccccccccccccccccccccccaaaaaaaaccccccccccggggoooooffffccaccaaaccccc
abacccaaaaaaaaccccccccccccccccccccaaaaaccccccccccccccaacccccccaaacaaaacccaaccccaaaaacccccccccccccccccccaacaacccccccaaaaaaaacccccccccccggggggggfffcccccccccccccc
abacccaaaaaaaaccccccccaaacccccccccaaaaaaccccccccccccccccccccccaaacaaaacaaaaccccaaaaaaccccccccaaccccccccaaaaaccccccccaaaaaaacccccccccccaaggggggffcccccccccccccca
abcccccccaaacccccccccaaaaaaccccccaaaaaaaacccccccccccccccccccaaaaaaaaaaaaaaaccccaaaaaaccccccacaaaacccccccaaaaacccccccaaaaaccccccccccccaaacgggggaccccccccccccccaa
abcccccccaaccccccccccaaaaaaccccccaaaaaaaacccccccaaacccccccccaaaaaaaaaaaaaaaacccaaaaaaccccccaaaaaaccccccaaaaaaccccccaaaaaaacccccccccccaaaccccaaaccccccccccaaacaa
abcccccccccccccccccccaaaaaccccccccccaaccccccccaaaaaccccccccccaaaaaaaaaaaaaaaaccccaaaccccccccaaaacccccccaaaaccccccccccccaaccccccccccccccccccccccccccccccccaaaaaa
abccccccccccccccccccccaaaaacccccccccaaccccccccaaaaaacccccccccaaaaaaaaaaaaaaaacccccccccccccccaaaacccccccccaacccccccccccccccccccccccccccccccccccccccccccccccaaaaa
";
}
