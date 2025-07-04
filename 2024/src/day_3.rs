/*!
--- Day 3: Mull It Over ---

"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're
welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole
Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All
of the instructions have been jumbled up!

It seems like the goal of the program is just to multiply some numbers. It does that with instructions like
mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result
of 2024. Similarly, mul(123,4) would multiply 123 by 4.

However, because the program's memory has been corrupted, there are also many invalid characters that
should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or
mul ( 2 , 4 ) do nothing.

For example, consider the following section of corrupted memory:

xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

Only the four highlighted sections are real mul instructions. Adding up the result of each instruction
produces 161 (2*4 + 5*5 + 11*8 + 8*5).

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the
results of the multiplications?

Your puzzle answer was 165225049.

--- Part Two ---

As you scan through the corrupted memory, you notice that some of the conditional statements are also
still intact. If you handle some of the uncorrupted conditional statements in the program, you might be
able to get an even more accurate result.

There are two new instructions you'll need to handle:

* The do() instruction enables future mul instructions.
* The don't() instruction disables future mul instructions.

Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions
are enabled.

For example:

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8)
instructions are disabled because there is a don't() instruction before them. The other mul instructions
function normally, including the one at the end that gets re-enabled by a do() instruction.

This time, the sum of the results is 48 (2*4 + 8*5).

Handle the new instructions; what do you get if you add up all of the results of just the enabled
multiplications?
*/

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use regex::Regex;

    fn part_1(input: &str) -> usize {
        Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)")
            .expect("Invalid regex")
            .captures_iter(input)
            .filter_map(|caps| {
                caps.iter()
                    .skip(1)
                    .flatten()
                    .map(|cap| cap.as_str().parse::<usize>().expect("Cannot parse integer"))
                    .collect_tuple::<(usize, usize)>()
            })
            .map(|(a, b)| a * b)
            .sum()
    }

    fn part_2(input: &str) -> usize {
        Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)|don't\\(\\)")
            .expect("Invalid regex")
            .captures_iter(input)
            .fold((true, 0usize), |(enabled, total), caps| {
                let mut caps = caps.iter();
                match caps
                    .next()
                    .and_then(|cap| cap.map(|cap| cap.as_str().split_once('('))) // Flatten the double nested options.
                    .flatten()
                {
                    Some((op, _)) => match (op, enabled) {
                        ("mul", true) => match caps
                            .filter_map(|cap| {
                                cap.map(|cap| {
                                    cap.as_str().parse::<usize>().expect("Cannot parse integer")
                                })
                            })
                            .collect_tuple::<(usize, usize)>()
                        {
                            Some((a, b)) => (true, total + a * b),
                            None => (true, total),
                        },
                        ("mul", false) | ("don't", _) => (false, total),
                        ("do", _) => (true, total),
                        _ => (enabled, total),
                    },
                    None => (enabled, total),
                }
            })
            .1
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE_1), 161);
        assert_eq!(part_1(INPUT), 165225049);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE_2), 48);
        assert_eq!(part_2(INPUT), 108830766);
    }

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const INPUT: &str = "@~don't()mul(683,461) >,~select()what()};<mul(848,589)!#{$$:,#mul(597,936)]!how();)+mul(944,148)who()mul(84,922)what()(mul(95,23)[;]*mul(186,673)$@+,;mul(662,571),^^&&mul(467,635)]>what()*,why()mul(456,228)%/'where()~{mul(508,422)mul(78,184)&why()/(mul(373,546)why()*</mul(840,607)(:how(428,64)>why():-~{@mul(615,760)}?/*>$mul(422,670)},?#''how()@#mul(597,259)??mul(281,991)when()why()]/;}:who()why()/mul(864,121)how()#{[from()&mul(336,536) what()/what()#:>mul(908,667)[%^>&]select()*(mul(716,31)what()[+~what())]where():mul(131,416)what()from()]^when()$mul(808,465)from(747,493)*@;mul(728,405)@mul(528,186)where(830,813) mul(138,404)&&$,,mul(220,842)(from()*~from()how()where()mul(44,647)~where()}%$+mul(523,78)}mul(506,925)where()) ?mul(650,260)what()))when()mul(992,831)*]?-(mul(382,514)<?mul(338,811)~who()<)mul(510,851)[mul(362,413)^why()how()mul(456,506)why())},)*where(294,226)do()<when()>@<>,{*<mul(695,394)from(){/~mul(192,894);how()where()what()who()!:mul(932,681)@select()(where()@{?^+mul(260,501)from()+select()?^mul(445,400)@:;>^/why()[don't()#>how(558,115)mul(303,593)${~from()%*$%do()-+[$mul(406,365)how()mul(561,819)how():select()[:when()mul(283,930)where()$>why():<from(){who()^mul(5,405);%#what()[(*>#mul(154,725)from(496,796))@[how()from()- mul(942,491)-don't()~who()^)#select()where()mul(880,650)>(when(),)who()]where()mul(735,764)<-select()where()~#mul(685,917)from()!how()&who(),~why()[/mul(838,662) }what()from()from()<mul(25,821)]$+?)[>[mul(217,645)who(){who()mul(712,250)*}~who(133,126)where()}what()-where()mul(879,905)mul(95,8)~{?%:/mul(885,299)mul(509,215)~mul(97,351)mul(11,228)mul(976,139)where()mul(532,340)mul(535,236)</@)+mul(91,686)^+)(/}where()why()(<mul#-+!%(mul(23,448)!{:}#,mul(761,358)select()<mul(557,974)<({select()& why()]'mul(940,978)from()*why(871,616)$[mul#mul(652,248)what(455,217)-$<)'!{who(),mul(530,568))<;[}^do(){where()#&$};,}mul(903,867);-#**[/~mul(698,27),~mul(701,325)where()#who()>)'mul(544,423)@@ mul(543,903)why()mul(610,22)where()when()^)mul(529,383) ;,/'when(179,924)mul(31,223)when());<mul(812,214)mul(375,504)]:(~& !who()[~mul(192,595){~[{what()who()'from()mul(127,309)who()mul(555,543)%/%!~select(),?[from()mul(832,869)mul(808,878){[@mul(301,808)?>*from()mul(810,928) ^'{'?~mul(840,456)?where()}~:who()'do()@where(){(@$select(471,179)mul(490,95)where()?>)<mul(355,525)from(971,891)+select(){mul(247,601))%how()[;~select()mul(960,14)? #}/~from()why()%from()mul(507,980+from()#@@mul(50,394)how(751,319)who()?~what()'when()$}-mul(791,444)))!#/>^$mul(192,331)?when()where()!}who()*%[%mul(778,800)+}when()who()how()mul(836,264)from()+)@where()mul(261,535)&)@why()-#/(mul(187,2)(?what()mul(467,281)when()where()#, [+what()select()(mul(232,619)don't()^mul(406,760)mul(400,865)%%who()where()}${?[!don't()#%@^/)when()]@%mul(268,776){-:how()}:'~+mul(817,838)}when()when()#/where()?how()+mul(712,30)select()(%* !mul(11,366)
 select(463,746)]mul(475,757)where()$ }~)<mul(467,155)<how()why();&mul(656,291-{>}'mul(201,678)what()when()]',who()how()mul(228,56)^'what()from()@$,+{[mul(725,736)mul(96,310)*:<>how()from()<when()mul(729,298)']~when()*mul(376,300){select()mul(385,844) %/[-'mul(831,34)<mul(397,823)#,,~mul(594,367)how()-)why()when(30,465)^why()^mul(273,629)@^?]!]mul(545,532)select()where()+where()<;^mul(321,301)!~~why()mul(98,108)[why()){$)mul(857,959)don't()when()^mul(417,755)/who();don't()how()?how()%@mul(274,441)~mul(562,453)<;}-from()@;$how()}mul(186,62*+[@'^%mul(995,463)?!!select(883,304)/^;{-#mul(964,420)@?%select()when()mul(89,676)(^;~<'@!mul(280,290)[?when();#}~{from() mul(5<(when()when())]+<where()mul(603,617)[mul(959,445)&<(#(where()!from()]mul(493,785)^-*/when()mul(268,895)-?:(~mul(379,592{~! [?-mul(822,591)from(295,31)<- *)(mul(757,754),)!who()where(),mul(386,923)@$what()[from()mul(984#what():&who(543,949)mul(514,887)!@when())from() :,%!mul(26,604)why()why()#when()from()mul(664,312){&do()mul(988,752):' ^:where()>&{>mul(533,345)>where()>%when()mul(816,202)#what()]+/}%when()<]don't():*%;mul(368,570)~from(251,737):^ ;select(589,302);~mul(780/what()/{&when()mul(780,721))';+>mul(191,651)#}why()!when())mul(321,579)%~ what();/!who()mul(707,979)!select()when():+<& do();>what()}what(318,824)mul(755,111)%/(?*}#how(431,277)mul(85,134)mul(280~<$:who()when()where(174,374)mul(113,173)>don't()^(who()(mul(765,356)!/$? select()-%+?mul(991,23)?why(270,631)when()!:)/&mul(782,296)(~['+*?don't()@)+?mul(553,968)how()what()%where()<when()!mul(44,677)how()select()'how()*,select()(-mul(247,522))when(),'mul(459,682)/why()&]' mul(917,91)]how(51,797),/@?who()when()[}do()*+!(mul(752,850)}%mul(750,273):#)'?why()@?mul(974,149)?what()>]where()&&mul(621,835)when()/:what()@,@where()>mul(317,451)~why()+where(),))mul(364,198)+what()}*[mul(883,802)]/'where()mul(387,43 where()~?;)@@what()how()mul(322,326)who()<mul(30,534),}%!{how()!%what()mul(610,849)who()]why()-mul@-}!)mul(108,806);:{/% mul(486,125)select()[!/^!{&mul(758,458)&{,how(){@&mul(46,465)%where(845,173)do()why():'+'?-)~mul(822,408)]]#from()!?+-select()mul(905 /from()+@(!mul(327,879)(+;;>+mul(326,133)/(]%,? mul(907,688))-(select()#mul(666,327),!who()mul(895,291);&)^)[mul(658,676)@who()/+>&mul(44,325):^>why()who()$*select()mul(641,527)mul(524,670)~mul(25,367)mul(499,446) ,why()who()%mul(100,283)#when()who() how() @&mul(351,89)why(36,81)'!,select()>who()!who()/mul(52,820)/^mul(630,788)from()%<(^:}mul(420,483)mul(473,512);*}-mul(761,735)'mul(419,165)&mul(523,446))mul(834,76)!#mul(562,151):-'who(285,490)[mul(984,224)'!;mul(469,881)>**-'mul(385,297)@mul(994,97)+*%mul(281,326)@//$mul(664,734)how()select()[:@?#,do() !)%what(202,601)+how()from()mul(391,934)!%?)how()mul(825,779)how()([how()do()who()+who()#%when();mul(642,60)from()&mul(425,517):what()mul(675,402))?@/how(668,137)from()-mul(400,170)mul(808,291)<!)'[?mul(284,214)why())mul(684,417) from()who()$#[$><mul(154,319)^why()who()$@}[<-mul(542,516)when()when()^'#do()what()where()mul(986,109)from();$do()from()&:who()mul(865,597))what()~why()>{how()mul(142,403)
mul(868,598)@from()%>&?;@mul(20,94)&?why())?mul(758,315)~&;where()'what()mul(152,875)mul(308,470)][select()$&-:>?mul(256,361)<')>&from();{why()mul(620,603):{~where();' -mul(693,340)$mul(646,441)-what()do()%!when()->mul(617,502),#,]mul(212,569)$when()])mul(635,595)~}:@*mul(292,790)}mul(933,20)@^{(mul(22,237)&'mul(103,788){}{why()who()%what()don't()(,#'(;mul]when()mul(268,616)how()mul(675,612)>**@$mul(111,445)$:!how()mul(56,5) mul(419,542)why()&@mul(214,101)]how()from()why()why();^where()mul(841,681)<when()mul(209,671)%select()%^where() :from()mul(76,245)+>)$',#%}&mul(777,293){mul(778,822)><!##why()!#mul(698,922) #!%;:how(188,944)@mul(593,401);+who()where()##mul(17,157)%)$+mul(949/}?]',mul(693,839)}}>*{{)'from(644,961)why()mul(559,996)*how()<(mul(41*!:select()%#''+what()mul(193,482)$;%([]-*mul(271,561)mul(632,842:^+/%)mul(605,351)<$}mul(80,304),!(]$/when()-when())mul(942,823)~^,,}#'mulhow()mul(558,938))+how(),mul(345,610)<what()$)how()]#mul(217,743)[ %when()when()?mul(624,634)-*&mul!&; &)mul(168,19)#{{}>]%<(mul(550,971):,#)mul(468,45);$:how() *(>mul(354,420)/:+how()#::mul(71,315)&mul(378,96); +*what()[when()]):mul(486,242)+(@from()'-^(}mul(727how()? }^/^mul(470,146)from()'why()how()<~,+-mul(522,547)#**?+:what()who()from()>mul(793,81)$)'~what();do()]how()select(432,392)+;-[mul(997,671) where()?mul(129,744)select()why()<+mul(567,796)from(817,482)@(mul(462,463)/)<mul(678,95)&?+what()#?mul(360,496)(what();mul(728,595)mul(605,288)where()select()when()mul(563,994))&+'mul(934,978)^*when()do()mul(218,50)why()what()( %mul(905,176)do()mul(559,597)?,when(52,254)who()why(486,729)mul(732,462)$]>!why());mul(893,320)@;&)why()?why()when()who();mul(731,381)when()/:how()%mul(999,422)where()from()]select(739,491)#*;mul(435,663):mul(872,159&/&where(70,34)};where()from()mul(518,656)>[*who()<{when():don't()(mul(616,333){@/why()?what()*>mul(823,123)how()mul(113,251)from()~('mul(894,176*when()#/*]>mul(542,271)when(159,532)]:~from()mul(629,280)from(676,859)[&&!]mul(980,396)where()}*' ^!how(298,757)mul(593,718#)~do()>!{what()';what()-mul(417,931)mul(379,961)#>!}-- mul(770,269)-]~&'*mul(179,685)how()why(){;^mul(515,206)+^$#]- ;@mul(854,862)-%,mul(217,603){why()^select():where()*mul(458,549)where()!% mul(336,105)from()!};$why()%[~why()mul(701,97)mul(566,796)'/}!mul(987,970)why()/ where()?*why()mul(938,415)(#^>(mul(358,974why()>mul(127,685)]what():mul(23,486)!+[}!mul(703,30)when()'%,#mul(182,500):mul(13,248)^-[,]what())'who()mul(150,306)>/mul(353,581);%:where():mul(296,383);%why()select())][ /mul(185,486)~#mul(383,477)mul(484,779)/select()what()#%+from()]'why()mul(410,683,mul(892,307)<-~,~mul(463,990)?#>-+mul(500,467)~~mul(41,729)-^don't() mul(478,617);,{$mul(722,601) when()who()>>&mul(107,557)}why()-why(){select()mul(963,953)+mul(56,679)/#!/usr/bin/perl(+where()?~%when()<mul(591,840)
mul(120,615)mul(290,327)@mul(908where()#(when()(-~mul(577,101)why(715,329)mul(176,185)where()[#how()mul(750,233)[mul(159,147)~)who()select()+who()why()mul(742,864when()mul(427,119)}]*%@$}how()mul(480,127)$^]}how()from()[/>,mul(415,271)mul(681,988)</~~/where(),&:,mul(130who()'mul(551,234)'what():'!@why()+!mul(710,188)^select()mul(59,292)&;^why(359,678)?>??mul(244,420)&mul(348,572)when(746,716)/]^why()what()mul(476,900)];what()mul(203,688)why()*#who()when()+(why()where()from()mul(362,853){#'!%%mul(606,167)select()mul(928,445)!+<mul(275,155) ?'$mul(226,200)do() ;$ ):&when()~mul(495,155)-]~what()(mul(388,132)+}-mul(816,927){when(50,691)select()[[mul(157,67)^,-when()/}<:mul(273,650)^mul(459,438){)who()mul(403,758)how()mul(163,436)^([~-how(){what()mul(550,927)#$<&%mul(883,534)where()+mul(964,832)}mul(68,704)how(987,646)()how()what()when()*'mul(309,481){{where()<how()%mul(501,81) 'why()mul(925,35)!what()+@mul(249,565)[(mul(396from(604,6)$)mul(206,227))when()>'select()mul(41,136)mul(245,634^who(419,312)#/'<)'%mul(804,572)'%+' who()+@mul(913,22?;+-from(278,530)#how()mul(141,596)@>select(540,148)!]{mul%!-@#&}mul(889,958)mul(485,341)~%+how()?select()#mul(777,805)'how()!mul(266,524)<+)when()$<when(55,561)/mul(304,989)~}<'mul(298,616)who()how()mul(436,599)!what()+'#[mul(430,767)*/&mul(949,856)%>{mul(880,580)select()&who()~#)mul(486,513)why()when()%;how()#mul(937,105)*mul(186,474)mul(420,471) what()[>,%)]mul(862,899)(&}]{+$mul(871,579)$why()#}#mul(125,369)when(271,818)<''}-select(){ don't();#who(){-#mul(268,334)^:%do()mul(75,572){what()mul(283,605)mul(919,23){;)what()~who(445,781)from()&>mul(294,330):*from()who()#from(868,274)-[mul(47,694)mul]$when()$+mul(747,604))!~;mul(588,239)what()(^(>}mul(376,745)why()]where()why()/who()mul(265,949)mul(889,524),,{what()+{/*mul(946,665);?*?;~,mul(738,941)&]why()]~^~where()*mul(865,982):',mul(146,728)~mul(399,437)(do()mul[why()<!$mul(538,185)/@({where(){%]^mul(687,547){)/%?>from()who(),:mul(277,862)from():'mul(367,848)where();why(),(select()-when()who(){mul(833,730),+select()mul(196,278) ?}}[!^^mul(825,271):select()&@~^+<mul(254,892/*),when()]&why()mul(456,902)$?when(119,330)'?why()$mul(862,499)'>why()<from()*'mul(993,416)what()>%mul(858,60)};how()}^mul(285,548)mul(989,956)$mul(849,412)}>/:>/@mul(803,749)select()>,#mul(270,484),mul(49,218)%{;/>select()'+mul(645,947)select()*?do()' >@'~;mul(222,599)#@from()where(),what()mul(989,460)?{)<$mul(444,901)<@'mul(402,13)/!*mul(657,625){#when()what()mul(762,228)#mul(704,630)@&{??!//?/mul(104,95) mul(337,388)#^ )who())where()mul(623,138)$mul(403,157);%mul(101,259)>what()don't()>mul(998,640)mul(448,211)?*!select()who(),? )mul(271,985)$}--~&mul(234,431)from()+where(891,795){$+:~)mul(11,379)
why(872,627)when() @who()&why()mul(585,812)-+#%(%)mul(774,83)where()<mul(895,861)mul(251,516);where()>-(/~mul(249,409)&>don't()('^mul(504,414)!%>%[#{*<%mul(188,200)mul(167,558)>?>select():@mul(56,920)-how()}[{?^*,mul(280,213how(298,460)/]<*mul(827,209)$^{where();mul(55,859)?mul(820,629)^],*@+!]&mul(51,796)*@[:mul(391,202)(*&($?%;{mul(715,516)where()when()when(878,620)>^%><()mul(200,147)##}^$ ]'select()+mul(830,239)((~select()'mul(107,50),:<{when()(select(441,699)when()mul(190,106)]what()+where()@{where()#who()mul(280,967)who();from()( do()+'] /;where()&select()/mul(196,704))<&mul(268,129)#)!'?+**mul(2,819)---{what()mul(340,265)]-;@}-{>mul(957,38)>@~>&@<<mul(135,648)~mul(544,906)-mul(406,769)who()from()*where(713,507))(do()'[*select()&<'who(784,971){$mul(520,472);from()mul(823,285)]?)%{}mul(412,741+&{mul(822,931)^'*'*why()where()&!mulwhen()%@;^from(){mul(432,703)+/when(331,316):&who(713,258)mul(763,674):}~:mul/!select()why()&]{who()}]who()mul(908,41)mul(209,528)^mul% %#@@how()mul(986,797)+];~>who()(how()'~mul(576,420)!]:(who()mul(696,352)what()*,{]how()<#mul(722,996*^>[mul(707,555)mul(690,368)+*#{why()/why()?;mul(191,355)-}mul(634select()from(662,490)^select(130,551)>$when()mul(785,141)how()from()@//^do()from()when()%mul(280,35)(select()who()<:who()#<-mul(656,404)&+&mul(876,651),<'>mul(932,833)where()(,when()when()!/mul(756,656)&where();-^why()]mul(247,388)where()}{why()}?do()$}why()select()mul(783,462) &],(who()?:;mul(981,136)<{];>mul(848,166),]-why()do()from(102,362):-~how() why()how()who()mul(163,237)%;]mul(762,71)~,<when()^) /mul:who()[>:-;mul(996,713)#!-~do()<mul(573,915))>/ ^^mul(190,532)&'why()@~mul(580,20)mul(512,144),how(712,274)# ([mul(74,589)>-what()what()~mul(144,880){mul(566,592), {@*why()mul(523,558)@)!mul(826,790)!select()why()from()(mul(110,522)!select()('mul(704,195>+from()?%>mul(274,353)))}who()[when(660,595)~how()]mul(552,121)from()select()why()mul(240,146)why()>   mul(679,332)why()(%(from()^:why(492,169)}~mul(799,127)%~}mul(755,381)[;%what()how())*when()what(323,755)?mul(454,719[-%/;why()how(316,859)^!when()mul(151,252)where()&<who()%mul(379,643)/!#]what()+*<[mul(495,86)%>/ &mul(802,262)**why()mul(860,835)<&]from()where(617,545);;mul(547,933)'mul(809,108)! ~%]})where()mul}'what():mul(270,513)who() (]@,~][mul(900,601)~:?!'mul(772,689)*(%'**~^from(){mul(664,414)&&-who();who()from()who()$,mul(658!*+mul(68,562)[*mul(227,260)^:#(;mul(171,823);>select()where()'+$$mul(520,111)>+*select()why()mul(713,530)}-<$;(;mul(94,879)?who()why()@/^/@mul(58,649)?why();(what(994,808)-:,mul(439,811)' mul(885,101)[what()mul(713,539)?/~where()!#/:#mul(119,763)where()select()[%;:from():+mul(78,791)select()}'mul(253,65)^ %mul(175,244)]#+why()where()@;mul(610,884)*;what()mul(70,112)( mul(375,253)mul(321,80);+mul(498,751)>[~?mul(848,622?$<how()':?']-what()mul(931,202)-@where()-$+$>mul(43,693)how(){(^<who()why()!*mul(696,319),when() ?,$mul(655,774)~who()mul(507,610)@^<]mul(303,538)[] what()what()!)?mul(699,30)$;}{who()(+when();mul(100,158)>mul(91[)why()^:^from()]*mul(773,980);how()@who()(mul(817,36)
!']*;&mul(832,66)[when()'^mul(839,760){ what()mul(374,389)-^*when()what()mul(743,565)+%where(24,265)--@mul(37,414)mul(200,132)&<&@when()who()]~what()mul(816,483))from()%{:mul(237,407)]what()select()where()mul(937,582)why())!)mul(196,261)>*<%;who()<:!mul(42,391)) where(711,384)mul(286from()'[%mul(112,582)from();:mul(755,219)!**where()<#?)mul(893,667)who()> #mul[:where()select()<mul(969,985){{mulfrom()#>>(/from()>:-mul(587,565)mul(979,685)mul(686,145)who()?;]:{$mul(815,666'?&mul(981,191)mul(710,249)~@what():*&+from()<mul(581,50)from() #$(; #;mul(299,214)when()%&where()/(&mul(982,443select()*?%>&mul(420,352)&(-mul(308,752)@'mul(408,712)why()<]mul(610,709)~{mul(495,81)~),select()^mul(257,534)where()}mul(154,658)^}mul(313,894)(why()mul(950,201)>why()%what(276,768)%-+??(mul(578,595)mul(860,931)&{>:)^mul(566,551)>what()+how()[*from()[)mul(879,564)from()~:^'}[select()#who()do()%what();who()#mul(614,173))>'mul(274,742)why()+!&*/mul(962,316)?do()<@mul(599,386))mul(355,316)+-;&where()-;don't()?#[]#where()[::mul(532,56)>~@mul(264,327)~?%]&@! when()+mul(739,388)<$*)what()mul(732,804)from()who()'who()#,~mul(642,712)?}?mul(362,448)&,}+}: -mul(980,701)']when()-;]^^mul(761,19)+,mul(69,535)$>@<mul(901,942)who()when()^how()when()mul(424,524)-why())select(934,173)what()}/where()>do()%/:mul(244,738) mul(836,850):;when()from()'?do()]who(931,224)+}*@when(690,226)how()!mul(651,163)?how()-(&$-@mul(159,702)@mul(165,723)}(mul(401,331)'what() what(75,87)){,mul(665,944)/<:@'why(290,555)&%';mul(906,18)select()mul(394,692)*where()'mul(127,395){^what()~-^mul(892,931)%@who()<!/!(mul(900,441)']:(~$%,from()$mul(128,485))^+]what()do())&from()}'?%-from()how()mul(72,571)]+:~where()^&/$ mul(265,657);why()mul(420,896)':$@~ +<?mul(738,771)*~mul(769,630)select(202,447)];mul(64,143),(mul(820,863)}[]:)<[from()mul(913,277)<;+;@[*!/mul(339,478)mul(533,783)^mul(925,371)@@what()]mul(554,646)]{do()--where(),}mul(869,832)?::-<>mul(770,492)when()mul(615,744)<;+]+how()}&(mul(849,515)who(651,542)/@$mul(551,298)mul(752,737)>?what();^{,mul(518,320)~select()(+-,>;)~mul(37,708)why()'mul(213,584) how()mul(552,286)mul(41,894) <(mul(410,393)(why(),^<(#{~+don't()what()/when()what(35,148)$how()  $'mul(352,31)'!?[mul(922,598)# mul(274,543)/{/'!-()mul(899,396))&-,#select()how()select(963,44)mul(719,545)(')mul(909,800)-[-mul(718,282);< )where();*#+mul(509,20)>?@^select() $do()]:what()>}!!^({mul(492,556);<{$how()mul(98,348){]'don't(){{',mul(564,736)>{how()'mul(154,829)'~from()]do(){mul(452,695)+where()who(),-mul(906,99){} >why()why()mul(12,879)~when(646,247) ;mul(878,782)+ '{+&>who() mul(726,701)%<##!mul(457,189)$>')+~!%do()where() @when()mul(159,260)<%[~when() why()mul(663,679)}/
";
}
