/*
--- Day 7: Camel Cards ---

Your all-expenses-paid trip turns out to be a one-way, five-minute
ride in an airship. (At least it's a cool airship!) It drops you off
at the edge of a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing,
wearing goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You
aren't sure what parts she's looking for; you're here to figure out
why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She
beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what
look like very large rocks covering half of the horizon. The Elf
explains that the rocks are all along the part of Desert Island that
is directly above Island Island, making it hard to even get
there. Normally, they use big machines to move the rocks and filter
the sand, but the machines have broken down because Desert Island
recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts
stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the
game of Camel Cards. Camel Cards is sort of similar to poker except
it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order
them based on the strength of each hand. A hand consists of five cards
labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative
strength of each card follows this order, where A is the highest and 2
is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

Five of a kind, where all five cards have the same label: AAAAA

Four of a kind, where four cards have the same label and one card has
a different label: AA8AA

Full house, where three cards have the same label, and the remaining
two cards share a different label: 23332

Three of a kind, where three cards have the same label, and the
remaining two cards are each different from any other card in the
hand: TTT98

Two pair, where two cards share one label, two other cards share a
second label, and the remaining card has a third label: 23432

One pair, where two cards share one label, and the other three cards
have a different label from the pair and each other: A23A4

High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full
house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes
effect. Start by comparing the first card in each hand. If these cards
are different, the hand with the stronger first card is considered
stronger. If the first card in each hand have the same label, however,
then move on to considering the second card in each hand. If they
differ, the hand with the higher second card wins; otherwise, continue
with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is
stronger because its first card is stronger. Similarly, 77888 and
77788 are both a full house, but 77888 is stronger because its third
card is stronger (and both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their
corresponding bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

This example shows five hands; each hand is followed by its bid
amount. Each hand wins an amount equal to its bid multiplied by its
rank, where the weakest hand gets rank 1, the second-weakest hand gets
rank 2, and so on up to the strongest hand. Because there are five
hands in this example, the strongest hand will have rank 5 and its bid
will be multiplied by 5.

So, the first step is to put the hands in order of strength:

32T3K is the only one pair and the other hands are all a stronger
type, so it gets rank 1.

KK677 and KTJJT are both two pair. Their first cards both have the
same label, but the second card of KK677 is stronger (K vs T), so
KTJJT gets rank 2 and KK677 gets rank 3.

T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first
card, so it gets rank 5 and T55J5 gets rank 4.

Now, you can determine the total winnings of this set of hands by
adding up the result of multiplying each hand's bid with its rank (765
* 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in
this example are 6440.

Find the rank of every hand in your set. What are the total winnings?

--- Part Two ---

To make things a little more interesting, the Elf introduces one
additional rule. Now, J cards are jokers - wildcards that can act like
whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker
even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8,
7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of
determining hand type; for example, QJJQ2 is now considered four of a
kind. However, for the purpose of breaking ties between two hands of
the same type, J is always treated as J, not the card it's pretending
to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

32T3K is still the only one pair; it doesn't contain any jokers, so
its strength doesn't increase.

KK677 is now the only two pair, making it the second-weakest hand.

T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3,
QQQJA gets rank 4, and KTJJT gets rank 5.

With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your
set. What are the new total winnings?

 */

#[cfg(test)]
mod test {
    use std::cmp::Ordering::*;
    use std::collections::HashMap;

    fn part_1(input: &str) -> usize {
        let inputlines = input.trim().lines();
        let mut cards = vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        cards.reverse();
        let cardmap: HashMap<char, usize> =
            HashMap::from_iter(cards.into_iter().enumerate().map(|(i, c)| (c, i + 1)));
        let mut handbids: Vec<_> = inputlines
            .map(|line| {
                let (hstr, bidstr) = line.split_once(' ').unwrap();
                let digits: Vec<_> = hstr
                    .chars()
                    .map(|c| cardmap.get(&c).unwrap().clone())
                    .take(5)
                    .collect();
                let unique = {
                    let mut copy = digits.clone();
                    copy.sort();
                    copy.dedup();
                    copy
                };
                let maxfreq = unique
                    .iter()
                    .map(|u| digits.iter().filter(|&d| d == u).count())
                    .max()
                    .unwrap();
                let htype: usize = match (unique.len(), maxfreq) {
                    (1, _) => 7,
                    (2, 4) => 6,
                    (2, 3) => 5,
                    (3, 3) => 4,
                    (3, 2) => 3,
                    (4, _) => 2,
                    (5, _) => 1,
                    _ => panic!("Weird hand"),
                };
                (htype, digits, bidstr.parse::<usize>().unwrap())
            })
            .collect();
        handbids.sort_by(
            |(ltype, ldigits, _), (rtype, rdigits, _)| match ltype.cmp(&rtype) {
                Less => Less,
                Equal => ldigits.cmp(&rdigits),
                Greater => Greater,
            },
        );
        handbids
            .iter()
            .enumerate()
            .map(|(i, (_, _, bid))| (i + 1) * bid)
            .sum()
    }

    fn part_2(input: &str) -> usize {
        let input = input.trim().lines();
        let mut cards = vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];
        cards.reverse();
        let cardmap: HashMap<char, usize> =
            HashMap::from_iter(cards.into_iter().enumerate().map(|(i, c)| (c, i + 1)));
        let mut handbids: Vec<_> = input
            .map(|line| {
                let (hstr, bidstr) = line.split_once(' ').unwrap();
                let digits: Vec<_> = hstr
                    .chars()
                    .map(|c| cardmap.get(&c).unwrap().clone())
                    .take(5)
                    .collect();
                let (unique, njokers) = {
                    let (mut unique, jokers): (Vec<usize>, Vec<usize>) =
                        digits.iter().partition(|&d| *d != 1);
                    unique.sort();
                    unique.dedup();
                    (unique, jokers.len())
                };
                let maxfreq = unique
                    .iter()
                    .map(|u| digits.iter().filter(|&d| d == u).count())
                    .max()
                    .unwrap_or(0)
                    + njokers;
                let htype: usize = match (unique.len(), maxfreq) {
                    (1, _) => 7,
                    (2, 4) => 6,
                    (2, 3) => 5,
                    (3, 3) => 4,
                    (3, 2) => 3,
                    (4, _) => 2,
                    (5, _) => 1,
                    (_, 5) => 7,
                    _ => panic!("Weird hand"),
                };
                (htype, digits, bidstr.parse::<usize>().unwrap())
            })
            .collect();
        handbids.sort_by(
            |(ltype, ldigits, _), (rtype, rdigits, _)| match ltype.cmp(&rtype) {
                Less => Less,
                Equal => ldigits.cmp(&rdigits),
                Greater => Greater,
            },
        );
        handbids
            .iter()
            .enumerate()
            .map(|(i, (_, _, bid))| (i + 1) * bid)
            .sum()
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 6440);
        assert_eq!(part_1(INPUT), 249483956);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 5905);
        assert_eq!(part_2(INPUT), 252137472);
    }

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const INPUT: &str = "T6JTT 716
8J29Q 523
486TK 358
3AAAA 945
J5552 239
38AJ8 703
K8Q88 653
7634K 905
A222A 390
AAJAA 317
288Q2 825
KQ77Q 107
JJ7K7 249
QQ4JJ 519
433J9 411
T6TTT 304
K59KK 178
QQQTT 524
AQKQ6 21
Q8886 853
79J6J 84
33Q3Q 740
TAJA2 998
464A4 584
55667 686
QTT6Q 810
A9799 708
5643T 157
353AK 319
228JK 466
44TTT 760
85637 553
9K9A4 897
A9J8A 334
QKQQ5 600
8QA5A 200
33823 273
J228A 345
65668 986
KA8K7 101
4A735 791
55A55 901
7727Q 301
889J9 494
44848 134
98298 408
83KK3 562
T48T4 587
46626 190
J425K 568
787JJ 739
T8JJQ 573
62J8K 976
9JJ99 823
7777J 860
J3KA2 1
65646 843
78877 316
JJ888 614
527KT 765
AA9AA 124
4774A 370
AT97Q 727
4J274 870
99J79 834
72479 387
T6343 623
7534K 426
226A2 194
3TQQ8 625
T3T33 492
8J472 895
JKKK8 641
66596 535
494QA 887
99333 388
JQJ55 793
3A373 179
85588 948
AATAA 3
52827 570
KKJ33 593
53A4K 246
JK3TQ 293
32442 530
434A9 842
KK5K6 154
49J48 714
79TA5 199
7K79K 542
T9TTT 916
92994 811
66676 306
K5JK5 490
59AQ6 285
A5A33 32
22282 833
982Q9 181
88388 63
QAQQ5 522
67454 717
8JT33 855
84888 997
2J2TQ 392
JTT8T 34
9399Q 457
687J8 541
2Q22J 391
66KK6 29
KQ2K2 350
777QQ 652
2TT33 911
T7272 250
3846T 375
KQ435 502
44333 96
Q8Q5Q 177
6668J 106
88262 772
JKAAK 170
5Q555 235
7666A 232
TJ777 123
K3K87 415
996Q5 113
TT555 683
2992J 707
67787 226
AQKJ9 909
A97KJ 926
Q2JQ6 109
6TA94 508
9K99K 993
KJK4A 732
56J28 598
88544 751
63577 39
3T4KK 382
KA948 747
3JTTQ 588
QAQT5 629
TTTT2 13
JA5KA 942
22QTT 114
A8A6A 186
3AA7A 832
K224T 912
5TKTJ 808
KJ3T9 947
3TJ43 55
25229 4
7QK25 696
T9JQ4 126
88577 288
J28K9 187
75579 621
KJ8QK 846
QK4T6 356
K574Q 478
9JJJ9 500
6J343 176
96666 734
T8JTJ 159
66J76 658
87A4T 308
66J66 725
A3Q44 520
Q668J 946
J22TJ 874
Q6466 719
325Q6 422
33T33 486
4AA2J 171
44297 512
92399 569
552TT 513
66J69 831
TKT99 768
44448 464
K4A44 787
JQJQ3 979
79QQQ 204
3KKK3 699
4T458 501
J894K 10
88K8K 260
77775 58
7J788 335
99969 401
82594 41
J98Q8 921
222JK 443
6549J 525
43423 436
9J949 908
A8Q95 555
4A235 394
57535 236
22J22 618
28QA6 326
74455 792
AATJA 477
4AJA4 960
4649A 264
2KK55 405
44J34 809
6J545 964
23323 36
AA266 476
9Q99Q 620
2J8T3 940
99997 851
62ATT 270
8JAA8 817
8A334 127
K4368 788
66Q23 148
672KJ 480
2K59J 68
36Q6K 69
35335 515
AAA33 321
K9474 670
49848 951
55J55 840
4T444 320
4Q89A 78
6K6J6 66
4J5TA 131
T958K 31
2Q74Q 928
AQQAQ 71
27598 169
J5995 287
K4T48 820
K55K3 532
22A32 25
272T4 576
668J7 440
Q9QQQ 742
AQ9TK 929
J76TK 968
9J66A 208
68QKQ 213
33373 82
JA2A2 859
25K2J 424
AK33K 247
2835Q 309
26262 702
QK3QQ 234
KK4K8 818
8T7TQ 611
46TJK 953
9J699 949
TT44J 757
92A77 459
47477 784
J9JAQ 245
65JKT 377
65J56 991
6QQ6Q 315
TJA9J 423
J5455 141
94949 565
QA26K 447
99Q99 117
2J779 824
KQ9Q4 505
KKKKA 110
TT83T 977
K55J7 826
44QQJ 795
Q4Q44 380
35333 189
6Q878 774
66336 746
T2222 781
8K888 445
9964Q 715
88J8K 104
J9999 995
8Q77T 351
426K5 5
K9K9K 581
9T22T 944
A45A6 927
27J27 147
47474 258
Q78QQ 983
77797 389
Q4Q64 384
AQKJ3 279
22Q35 52
3K3J5 759
37JQQ 129
QA688 559
J8JJJ 271
9365T 627
4K9K2 962
3TT3T 894
8Q7JT 328
2552Q 694
4K98K 789
2T555 45
25469 721
83TTA 982
272K2 140
6T235 816
KAAKT 580
TT89T 586
46A67 920
63686 959
8K457 965
TTJAT 57
6K6JK 780
842J9 455
JKKKQ 654
77232 175
4A444 830
9KA99 302
QT6TT 324
Q88Q8 402
24937 575
AA96A 357
98599 799
97499 917
JAT87 296
J474Q 904
A88Q8 722
55223 192
Q5939 504
622J2 649
JKKKK 50
AJ3Q9 160
58AJ2 753
452T7 277
JATT4 589
33363 841
64747 661
2KK2K 764
JTJT3 709
AJJA3 850
K6T59 48
929Q7 626
5A6TK 156
K26T9 161
TAAA2 307
3TTTT 136
Q9Q5Q 120
T5468 289
44454 647
AJ333 105
KK63K 521
J8388 952
99T9T 139
53232 821
484J3 26
66J85 790
2228Q 27
TT737 212
55754 470
5Q666 943
6J48T 578
8AA66 941
33323 215
A22TA 680
QQQTQ 755
62222 882
A964J 371
8QQQJ 679
A99AK 497
2KJTJ 481
T8Q6J 479
799AA 379
4T672 880
448T8 607
66868 329
K66KT 975
44664 974
J4423 1000
66922 488
T335T 115
T55T3 28
3A9A3 431
57575 891
74AJJ 773
QAKKQ 534
A74A7 984
3KJ33 603
QA7JK 460
87728 331
7TJ7T 354
99995 432
A5T25 97
55T55 242
T8669 410
J8A2A 228
9T7J7 318
Q4JQQ 805
88886 360
6T75T 552
QAAQA 310
T355Q 149
QQ8QQ 622
9QQQ9 22
K9849 257
9962T 672
999K9 981
JJ7T9 640
55JQ5 230
99T99 735
QK23J 599
4Q774 866
AK9JA 17
T6Q2Q 720
Q8K46 474
5J96T 280
4A5TT 499
T4QTQ 776
33977 263
T5429 60
575J6 954
ATTTT 491
T656T 376
J7A7A 971
JJTAT 879
A6686 385
94999 8
QQ99A 229
958TQ 248
68597 583
8885J 591
24355 963
JT5A3 251
95993 676
QJ556 80
3K8K8 881
96892 803
99Q9A 838
25K8J 441
444J4 752
J3J33 634
2A26A 907
3J3T3 561
67A47 217
8273K 303
TAK9K 967
476T7 711
4J3T9 632
48Q9Q 173
33K36 737
3QQ44 433
JTTT5 845
5555K 869
64K82 238
28288 689
869J7 83
777K5 314
9KJK9 617
54544 844
44456 723
337QJ 397
4K6Q2 413
65695 980
KTJT3 262
6KQ35 543
Q4Q77 462
5952K 900
AJAJA 861
T57TJ 992
K55QA 681
JTJ73 366
999QT 77
TKKJK 343
44242 330
447AA 241
AKKKJ 786
73243 152
343KT 227
95A59 495
A25Q5 9
29923 222
ATA8T 615
6JK7K 163
5TTTT 876
692A8 514
QQ733 338
6Q5JA 985
76Q53 987
JJ4J2 517
T829T 744
69969 966
22K26 442
8J833 467
8T889 763
5J947 955
6JQQ8 218
J83J5 518
K5KKK 243
459K2 463
QJQAJ 572
QJKQ8 730
K4Q56 969
Q577Q 868
88A86 619
23T9K 678
A4AA4 313
K4KJ3 93
474A4 918
5Q5Q5 972
676T7 931
8A282 297
44945 650
Q9KKQ 197
8567K 254
QAJA9 778
A4J77 718
A7457 294
37Q89 342
JTTJT 18
TT22T 240
773T3 325
46K62 64
TTKJK 164
33K33 406
8T8QT 704
7QK27 506
9K3KK 836
T55QT 577
3Q333 871
99QQ6 348
QQQQ3 225
Q3Q3Q 724
99AQA 327
883Q3 749
A2A2A 551
J655A 656
999T2 446
QAAA4 407
33738 361
555AJ 865
3JJ36 437
TT7QQ 73
QQ8QK 162
5J533 119
A8A22 283
33J33 896
2KQ83 33
6K753 690
97397 155
67773 130
A438J 166
KQK74 195
Q362K 839
Q6QJ6 373
9992Q 794
9K43T 487
K5QK5 566
JJK9K 574
J2T2T 544
T6T66 300
2J288 403
K43KK 858
5QKK6 852
69J37 132
554Q5 655
37777 182
T6TK4 265
56666 383
T6856 538
832KK 484
9JTQ3 819
3J393 458
JK6JT 122
K2222 59
98299 914
J8TJ8 332
62JKT 815
889T5 922
477J4 378
7TA4J 62
53TA4 453
44434 399
55J6J 669
T9559 81
78777 349
6446J 207
AK8JK 11
TJ544 782
56JK7 667
Q9KQT 269
K86Q7 430
K6K6K 475
75K46 651
42Q99 684
4823J 125
Q44JT 567
4Q444 362
75A24 75
K5AKA 701
J9TJT 592
KKK88 337
Q7K77 417
AA44K 2
822J2 546
95JQ4 498
Q2JQJ 496
QQAKQ 937
Q99Q3 564
J88KJ 465
56566 268
72777 193
7A747 90
9K8QA 674
A6AK6 883
38383 211
K9KKK 713
627QQ 305
2A32T 340
JTTTT 116
92555 783
34TTQ 471
96299 712
324J3 590
4444K 726
9T988 420
QT279 537
6A77A 585
QQQ28 758
Q4Q24 536
JT886 903
T88J8 516
3JA3Q 368
9Q9KK 639
84QJ9 867
6593Q 6
8A5A5 95
A6A66 43
373K7 85
7A737 108
88J88 738
7T6K8 412
88887 970
585J8 180
333QJ 23
T5TT5 829
A5755 54
2J2J2 355
94223 666
995J9 608
77JAT 973
T9J8J 893
J49A9 503
A52K5 692
9Q638 527
QQQAQ 472
66677 835
KT2A7 451
29TQ2 606
QAQ8Q 631
5J5J5 99
49KQT 419
TAATT 210
TQQJ3 282
TTKKK 736
82822 710
92669 344
6AAA6 205
TA9JT 367
6965Q 168
4T5TT 728
A73A3 933
37J94 657
3K873 994
59Q33 364
A78QA 571
Q5487 449
JJ444 597
JK2K2 216
TTAT6 206
QQ767 341
757Q4 30
J68TJ 932
KJT45 65
T88JT 244
JJ922 237
A5Q55 779
89899 733
AAJ4A 812
4QQQQ 172
89898 16
99J77 20
66899 223
99A28 395
8T8T8 272
K8282 862
5A6Q5 14
55A77 369
QQ366 837
J7J2T 828
Q7948 429
A7A7T 890
3TJ8T 637
J6JQK 336
T4322 550
6JQ73 224
66JAA 284
343J3 531
7569T 292
247Q2 252
22522 165
7QJAJ 961
KQJJK 743
K4828 15
K5T22 646
QKKQK 630
T3949 756
655J5 596
AKJ83 100
7Q939 266
T4TKT 856
32337 705
J4Q4A 769
J6747 469
22T29 468
2A555 448
5T95J 255
AAA7A 540
5575Q 664
8AA88 347
JQAA8 323
KQQQK 807
J8643 766
J3399 616
999Q6 24
Q55T5 695
8K887 128
J8559 40
99229 624
22267 750
8TJ3K 202
69755 89
6Q8K6 802
AKA7K 322
9JT99 461
7JJ77 729
K825Q 151
55557 118
K96AQ 548
49797 363
974QT 604
327Q6 875
26KQT 978
8K82J 146
9Q4QQ 748
65JK4 510
3A6K5 49
2Q2QQ 72
K234Q 798
J5575 220
6JA46 56
QQ2QQ 638
6J39J 957
99595 700
9TQ9Q 601
79737 12
JK787 74
QAAAA 873
75588 673
QK6J5 111
93TTT 209
A9A9A 511
KQQQQ 804
Q8858 602
55525 112
ATJKQ 767
77557 88
322AQ 671
T78JT 454
22768 633
98459 91
42A9J 473
7QQQQ 365
5K4KT 233
55355 936
Q3A6A 274
49444 339
T79Q3 44
69632 298
33QA5 785
44244 144
7A579 642
38245 863
QJ2K4 813
22582 706
44433 582
77K77 770
5T432 877
26TTT 439
44A3A 956
3T33K 450
TJQTQ 435
3Q279 682
5KTK5 872
85555 51
J6534 489
67272 434
J2777 800
77KJK 889
77767 352
22QQ2 528
7J4T6 775
9666A 203
42AA2 35
33338 915
99A2A 174
T878Q 253
979K9 806
7AAQ6 636
A8Q8A 698
T7T7Q 754
5A453 906
AT997 628
A6666 482
4294Q 53
66626 884
6626J 801
JJJJJ 145
T4TTT 86
KT2J2 659
5KJ55 563
5935J 848
6Q7TA 137
592KQ 493
QT2A8 92
3KJ72 919
5827T 665
KKKK6 797
6TKT6 643
59QJ2 416
57567 635
5QQAA 645
84722 428
QQ5QQ 103
4KJK9 188
QTTTT 456
K23KK 485
9QQ4J 950
63667 295
8888Q 143
75A76 693
T8TTA 996
7A7KK 94
3KJQ9 796
6J6T6 999
929K9 612
29292 549
8AAT3 771
78T36 777
845AA 38
KKQ68 438
K47JK 185
A92Q7 167
777TT 418
878T5 509
2A2KA 849
TT24T 133
85K8K 930
55J53 256
393T3 675
KK72K 662
7T529 135
TTK77 275
5J595 138
J8577 827
6J466 444
AAAA4 276
T7T3T 183
JQ9Q6 529
38AAJ 98
832TT 221
KTJ82 864
22392 214
58J29 935
7JK57 885
49K9T 333
JKKJK 359
94A5T 579
AJQQQ 291
J776J 121
2T535 847
77373 267
76886 892
TTJ45 483
TQ84A 595
43J34 745
QQ9QA 924
92222 452
74K3Q 286
84383 19
42J24 697
22223 198
69A3K 142
66686 663
6672J 201
44J6J 610
58988 46
77J79 425
T2889 609
677J6 925
AJQQA 393
JJ838 290
4JKK5 613
83KQA 312
QQJQQ 231
45455 7
9Q7TJ 556
8KJ48 42
A978Q 346
2QQQA 398
T5999 37
82QTQ 939
TKTTT 526
T4885 353
6J868 691
565K9 910
TT887 259
63982 913
J8J79 687
88AA7 404
29924 414
69Q2A 660
TTTK6 219
A87AA 648
QJQJQ 761
22366 158
26942 605
47T36 261
2K267 888
7K7KK 857
9Q9QJ 668
Q75T5 898
832TQ 822
9KAT5 547
6Q666 87
52992 372
Q6Q6T 102
J6J66 886
53Q5A 79
938AA 70
KKK2K 902
777JK 67
64766 554
6J833 990
425A2 427
AATTA 409
JQ88Q 677
49J44 741
A4T98 560
8A357 762
T974A 533
576TA 76
66Q6Q 61
KQT3A 381
5384K 989
T4T59 400
473K3 899
45A4A 386
T5ATJ 958
34QJQ 878
44644 278
46K43 934
33QJQ 153
36QQQ 545
72757 184
7847J 644
66AA4 539
7JA3A 854
TTT77 594
T5Q23 47
6446K 150
285AK 196
J28AK 421
T6J2T 507
39334 685
8QJQ4 558
K222K 557
6J66A 731
AAJ5A 988
K9456 311
Q3J8J 396
TKQ56 923
J7775 299
824J5 938
TA22J 688
J99TT 814
K4K4K 191
JJ33J 281
67QT9 374
";
}
