/*
You take the boat and find the gardener right where you were told he
would be: managing a giant "garden" that looks more to you like a
farm.

"A water source? Island Island is the water source!" You point out
that Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it
with! Can't make snow with dirty water. Don't worry, I'm sure we'll
get more sand soon; we only turned off the water a few
days... weeks... oh no." His face sinks into a look of horrified
realization.

"I've been so busy making sure everyone here has food that I
completely forgot to check why we stopped getting more sand! There's a
ferry leaving soon that is headed over in that direction - it's much
faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up
another. "While you wait for the ferry, maybe you can help us with our
food production problem. The latest Island Island Almanac just arrived
and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be
planted. It also lists what type of soil to use with each kind of
seed, what type of fertilizer to use with each kind of soil, what type
of water to use with each kind of fertilizer, and so on. Every type of
seed, soil, fertilizer and so on is identified with a number, but
numbers are reused by each category - that is, soil 123 and fertilizer
123 aren't necessarily related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

The almanac starts by listing which seeds need to be planted: seeds
79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to
convert numbers from a source category into numbers in a destination
category. That is, the section that starts with seed-to-soil map:
describes how to convert a seed number (the source) to a soil number
(the destination). This lets the gardener and his team know which soil
to use with which seeds, which water to use with which fertilizer, and
so on.

Rather than list every source number and its corresponding destination
number one by one, the maps describe entire ranges of numbers that can
be converted. Each line within a map contains three numbers: the
destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48

The first line has a destination range start of 50, a source range
start of 98, and a range length of 2. This line means that the source
range starts at 98 and contains two values: 98 and 99. The destination
range is the same length, but it starts at 50, so its two values are
50 and 51. With this information, you know that seed number 98
corresponds to soil number 50 and that seed number 99 corresponds to
soil number 51.

The second line means that the source range starts at 50 and contains
48 values: 50, 51, ..., 96, 97. This corresponds to a destination
range starting at 52 and also containing 48 values: 52, 53, ..., 98,
99. So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same
destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil
numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51

With this map, you can look up the soil number required for each
initial seed number:

Seed number 79 corresponds to soil number 81.

Seed number 14 corresponds to soil number 14.

Seed number 55 corresponds to soil number 57.

Seed number 13 corresponds to soil number 13.

The gardener and his team want to get started as soon as possible, so
they'd like to know the closest location that needs a seed. Using
these maps, find the lowest location number that corresponds to any of
the initial seeds. To do this, you'll need to convert each seed number
through other categories until you can find its corresponding location
number. In this example, the corresponding types are:

Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78,
humidity 78, location 82.

Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42,
humidity 43, location 43.

Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82,
humidity 82, location 86.

Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34,
humidity 35, location 35.

So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the
initial seed numbers?

--- Part Two ---

Everyone will starve if you only plant such a small number of
seeds. Re-reading the almanac, it looks like the seeds: line actually
describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair,
the first value is the start of the range and the second value is the
length of the range. So, in the first line of the example above:

seeds: 79 14 55 13

This line describes two ranges of seed numbers to be planted in the
garden. The first range starts with seed number 79 and contains 14
values: 79, 80, ..., 91, 92. The second range starts with seed number
55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a
total of 27 seed numbers.

In the above example, the lowest location number can be obtained from
seed number 82, which corresponds to soil 84, fertilizer 84, water 84,
light 77, temperature 45, humidity 46, and location 46. So, the lowest
location number is 46.

Consider all of the initial seed numbers listed in the ranges on the
first line of the almanac. What is the lowest location number that
corresponds to any of the initial seed numbers?

 */

#[cfg(test)]
mod test {

    fn part_1(input: &str) -> usize {
        let mut lines = input.trim().lines().filter(|ln| !ln.is_empty());
        let seeds: Vec<usize> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|numstr| numstr.parse().unwrap())
            .collect();
        let lines: Vec<_> = lines.collect();
        lines
            .split(|ln| ln.ends_with(" map:"))
            .map(|maplines| {
                maplines
                    .iter()
                    .map(|ln| {
                        let (dst, rest) = ln.split_once(' ').unwrap();
                        let (src, len) = rest.split_once(' ').unwrap();
                        (
                            src.parse().unwrap(),
                            dst.parse().unwrap(),
                            len.parse().unwrap(),
                        )
                    })
                    .collect::<Vec<(usize, usize, usize)>>()
            })
            .fold(seeds, |acc, map| {
                acc.iter()
                    .map(|seed| {
                        match map
                            .iter()
                            .find(|(src, _dst, len)| *seed >= *src && *seed < *src + *len)
                        {
                            Some((src, dst, _len)) => *dst + (*seed - *src),
                            None => *seed,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .iter()
            .min()
            .unwrap()
            .clone()
    }

    fn part_2(input: &str) -> i64 {
        let mut lines = input.trim().lines().filter(|ln| !ln.is_empty());
        let seedranges: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|numstr| numstr.parse::<i64>().unwrap())
            .collect();
        assert_eq!(seedranges.len() % 2, 0);
        let mut ping: Vec<_> = seedranges
            .chunks(2)
            .map(|c| (c[0], (c[1] + c[0])))
            .collect();
        let mut pong: Vec<(i64, i64)> = Vec::new();
        let mut maping: Vec<(i64, i64)> = Vec::new();
        let mut mapong: Vec<(i64, i64)> = Vec::new();
        let lines = lines.collect::<Vec<_>>();
        fn is_valid((start, end): (i64, i64)) -> bool {
            start > 0 && end > start
        }
        for maplines in lines.split(|l| l.ends_with(" map:")) {
            maping.clear();
            maping.extend(ping.iter());
            pong.clear();
            for line in maplines {
                let (src_lower, src_upper, shift) = {
                    let (dst, rest) = line.split_once(' ').unwrap();
                    let (src, len) = rest.split_once(' ').unwrap();
                    let src = src.parse::<i64>().unwrap();
                    let dst = dst.parse::<i64>().unwrap();
                    let len = len.parse::<i64>().unwrap();
                    (src, src + len, dst - src)
                };
                mapong.clear();
                for &(start, end) in maping.iter() {
                    if !is_valid((start, end)) {
                        continue;
                    }
                    let range = (start, i64::min(end, src_lower));
                    if is_valid(range) {
                        mapong.push(range);
                    }
                    let range = (
                        i64::max(start, src_lower) + shift,
                        i64::min(end, src_upper) + shift,
                    );
                    if is_valid(range) {
                        pong.push(range);
                    }
                    let range = (i64::max(start, src_upper), end);
                    if is_valid(range) {
                        mapong.push(range);
                    }
                }
                maping.clear();
                maping.extend(mapong.iter());
            }
            pong.extend(maping.iter());
            ping.clear();
            ping.extend(pong.iter());
        }
        ping.iter().map(|(start, _end)| *start).min().unwrap()
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 35);
        assert_eq!(part_1(INPUT), 165788812);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 46);
        assert_eq!(part_2(INPUT), 1928058);
    }

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    const INPUT: &str = "seeds: 2494933545 159314859 4045092792 172620202 928898138 554061882 2740120981 81327018 2031777983 63513119 2871914181 270575980 2200250633 216481794 3289604059 25147787 3472625834 10030240 260990830 232636388

seed-to-soil map:
3272284283 2724782980 1022683013
138187491 4195038636 99928660
2359623759 797621236 127984779
662451929 2224466386 266466256
928918185 714355413 83265823
1012184008 3891516474 303522162
3063776460 1098322140 208507823
2194238166 1306829963 50525692
357106588 2091837170 132629216
2244763858 2490932642 114859901
2050187685 3747465993 144050481
489735804 925606015 172716125
2487608538 138187491 576167922
238116151 2605792543 118990437
1315706170 1357355655 734481515

soil-to-fertilizer map:
4265669768 2142212766 29297528
2030756625 2171510294 69737894
3038084234 3411621093 262803613
2410534622 3266307064 145314029
2667304792 2241248188 370779442
2100494519 3921619167 310040103
3611390334 2612027630 654279434
2555848651 2030756625 111456141
733063720 869238953 195075492
3300887847 4231659270 63308026
3364195873 3674424706 247194461
928139212 733063720 136175233

fertilizer-to-water map:
0 772139976 154052576
909628165 428370542 51644443
3172969725 4109584032 185383264
1116931128 1046566515 14194115
223777814 10055892 255169216
2512535520 1229983026 60386000
3109777744 3899207072 16374329
4030761870 3829858282 12540292
828135093 718323602 53816374
2358450176 2554590817 154085344
3126152073 3152512175 46817652
4043302162 3842398574 56808498
3694349069 3493296400 336412801
770911368 661099877 57223725
3574254366 3032417472 120094703
2689829955 2124052738 139986329
3358352989 1925653441 3542661
154052576 1060760630 69725238
3460616091 3829709201 149081
881951467 480014985 9494517
1777535488 3915581401 135358522
2829816284 1290369026 84715328
478947030 489509502 171590375
2686813330 1226966401 3016625
3361895650 3199329827 98720441
1226966401 1375084354 550569087
3515610257 4050939923 58644109
650537405 926192552 120373963
4100110660 1929196102 194856636
1912894010 2708676161 323741311
3460765172 2264039067 54845085
898932898 0 10055892
891445984 265225108 7486914
2914531612 3298050268 195246132
2572921520 2318884152 113891810
961272608 272712022 155658520
908988790 1130485868 639375
2236635321 2432775962 121814855

water-to-light map:
2821176146 2286693663 106119314
3822234587 2463633329 180779736
1725724347 2842879211 104224606
3308097155 4172728180 122239116
3299768179 2834550235 8328976
525232540 357109336 38255672
751267412 803626289 867213460
2181067610 2392812977 70820352
2251887962 2644413065 134698828
3430336271 3994876090 163182805
357109336 395365008 62712446
3646455511 2158365540 128328123
3084834769 2947103817 214933410
1618480872 3954660777 40215313
1673365470 751267412 52358877
2927295460 3797121468 157539309
2386586790 1723776184 434589356
3774783634 2779111893 47450953
4003014323 3162037227 291952973
419821782 458077454 105410758
3593519076 1670839749 52936435
1658696185 4158058895 14669285
2173080221 2826562846 7987389
1829948953 3453990200 343131268

light-to-temperature map:
457330729 4090205185 204762111
2982196520 3401667644 30193953
2238727594 3778270640 263367024
2540710222 1921368253 380651678
2224576409 4041637664 14151185
1803946096 1170025919 125923944
947791690 3069412788 65888847
3354708582 3597834895 180435745
1173448701 516297801 630497395
4079424710 2899828022 88028778
1929870040 1295949863 294706369
928458849 495946333 19332841
3145865299 515279174 1018627
3071616223 2825578946 74249076
3012390473 3431861597 52676537
0 156654134 174053721
1013680537 1590656232 159768164
2502094618 457330729 38615604
4167453488 3484538134 113296761
4058703342 3048691420 20721368
3146883926 1750424396 164394644
662092840 3135301635 266366009
174053721 0 156654134
4280750249 4055788849 14217047
3311278570 1146795196 23230723
3334509293 4070005896 20199289
3535144327 2302019931 523559015
3065067010 1914819040 6549213
2921361900 2987856800 60834620

temperature-to-humidity map:
19014508 1616728169 261978440
479364011 879054632 183139707
3422279791 4197415651 97551645
2947838505 2859883311 474441286
3922771609 3441941550 130194267
1538347549 1285663854 77335299
4074125861 3334324597 17231539
2692139672 3923798143 165313419
3867887507 3572135817 45827004
4091357400 2695950683 148956394
280992948 874337342 4717290
3585981058 4137845928 59569723
1194193608 267159640 344153941
3645550781 2226309992 39998882
2606341883 3617962821 85797789
2452453972 3769910232 153887911
4279991062 2844907077 14976234
4240313794 4089111562 39677268
285710238 0 193653773
662503718 1585706204 31021965
4052965876 2266308874 21159985
2226309992 2287468859 226143980
0 248145132 19014508
3685549663 2513612839 182337844
2857453091 3351556136 90385414
1139702249 193653773 54491359
3913714511 4128788830 9057098
916232734 1062194339 223469515
1615682848 611313581 263023761
693525683 1362999153 222707051
3519831436 3703760610 66149622

humidity-to-location map:
3722067319 3568864729 46052123
761939125 1263883488 182519766
3952597071 3400791743 168072986
1928058 204065059 218803536
1797120632 863951513 248903371
3409129274 3614916852 109595510
0 1261955430 1928058
3854871689 2940386871 97725382
3518724784 3898809601 159455982
220731594 1446403254 100124613
944458891 1813379640 207908225
3194065032 4058265583 117770911
3059317673 3222157831 134747359
3678180766 3356905190 43886553
3375153977 3124864500 33975297
2940386871 4176036494 118930802
1152367116 2021287865 24736138
3311835943 3158839797 63318034
320856207 422868595 441082918
1593055573 0 204065059
4120670057 3724512362 174297239
1177103254 1112854884 149100546
1326203800 1546527867 266851773
3768119442 3038112253 86752247
";
}
