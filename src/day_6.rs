pub fn part_1() {
    fn parse_nums<'a>(line: &'a str) -> impl Iterator<Item = usize> + 'a {
        let (_, numstr) = line.split_once(':').unwrap();
        numstr
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
    }
    let mut lines = _INPUT.trim().lines();
    println!(
        "Wins: {}",
        parse_nums(lines.next().unwrap())
            .zip(parse_nums(lines.next().unwrap()))
            .map(|(time, distance)| {
                (0..time)
                    .map(|t| (time - t) * t)
                    .filter(|d| *d > distance)
                    .count()
            })
            .product::<usize>()
    );
}

pub fn part_2() {
    fn parse_digits(line: &str) -> usize {
        let (_, numstr) = line.split_once(':').unwrap();
        numstr
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .parse::<usize>().unwrap()
    }
    let mut lines = _INPUT.trim().lines();
    let time = parse_digits(lines.next().unwrap()) as f64;
    let distance = parse_digits(lines.next().unwrap()) as f64;
    let disc = (time * time) - (4. * distance);
    println!("Wins: {}", if disc < 0. {0} else {
        f64::floor(f64::sqrt(disc)) as usize
    });
}

const _EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

const _INPUT: &str = "Time:        48     93     84     66
Distance:   261   1192   1019   1063";
