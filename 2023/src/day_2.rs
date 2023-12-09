/*
--- Day 2: Cube Conundrum ---

You're launched high into the atmosphere! The apex of your trajectory
just barely reaches the surface of a large island floating in the
sky. You gently land in a fluffy pile of leaves. It's quite cold, but
you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for
the lack of snow. He'll be happy to explain the situation, but it's a
bit of a walk, so you have some time. They don't get many visitors up
here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are
either red, green, or blue. Each time you play this game, he will hide
a secret number of cubes of each color in the bag, and your goal is to
figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf
will reach into the bag, grab a handful of random cubes, show them to
you, and then put them back in the bag. He'll do this a few times per
game.

You play several games and record the information from each game (your
puzzle input). Each game is listed with its ID number (like the 11 in
Game 11: ...) followed by a semicolon-separated list of subsets of
cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put
back again). The first set is 3 blue cubes and 4 red cubes; the second
set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is
only 2 green cubes.

The Elf would first like to know which games would have been possible
if the bag contained only 12 red cubes, 13 green cubes, and 14 blue
cubes?

In the example above, games 1, 2, and 5 would have been possible if
the bag had been loaded with that configuration. However, game 3 would
have been impossible because at one point the Elf showed you 20 red
cubes at once; similarly, game 4 would also have been impossible
because the Elf showed you 15 blue cubes at once. If you add up the
IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been
loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What
is the sum of the IDs of those games?

--- Part Two ---

The Elf says they've stopped producing snow because they aren't
getting any water! He isn't sure why the water stopped; however, he
can show you how to get to the water source to check it out for
yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each
game you played, what is the fewest number of cubes of each color that
could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, the game could have been played with as few as 4 red, 2
green, and 6 blue cubes. If any color had even one fewer cube, the
game would have been impossible.

Game 2 could have been played with a minimum of 1 red, 3 green, and 4
blue cubes.

Game 3 must have been played with at least 20 red, 13 green, and 6
blue cubes.

Game 4 required at least 14 red, 3 green, and 15 blue cubes.

Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the
bag.

The power of a set of cubes is equal to the numbers of red, green, and
blue cubes multiplied together. The power of the minimum set of cubes
in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36,
respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been
present. What is the sum of the power of these sets?

*/

#[cfg(test)]
mod test {
    fn part_1(input: &str) -> usize {
        let input = input.trim();
        const MAX_COUNTS: [usize; 3] = [12, 13, 14];
        let mut total: usize = 0;
        for game in input.lines() {
            let parts: Vec<_> = game.split(':').collect();
            assert_eq!(parts.len(), 2);
            let gi: usize = match parts[0].strip_prefix("Game") {
                Some(numstr) => numstr.trim().parse().unwrap(),
                None => panic!("Cannot parse game index"),
            };
            let mut possible: bool = true;
            'outer: for revealstr in parts[1].trim().split(';') {
                for pairstr in revealstr.trim().split(',') {
                    let pair: Vec<_> = pairstr.split_whitespace().collect();
                    let count: usize = pair[0].parse().unwrap();
                    let color = pair[1].trim();
                    if count
                        > MAX_COUNTS[match color {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!("Unknown color!"),
                        }]
                    {
                        possible = false;
                        break 'outer;
                    }
                }
            }
            if possible {
                total += gi;
            }
        }
        return total;
    }

    fn part_2(input: &str) -> usize {
        let input = input.trim();
        let mut total: usize = 0;
        for game in input.lines() {
            let parts: Vec<_> = game.split(':').collect();
            assert_eq!(parts.len(), 2);
            let mut minset: [usize; 3] = [0, 0, 0];
            let _gi: usize = match parts[0].strip_prefix("Game") {
                Some(numstr) => numstr.trim().parse().unwrap(),
                None => panic!("Cannot parse game index"),
            };
            for revealstr in parts[1].trim().split(';') {
                for pairstr in revealstr.trim().split(',') {
                    let pair: Vec<_> = pairstr.split_whitespace().collect();
                    let count: usize = pair[0].parse().unwrap();
                    let color = match pair[1].trim() {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!("Unknown color!"),
                    };
                    minset[color] = usize::max(count, minset[color]);
                }
            }
            let power = minset.iter().fold(1, |acc, v| acc * v);
            total += power;
        }
        return total;
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 8);
        assert_eq!(part_1(INPUT), 2237);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 2286);
        assert_eq!(part_2(INPUT), 66681);
    }

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    const INPUT: &str = "Game 1: 4 red, 1 green, 15 blue; 6 green, 2 red, 10 blue; 7 blue, 6 green, 4 red; 12 blue, 10 green, 3 red
Game 2: 3 green, 18 blue; 14 green, 4 red, 2 blue; 3 red, 14 green, 15 blue
Game 3: 12 green, 2 blue; 9 green; 1 red, 11 blue, 4 green
Game 4: 4 blue, 8 green, 5 red; 6 red, 7 blue, 9 green; 2 green, 2 red, 2 blue; 2 green, 6 blue, 9 red; 10 red, 9 green
Game 5: 12 red, 1 green, 7 blue; 13 red, 16 blue; 16 blue, 10 red; 4 blue; 16 blue, 7 red; 1 blue, 7 red
Game 6: 17 blue, 2 red; 5 blue, 6 green, 2 red; 5 green, 5 blue; 5 green, 12 blue, 4 red
Game 7: 2 red, 1 blue, 10 green; 8 red, 14 green, 9 blue; 15 red, 1 blue, 6 green; 9 blue, 3 green, 10 red; 7 blue, 13 red, 4 green
Game 8: 1 green, 2 blue; 7 red, 2 blue, 1 green; 1 red, 2 green; 4 red, 1 blue; 11 red, 2 green, 2 blue; 1 blue, 2 green, 11 red
Game 9: 11 green, 11 blue, 6 red; 2 green, 3 blue, 2 red; 2 red, 11 blue, 14 green; 5 green, 7 red, 7 blue; 7 green, 1 red, 12 blue; 1 red, 8 green, 7 blue
Game 10: 2 red, 8 green, 7 blue; 10 red, 5 green, 2 blue; 4 red, 8 green, 16 blue; 10 blue, 3 green, 15 red
Game 11: 2 blue, 2 green, 5 red; 1 green, 3 red, 3 blue; 11 green, 1 red, 2 blue
Game 12: 8 blue, 11 green, 14 red; 10 green, 13 red, 2 blue; 1 red, 6 green, 4 blue; 13 red, 11 green, 6 blue
Game 13: 15 red, 17 green, 1 blue; 12 red, 1 blue, 1 green; 2 red, 1 blue, 14 green
Game 14: 6 green, 11 red, 3 blue; 6 green, 2 blue; 2 green, 10 red, 8 blue; 2 red; 1 green, 9 red; 3 blue, 1 green, 3 red
Game 15: 11 blue, 11 green, 4 red; 3 green, 10 blue; 2 red, 9 green, 9 blue
Game 16: 2 blue, 11 green; 1 red, 1 blue, 11 green; 12 green, 1 blue, 1 red; 3 blue, 14 green, 1 red; 14 green, 4 blue; 2 blue, 12 green
Game 17: 1 red, 2 blue, 4 green; 4 blue, 3 green; 1 green, 1 red, 6 blue; 1 red, 7 blue; 2 green
Game 18: 3 red, 3 blue, 7 green; 2 blue, 2 red, 2 green; 4 red, 12 green; 5 green, 2 blue, 4 red; 3 red
Game 19: 15 red, 7 blue, 10 green; 5 green, 8 red; 9 green, 8 red; 5 red, 10 green
Game 20: 15 blue, 6 green, 11 red; 13 red, 9 blue, 1 green; 15 blue, 10 red, 11 green
Game 21: 15 red, 4 green; 11 red, 2 blue, 4 green; 5 blue, 2 green, 4 red; 4 red, 5 blue; 6 red, 3 blue, 1 green
Game 22: 4 green, 4 red, 13 blue; 3 red, 7 blue, 9 green; 12 blue, 13 green, 5 red
Game 23: 20 green, 4 red; 6 blue, 9 red, 7 green; 6 green
Game 24: 1 green, 3 blue, 6 red; 1 green, 1 blue, 2 red; 3 blue, 5 red, 1 green
Game 25: 2 red, 9 blue, 2 green; 2 green, 1 red, 5 blue; 3 red, 1 green, 3 blue; 8 blue, 2 green, 3 red; 12 blue, 3 red; 1 blue, 2 green, 1 red
Game 26: 2 blue, 5 green, 20 red; 2 blue, 6 red, 9 green; 3 red, 2 blue, 5 green
Game 27: 17 blue, 2 red, 14 green; 15 green, 16 blue, 2 red; 13 blue, 13 green; 1 red, 7 green, 3 blue; 1 blue, 2 green
Game 28: 5 blue, 6 red, 3 green; 7 red, 19 green; 11 blue, 13 green
Game 29: 1 blue, 8 red, 7 green; 1 green, 1 red; 8 red, 7 green, 1 blue; 7 green, 2 red; 1 blue, 7 red; 1 blue, 2 red, 5 green
Game 30: 3 red, 17 blue; 11 red, 3 blue, 8 green; 7 green, 12 blue, 10 red; 5 blue, 2 green
Game 31: 14 blue, 7 green; 12 green, 14 blue, 2 red; 17 blue, 2 red, 8 green; 2 red, 3 blue, 11 green; 9 green, 4 blue; 1 red, 3 green, 1 blue
Game 32: 15 red, 1 blue, 10 green; 15 green, 10 red, 1 blue; 2 red, 6 green, 1 blue
Game 33: 10 green, 1 red, 16 blue; 11 blue, 14 green, 3 red; 14 green, 13 blue; 17 blue, 2 red, 3 green
Game 34: 8 red, 7 blue, 8 green; 3 green, 1 red; 1 red, 1 green, 5 blue; 6 red, 8 green, 2 blue; 7 red, 8 blue, 3 green
Game 35: 5 blue, 19 red; 2 blue, 11 red, 1 green; 16 red, 10 blue; 7 green, 3 blue, 6 red; 3 green, 18 red, 5 blue; 8 blue, 5 red
Game 36: 9 red, 6 green, 10 blue; 9 red, 15 green, 6 blue; 6 red, 1 blue, 14 green
Game 37: 7 green, 8 red, 2 blue; 3 blue, 5 red, 16 green; 1 green, 1 red, 3 blue
Game 38: 5 green, 5 red, 3 blue; 10 blue, 19 red, 9 green; 2 red, 3 blue, 11 green
Game 39: 15 red, 11 blue, 5 green; 11 green, 2 red, 6 blue; 2 blue, 3 green, 6 red; 15 red, 3 blue, 13 green
Game 40: 7 green, 4 red, 1 blue; 6 blue, 6 green, 2 red; 2 blue, 3 red, 1 green; 1 blue, 3 red, 3 green; 2 red, 5 green, 3 blue
Game 41: 10 blue, 8 green, 9 red; 7 blue, 9 red, 2 green; 10 blue, 4 red, 5 green
Game 42: 8 blue, 13 green, 14 red; 8 blue, 1 green, 11 red; 4 red, 6 green, 3 blue; 14 green, 4 red, 2 blue
Game 43: 2 red, 10 green, 19 blue; 5 blue, 4 green, 9 red; 9 green, 9 red, 2 blue
Game 44: 6 red, 2 green, 3 blue; 2 blue, 12 red, 6 green; 1 red, 10 blue; 12 red, 6 green, 2 blue; 14 red, 13 green, 3 blue; 10 green, 9 blue, 11 red
Game 45: 2 blue, 1 red, 1 green; 1 green, 1 blue; 2 green, 2 blue
Game 46: 7 green, 1 red; 1 green, 4 blue, 1 red; 3 blue, 4 green, 1 red; 1 red, 4 green; 1 blue, 12 green, 1 red; 16 green, 1 blue
Game 47: 4 blue, 8 green, 3 red; 6 red, 1 green, 3 blue; 16 green, 4 blue, 1 red; 4 blue, 8 red
Game 48: 1 blue, 9 red, 8 green; 8 green, 2 blue, 6 red; 2 green; 4 blue, 5 red; 1 blue, 9 red, 9 green; 1 red, 1 blue, 3 green
Game 49: 3 green, 2 blue; 7 blue, 4 red; 20 green, 5 red, 13 blue; 20 green, 1 red, 6 blue
Game 50: 3 red, 3 green; 3 green, 3 red; 2 blue, 10 red; 3 blue, 5 green; 14 red, 2 green, 2 blue; 7 red, 2 green
Game 51: 3 green, 3 blue, 2 red; 4 green, 16 red, 3 blue; 1 blue, 3 red; 9 red, 1 blue, 4 green
Game 52: 6 red, 18 green, 7 blue; 2 blue, 1 red, 5 green; 8 blue, 6 red, 1 green; 1 red, 1 blue; 6 red, 3 green, 10 blue
Game 53: 1 blue, 10 red, 3 green; 13 red, 2 green, 1 blue; 1 green, 2 red
Game 54: 4 blue, 6 green, 2 red; 5 blue, 6 red, 2 green; 6 blue, 4 green, 8 red; 13 red, 10 blue, 1 green; 5 red, 5 green, 9 blue
Game 55: 4 green, 18 red, 4 blue; 9 blue, 7 green, 16 red; 5 red, 6 blue, 14 green; 13 green, 11 red, 9 blue; 6 blue, 13 green, 1 red; 10 blue, 12 red, 14 green
Game 56: 8 green, 5 blue, 10 red; 10 green, 7 red, 12 blue; 11 red, 12 blue, 1 green; 4 blue, 6 red, 10 green; 17 blue, 8 green, 2 red
Game 57: 1 green, 2 red; 2 green, 5 red, 1 blue; 13 red, 3 green, 4 blue; 3 blue, 13 red, 9 green
Game 58: 1 red, 7 blue, 4 green; 2 green, 1 blue, 1 red; 1 green, 11 blue; 12 blue; 1 blue, 5 green, 1 red; 3 green, 11 blue, 1 red
Game 59: 5 green, 3 blue, 17 red; 2 red, 9 green; 1 blue, 4 green
Game 60: 5 red, 5 green, 1 blue; 2 red, 2 blue, 6 green; 2 red, 3 blue, 3 green
Game 61: 2 green, 3 blue, 4 red; 17 green, 1 blue; 1 green, 6 red, 4 blue; 3 blue, 9 green, 3 red; 18 green, 7 red, 2 blue
Game 62: 5 red; 3 blue, 9 green; 3 red, 13 blue, 10 green; 14 green, 1 red, 2 blue; 7 blue, 13 green
Game 63: 12 blue, 5 green; 5 green, 1 red, 1 blue; 4 red, 7 green, 9 blue; 8 blue, 2 green, 7 red
Game 64: 3 blue, 11 green; 5 blue, 2 red, 5 green; 17 green, 5 blue, 1 red; 4 red, 3 blue, 4 green
Game 65: 2 red, 1 blue, 2 green; 7 green, 2 red, 1 blue; 2 blue, 7 green, 1 red; 3 blue, 8 green, 3 red
Game 66: 4 red, 12 blue, 1 green; 20 blue, 3 green, 2 red; 11 blue, 1 green
Game 67: 12 blue, 10 red, 13 green; 19 green, 4 red, 7 blue; 12 red, 9 blue, 13 green
Game 68: 2 blue, 17 green; 12 green, 2 red; 5 red, 2 green, 4 blue; 4 blue
Game 69: 17 blue, 3 red, 1 green; 4 green, 8 blue, 8 red; 4 green, 7 red, 1 blue; 8 red, 1 green, 11 blue; 13 blue, 10 red, 9 green; 14 blue, 5 green, 6 red
Game 70: 1 red, 2 blue, 4 green; 13 blue, 3 red, 2 green; 6 green, 8 blue
Game 71: 5 red, 7 green, 1 blue; 11 green, 4 red, 1 blue; 1 red, 12 green, 10 blue; 1 red, 7 blue, 12 green
Game 72: 9 blue, 4 green, 1 red; 6 green, 4 blue; 8 green, 5 blue, 1 red
Game 73: 1 blue, 10 green, 14 red; 4 green; 2 blue, 9 red, 4 green; 2 blue, 13 green; 13 green, 13 red; 7 red, 5 green, 2 blue
Game 74: 3 red, 1 blue, 3 green; 4 green, 1 blue, 1 red; 2 blue, 10 green, 1 red; 1 blue, 3 red, 1 green
Game 75: 1 red, 1 blue, 1 green; 2 red, 1 green, 4 blue; 2 red, 4 blue; 1 blue, 1 red
Game 76: 4 green, 2 blue, 6 red; 7 green, 1 red; 8 green, 4 red
Game 77: 8 green, 7 blue, 5 red; 6 red, 14 green, 7 blue; 8 green, 7 blue; 1 red, 8 green, 8 blue
Game 78: 6 red, 3 blue, 3 green; 7 blue, 10 red; 5 green, 10 blue, 1 red; 3 green, 11 blue, 4 red; 14 red, 9 blue, 2 green; 16 red, 2 green, 12 blue
Game 79: 1 green; 5 green; 11 green, 3 blue, 2 red; 3 blue
Game 80: 2 green, 2 red; 1 blue, 1 green, 1 red; 1 blue, 1 green, 2 red; 2 red; 5 green
Game 81: 10 blue, 2 red, 9 green; 4 red, 12 blue, 5 green; 7 green, 4 blue, 6 red; 1 red, 13 green, 14 blue; 13 green, 11 blue
Game 82: 4 blue, 2 green; 7 blue, 3 green, 5 red; 1 red, 4 blue, 3 green; 5 blue, 1 red, 6 green; 6 green, 4 red; 11 blue, 3 red, 5 green
Game 83: 12 green; 5 red, 8 green; 11 red, 14 green, 1 blue; 9 green, 4 red
Game 84: 5 blue, 1 red; 16 blue, 5 green; 1 red, 9 blue, 3 green; 11 blue; 1 green, 2 blue; 1 red, 7 blue, 4 green
Game 85: 17 red, 5 blue; 18 blue, 2 red, 2 green; 18 blue, 2 green, 8 red
Game 86: 4 red, 1 blue, 11 green; 6 blue, 7 green, 1 red; 3 green, 4 blue; 2 red, 7 blue, 2 green
Game 87: 4 red, 5 blue; 1 green, 15 red, 1 blue; 11 blue, 12 red
Game 88: 11 green, 3 red, 1 blue; 6 green, 1 blue, 1 red; 1 blue, 3 green; 2 blue, 4 green, 2 red
Game 89: 2 green; 1 red, 2 green, 3 blue; 4 blue, 1 red, 10 green; 4 blue, 5 green; 6 blue, 1 red, 10 green
Game 90: 15 red, 7 green, 17 blue; 7 blue, 1 red; 7 green, 6 red, 3 blue
Game 91: 2 blue, 17 red, 6 green; 1 green, 1 blue, 6 red; 6 red, 4 blue; 10 green, 14 red, 1 blue; 7 blue, 10 green, 10 red; 16 red, 11 green, 9 blue
Game 92: 1 green, 8 blue, 4 red; 4 green, 4 red, 4 blue; 1 green, 7 red, 4 blue
Game 93: 11 blue, 12 red, 1 green; 9 blue, 2 green, 5 red; 7 red, 5 blue, 2 green
Game 94: 7 blue, 10 green; 9 green, 9 blue, 2 red; 1 red, 5 green, 4 blue
Game 95: 1 green, 1 blue, 2 red; 6 red; 1 blue; 1 green, 1 blue, 6 red
Game 96: 1 blue, 1 red, 2 green; 4 red, 13 green, 1 blue; 1 blue, 13 green, 5 red; 7 green, 4 red
Game 97: 10 blue, 5 red, 5 green; 4 red, 8 green, 2 blue; 5 red, 2 green, 15 blue; 2 red, 1 green, 4 blue; 2 red, 14 blue; 14 blue, 4 green
Game 98: 11 red, 8 green, 9 blue; 3 blue, 1 green, 14 red; 10 blue, 2 red, 4 green; 7 blue, 11 red, 3 green; 5 red, 12 blue, 4 green; 7 green, 7 blue, 8 red
Game 99: 3 green, 2 blue, 1 red; 15 red, 8 blue, 7 green; 18 red, 12 blue, 2 green
Game 100: 11 red, 1 blue, 2 green; 3 red, 3 green; 1 blue, 8 red, 4 green; 5 green, 5 blue, 1 red; 2 green, 1 red, 6 blue; 2 green, 8 red, 1 blue
";
}
