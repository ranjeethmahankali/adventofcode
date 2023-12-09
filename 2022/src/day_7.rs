/*
--- Day 7: No Space Left On Device ---

You can hear birds chirping and raindrops hitting leaves as the
expedition proceeds. Occasionally, you can even hear much louder
sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its
communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top Error: No
space left on device

Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the
resulting terminal output (your puzzle input). For example:

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

The filesystem consists of a tree of files (plain data) and
directories (which can contain other directories or files). The
outermost directory is called /. You can navigate around the
filesystem, moving into or out of directories and listing the contents
of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you
executed, very much like some modern computers:

cd means change directory. This changes which directory is the current
 directory, but the specific result depends on the argument:

cd x moves in one level: it looks in the current directory for the
 directory named x and makes it the current directory.

cd .. moves out one level: it finds the directory that contains the
 current directory, then makes that directory the current directory.

cd / switches the current directory to the outermost directory, /.

ls means list. It prints out all of the files and directories
immediately contained by the current directory:


123 abc means that the current directory contains a file named abc
with size 123.

dir xyz means that the current directory contains a directory named
xyz.

Given the commands and output in the example above, you can determine
that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

Here, there are four directories: / (the outermost directory), a and d
(which are in /), and e (which is in a). These directories also
contain files of various sizes.

Since the disk is full, your first step should probably be to find
directories that are good candidates for deletion.  To do this, you
need to determine the total size of each directory. The total size of
a directory is the sum of the sizes of the files it contains, directly
or indirectly. (Directories themselves do not count as having any
intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file
i of size 584 and no other directories.

The directory a has total size 94853 because it contains files f (size
29116), g (size 2557), and h.lst (size 62596), plus file i indirectly
(a contains e which contains i).

Directory d has total size 24933642.

As the outermost directory, / contains every file. Its total size is
48381165, the sum of the size of every file.

To begin, find all of the directories with a total size of at most
100000, then calculate the sum of their total sizes. In the example
above, these directories are a and e; the sum of their total sizes is
95437 (94853 + 584). (As in this example, this process can count files
more than once!)

Find all of the directories with a total size of at most 100000. What
is the sum of the total sizes of those directories?

--- Part Two ---

Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run
the update, you need unused space of at least 30000000. You need to
find a directory you can delete that will free up enough space to run
the update.

In the example above, the total size of the outermost directory (and
thus the total amount of used space) is 48381165; this means that the
size of the unused space must currently be 21618835, which isn't quite
the 30000000 required by the update. Therefore, the update still
requires a directory with total size of at least 8381165 to be deleted
before it can run.

To achieve this, you have the following options:

Delete directory e, which would increase unused space by 584.

Delete directory a, which would increase unused space by 94853.

Delete directory d, which would increase unused space by 24933642.

Delete directory /, which would increase unused space by 48381165.

Directories e and a are both too small; deleting them would not free
up enough space. However, directories d and / are both big enough!
Between these, choose the smallest: d, increasing unused space by
24933642.

Find the smallest directory that, if deleted, would free up enough
space on the filesystem to run the update.  What is the total size of
that directory?

*/

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    fn get_size_map(input: &str) -> HashMap<String, usize> {
        let mut sizemap: HashMap<String, usize> = HashMap::new();
        let mut stack: Vec<&str> = Vec::new();
        for line in input.trim().lines() {
            if let Some(cmd) = line.strip_prefix("$ ") {
                if let Some(dir) = cmd.strip_prefix("cd ") {
                    match dir {
                        "/" => {
                            stack.clear();
                            stack.push(dir);
                        }
                        ".." => {
                            stack.pop().unwrap();
                        }
                        _ => {
                            stack.push(dir);
                        }
                    }
                }
            } else if let None = line.strip_prefix("dir ") {
                let (sizestr, _fname) = line.split_once(' ').unwrap();
                let size: usize = sizestr.parse().unwrap();
                for i in 0..stack.len() {
                    let entry = sizemap
                        .entry(stack[0..(i + 1)].join("_").to_string())
                        .or_insert(0usize);
                    *entry += size;
                }
            }
        }
        return sizemap;
    }

    fn part_1(input: &str) -> usize {
        get_size_map(input)
            .iter()
            .map(|(_d, size)| *size)
            .filter(|s| *s <= 100000)
            .sum()
    }

    fn part_2(input: &str) -> usize {
        let sizemap = get_size_map(input);
        let todelete = 30000000 - (70000000 - sizemap.get("/".into()).unwrap());
        sizemap
            .values()
            .filter(|&&s| s >= todelete)
            .min()
            .unwrap()
            .clone()
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 95437);
        assert_eq!(part_1(INPUT), 1432936);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 24933642);
        assert_eq!(part_2(INPUT), 272298);
    }

    const EXAMPLE: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    const INPUT: &str = "
$ cd /
$ ls
dir plws
dir pwlbgbz
dir pwtpltr
dir szn
$ cd plws
$ ls
dir ffpzc
dir frcmjzts
92461 nbvnzg
dir phqcg
21621 vqgsglwq
$ cd ffpzc
$ ls
48459 dzdfc.vqq
143107 jql.jzl
208330 mmnvqn.hqb
290122 svjvhflz
218008 wjlmgq
$ cd ..
$ cd frcmjzts
$ ls
dir bsltmjz
dir jfzgrbm
$ cd bsltmjz
$ ls
34237 dzdfc.vqq
58741 mdgdhqgw
$ cd ..
$ cd jfzgrbm
$ ls
132811 fcmpng
103661 lgt.swt
173031 vqgsglwq
29134 wprjfg.zbr
$ cd ..
$ cd ..
$ cd phqcg
$ ls
955 jgfs.zjw
$ cd ..
$ cd ..
$ cd pwlbgbz
$ ls
dir gbg
dir mjzhcwrd
dir njcscpj
dir sphbzn
dir tbgjpphc
55938 tvrfpczc.djm
4840 twd
$ cd gbg
$ ls
287003 fcsjl.bzm
dir wgq
$ cd wgq
$ ls
22963 fcsjl.fcm
$ cd ..
$ cd ..
$ cd mjzhcwrd
$ ls
228632 clfnpmbq.zmb
28276 dzdfc.vqq
2982 tdbg.wgn
$ cd ..
$ cd njcscpj
$ ls
dir dqzgqgv
275186 ffpzc
192491 gjnflc.plq
$ cd dqzgqgv
$ ls
70309 dzdfc.vqq
56139 fcsjl
142095 sgwz.cdz
dir snjntth
dir sphbzn
284618 wjlmgq
$ cd snjntth
$ ls
51918 ffpzc
dir vrfgfds
$ cd vrfgfds
$ ls
155233 jlscz
$ cd ..
$ cd ..
$ cd sphbzn
$ ls
dir qbzwrrw
dir qwpzn
$ cd qbzwrrw
$ ls
278531 fcsjl.tqj
211591 snjntth.gpd
$ cd ..
$ cd qwpzn
$ ls
174183 vqgsglwq
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd sphbzn
$ ls
185471 bsltmjz.fqz
dir bsvh
dir fvzcs
dir ndw
dir nlml
dir pcbt
286260 zhcmrpvt
$ cd bsvh
$ ls
130814 wjlmgq
$ cd ..
$ cd fvzcs
$ ls
dir cgmv
dir ggzwljr
298241 qvzghdpw.lms
dir snjntth
dir sphbzn
$ cd cgmv
$ ls
46843 dzdfc.vqq
dir lmqcbbm
dir rstcqsmd
215261 snjntth
$ cd lmqcbbm
$ ls
229898 bdmbvgp
25529 ffpzc.stm
16871 lnpjzvg.qlj
$ cd ..
$ cd rstcqsmd
$ ls
289038 zrbbbwng.smf
$ cd ..
$ cd ..
$ cd ggzwljr
$ ls
198200 bcthn
$ cd ..
$ cd snjntth
$ ls
191672 fwp.phf
68229 hzs.zpg
dir pggcwb
222426 qbv.bwj
dir snjntth
155354 vmqcm
$ cd pggcwb
$ ls
154272 fqztwvnv.lvv
dir pdjg
62393 vqgsglwq
dir wjhrtg
$ cd pdjg
$ ls
260644 gvhlrcf
209906 wpls.pbd
$ cd ..
$ cd wjhrtg
$ ls
148640 dljf.zrq
172168 dzdfc.vqq
196203 hjdphcfm
247620 sgwz.cdz
$ cd ..
$ cd ..
$ cd snjntth
$ ls
37467 ndlshlmj.cjq
257404 snjntth.nsf
$ cd ..
$ cd ..
$ cd sphbzn
$ ls
64082 bfdv.lwv
dir bsltmjz
58942 dzdfc.vqq
dir snjntth
$ cd bsltmjz
$ ls
dir bsqqdr
266007 fcsjl.gfm
dir ffpzc
dir frsmrd
72122 nthnhzwf
286705 wjlmgq
$ cd bsqqdr
$ ls
117496 wcqt
$ cd ..
$ cd ffpzc
$ ls
280224 mmnvqn.hqb
105346 vrr
$ cd ..
$ cd frsmrd
$ ls
111509 sphbzn.shz
$ cd ..
$ cd ..
$ cd snjntth
$ ls
177491 mplj
9029 pvbz.jwn
92891 snjntth.zrv
203356 vnnnw.gql
134728 vqgsglwq
$ cd ..
$ cd ..
$ cd ..
$ cd ndw
$ ls
241303 bht.rpj
173068 vqgsglwq
$ cd ..
$ cd nlml
$ ls
228982 hzglfpvq.ftt
114981 sgwz.cdz
$ cd ..
$ cd pcbt
$ ls
dir bsltmjz
dir ffpzc
dir fjsjwfg
dir fwm
dir jvwt
227943 tmr.jdc
dir vwpqzdwh
31258 wjlmgq
$ cd bsltmjz
$ ls
177750 bsltmjz.spj
dir ffpzc
dir flrpwfs
138824 mtmdtcpv.cfj
165770 wzqwczj.qwn
$ cd ffpzc
$ ls
179645 snjntth.dss
$ cd ..
$ cd flrpwfs
$ ls
60566 wvjq.gmm
$ cd ..
$ cd ..
$ cd ffpzc
$ ls
97847 qzhhtmd.bhw
1197 vqgsglwq
$ cd ..
$ cd fjsjwfg
$ ls
152232 dnsdd.jgz
181301 gsb.wsh
dir jqpb
dir jscbg
dir snjntth
227677 snjntth.vvg
dir sphbzn
75358 vqgsglwq
2589 wjlmgq
$ cd jqpb
$ ls
253403 mmnvqn.hqb
108325 rvq.mrc
$ cd ..
$ cd jscbg
$ ls
dir dtm
dir gsdnz
208269 prh
25977 qdzljgh
169262 vmnq.mth
$ cd dtm
$ ls
80072 gzqnb
$ cd ..
$ cd gsdnz
$ ls
dir dsqzjs
297895 sgwz.cdz
129983 vqgsglwq
$ cd dsqzjs
$ ls
2621 jqrlsf.gzs
164844 snjntth
$ cd ..
$ cd ..
$ cd ..
$ cd snjntth
$ ls
118553 cbhql
dir ffpzc
dir snjntth
$ cd ffpzc
$ ls
dir lmn
12104 tvlwn.vhh
$ cd lmn
$ ls
46401 bsltmjz
96888 shrnqhvq
$ cd ..
$ cd ..
$ cd snjntth
$ ls
dir snjntth
dir vlnfhbq
dir wpwl
$ cd snjntth
$ ls
dir ctj
$ cd ctj
$ ls
82485 fcsjl.lfl
$ cd ..
$ cd ..
$ cd vlnfhbq
$ ls
250106 qvf
$ cd ..
$ cd wpwl
$ ls
153950 cmsd.rlg
$ cd ..
$ cd ..
$ cd ..
$ cd sphbzn
$ ls
dir glgq
$ cd glgq
$ ls
285182 wjlmgq
$ cd ..
$ cd ..
$ cd ..
$ cd fwm
$ ls
251865 ffpzc.qgb
279522 zvvpfqtp
$ cd ..
$ cd jvwt
$ ls
48990 bsltmjz.nzp
219604 ffpzc
69573 mvmdfzr.lwb
$ cd ..
$ cd vwpqzdwh
$ ls
267581 pvcch
$ cd ..
$ cd ..
$ cd ..
$ cd tbgjpphc
$ ls
255571 dstpcgr.tfq
dir fdbwbrpp
dir gjzwh
dir hjvrtjt
dir rhzczj
292888 sgwz.cdz
dir wlzhr
149395 wnfzrqgz.dtn
$ cd fdbwbrpp
$ ls
dir ffpzc
dir qbrth
51164 qprp
dir slpt
117026 sphbzn
295685 vqgsglwq
dir znmj
$ cd ffpzc
$ ls
dir jhnzrdvb
$ cd jhnzrdvb
$ ls
217775 ffpzc.sgw
$ cd ..
$ cd ..
$ cd qbrth
$ ls
183969 lpbwgfjv.vcg
47333 wjlmgq
$ cd ..
$ cd slpt
$ ls
32343 tqhtj.jwz
$ cd ..
$ cd znmj
$ ls
55058 mmnvqn.hqb
$ cd ..
$ cd ..
$ cd gjzwh
$ ls
dir dvcbcd
202530 dzdfc.vqq
dir fsgz
dir hfrrqq
54897 jlzn.qsn
16097 ldzqsbb.jzl
225078 pswccrd
dir rqqmldw
292464 rzrdhbp.vld
dir ssqbqqp
dir zsztqrc
$ cd dvcbcd
$ ls
187837 dzdfc.vqq
dir jlwtvf
dir jnjvshs
164053 nrf.fqd
5665 tlp.jmt
13801 wjlmgq
$ cd jlwtvf
$ ls
219985 sphbzn.dvj
$ cd ..
$ cd jnjvshs
$ ls
dir bsltmjz
dir nrpm
$ cd bsltmjz
$ ls
152972 qgdqj
$ cd ..
$ cd nrpm
$ ls
18509 wjlmgq
$ cd ..
$ cd ..
$ cd ..
$ cd fsgz
$ ls
224576 mmnvqn.hqb
$ cd ..
$ cd hfrrqq
$ ls
dir bwgsnfvb
dir fcsjl
294608 ffpzc.gvm
136880 qjcgtw
dir sphbzn
$ cd bwgsnfvb
$ ls
29735 dzdfc.vqq
dir wstmzfml
$ cd wstmzfml
$ ls
158447 bnvhbvvc.nrt
59889 jclclgd
$ cd ..
$ cd ..
$ cd fcsjl
$ ls
138297 ffpzc.szw
$ cd ..
$ cd sphbzn
$ ls
dir wqdths
$ cd wqdths
$ ls
8326 cgvtw.jpz
$ cd ..
$ cd ..
$ cd ..
$ cd rqqmldw
$ ls
226757 dzdfc.vqq
115055 mwb.btc
dir qpts
298524 sgwz.cdz
$ cd qpts
$ ls
60860 bsltmjz.frp
dir fcsjl
94904 sgwz.cdz
dir wnmqqspz
$ cd fcsjl
$ ls
25082 mmnvqn.hqb
$ cd ..
$ cd wnmqqspz
$ ls
165529 sgwz.cdz
$ cd ..
$ cd ..
$ cd ..
$ cd ssqbqqp
$ ls
192477 pvrgm
$ cd ..
$ cd zsztqrc
$ ls
254053 lht.htn
$ cd ..
$ cd ..
$ cd hjvrtjt
$ ls
189942 fwps
$ cd ..
$ cd rhzczj
$ ls
36502 bmtfr
dir ffjz
35148 nctfhmzm.vsz
dir qdgjzcz
208196 rwql
79863 sgwz.cdz
dir snjntth
$ cd ffjz
$ ls
dir grsvhwm
$ cd grsvhwm
$ ls
50231 fwj.rdv
dir snjntth
$ cd snjntth
$ ls
dir dtbgb
$ cd dtbgb
$ ls
150245 vdflm.lmq
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd qdgjzcz
$ ls
222389 dzdfc.vqq
$ cd ..
$ cd snjntth
$ ls
56794 mmnvqn.hqb
$ cd ..
$ cd ..
$ cd wlzhr
$ ls
116628 bsltmjz
60122 jqpbsgnr.fgb
252605 lfss
300065 qwjdl.vhr
$ cd ..
$ cd ..
$ cd ..
$ cd pwtpltr
$ ls
dir dplsvrhl
140951 gwtfzqvd.znb
dir jbvdb
dir jst
dir qhjv
dir snjntth
$ cd dplsvrhl
$ ls
272101 fcsjl
dir ffpzc
58852 mmnvqn.hqb
dir mnhntjz
91899 sgwz.cdz
228077 snjntth.btv
$ cd ffpzc
$ ls
5499 bsltmjz
dir qmfwpjhl
dir rsrb
dir wgt
$ cd qmfwpjhl
$ ls
300362 dzdfc.vqq
$ cd ..
$ cd rsrb
$ ls
252983 dzdfc.vqq
107744 ltssrgqb.zvj
214895 rhglgcwr.wpw
249727 sgwz.cdz
$ cd ..
$ cd wgt
$ ls
141984 dzdfc.vqq
194686 mmnvqn.hqb
258023 pgr
$ cd ..
$ cd ..
$ cd mnhntjz
$ ls
dir bdvght
dir jprwchh
dir snjntth
$ cd bdvght
$ ls
243166 vpsvjdq.wsn
$ cd ..
$ cd jprwchh
$ ls
178943 bmpc.bjb
$ cd ..
$ cd snjntth
$ ls
dir nlbm
dir zjmjntff
$ cd nlbm
$ ls
33050 fcsjl.rcc
dir sphbzn
17446 wjlmgq
$ cd sphbzn
$ ls
214563 prrfhff.pbp
$ cd ..
$ cd ..
$ cd zjmjntff
$ ls
82134 sgwz.cdz
52203 vrtlgdq.crp
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd jbvdb
$ ls
dir wmtjh
$ cd wmtjh
$ ls
dir ggvwn
$ cd ggvwn
$ ls
192285 spqvmf.sdh
$ cd ..
$ cd ..
$ cd ..
$ cd jst
$ ls
dir bsltmjz
212740 dzdfc.vqq
dir gncztvtb
dir jsqjcqnt
286257 jvs
36654 sdcsm.mbg
192040 sgwz.cdz
dir tbqphzb
dir vdcqgts
285843 wjlmgq
$ cd bsltmjz
$ ls
215705 snjntth.gpv
213665 wjlmgq
$ cd ..
$ cd gncztvtb
$ ls
229298 vqgsglwq
$ cd ..
$ cd jsqjcqnt
$ ls
dir bsltmjz
dir fcsjl
dir ffpzc
dir sphbzn
70864 vqgsglwq
$ cd bsltmjz
$ ls
14981 pqzffzjc
$ cd ..
$ cd fcsjl
$ ls
140328 jwhczwbc
$ cd ..
$ cd ffpzc
$ ls
213650 mmnvqn.hqb
$ cd ..
$ cd sphbzn
$ ls
dir psmtphhq
dir sphbzn
$ cd psmtphhq
$ ls
dir ffpzc
123131 tzgwd
$ cd ffpzc
$ ls
49737 cfngvmd
dir gcnrp
172799 gmd.cwl
dir llnztjf
dir nbqs
79661 rrqz
$ cd gcnrp
$ ls
283 vqnrgl.vwp
$ cd ..
$ cd llnztjf
$ ls
63263 tjhm.bwh
$ cd ..
$ cd nbqs
$ ls
dir vssmq
$ cd vssmq
$ ls
88980 dzdfc.vqq
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd sphbzn
$ ls
20140 fcsjl.zrs
260579 snjntth
$ cd ..
$ cd ..
$ cd ..
$ cd tbqphzb
$ ls
93470 sgwz.cdz
$ cd ..
$ cd vdcqgts
$ ls
223564 dzdfc.vqq
dir ffpzc
dir gwhfgwf
dir nbjtqnng
dir snjntth
$ cd ffpzc
$ ls
42813 qwwmw.nmt
$ cd ..
$ cd gwhfgwf
$ ls
59918 jvfv.mpm
dir mjl
211039 pcwl
$ cd mjl
$ ls
13004 pgjb.tpq
195995 tms.fjz
$ cd ..
$ cd ..
$ cd nbjtqnng
$ ls
107058 dzdfc.vqq
dir ldrsd
111631 vqgsglwq
104599 wbzmdljw.tjq
155747 wjlmgq
$ cd ldrsd
$ ls
107439 jvjm
$ cd ..
$ cd ..
$ cd snjntth
$ ls
242680 fgrt.gng
$ cd ..
$ cd ..
$ cd ..
$ cd qhjv
$ ls
dir bmnm
68453 hjjpdgn.hwl
dir sjlbj
dir vqnrj
$ cd bmnm
$ ls
1238 vqgsglwq
$ cd ..
$ cd sjlbj
$ ls
44239 wzzbtmrz.gml
$ cd ..
$ cd vqnrj
$ ls
3286 bsltmjz.qlc
$ cd ..
$ cd ..
$ cd snjntth
$ ls
130833 blm.wmt
dir snjntth
dir tcnmbcgg
218869 wjlmgq
$ cd snjntth
$ ls
dir snmrdfbt
$ cd snmrdfbt
$ ls
281025 bzrsds.lfg
$ cd ..
$ cd ..
$ cd tcnmbcgg
$ ls
194998 fcsjl
dir qdrmpqgn
dir rzqd
dir tcsds
$ cd qdrmpqgn
$ ls
165713 qmzgt.tnc
$ cd ..
$ cd rzqd
$ ls
dir cwhnmlv
57819 fcsjl
246864 pjnzdvd.gjm
$ cd cwhnmlv
$ ls
287539 mmnvqn.hqb
215636 pbnjt.zbn
124638 sqd
$ cd ..
$ cd ..
$ cd tcsds
$ ls
78812 gfmgb.wqj
218987 hnhfvz.dln
209640 mzzhqlq.zqp
102492 nml.ppg
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd szn
$ ls
dir fcsjl
dir snjntth
dir zjbp
$ cd fcsjl
$ ls
158019 jsv.pmz
$ cd ..
$ cd snjntth
$ ls
229510 dfvpvp
191061 fgplbptq.jlt
dir lfb
234911 lfsrwr.wcb
dir lrfcgzl
48031 stbbw
219691 vqgsglwq
dir zshh
$ cd lfb
$ ls
dir btj
99591 dhrjbvvg.gwm
137224 dzdfc.vqq
201972 jtzmqsvj.wnd
9704 mmnvqn.hqb
dir pwg
200308 snjntth.css
dir wcmhcfm
dir zwhvmln
$ cd btj
$ ls
dir rnbzdfgn
51799 wdhsm
dir wvf
$ cd rnbzdfgn
$ ls
117095 bsltmjz.tlv
$ cd ..
$ cd wvf
$ ls
dir ffpzc
dir ncbmgpsc
dir wtwrmjnt
$ cd ffpzc
$ ls
249919 lsth.fmf
$ cd ..
$ cd ncbmgpsc
$ ls
147899 dzdfc.vqq
$ cd ..
$ cd wtwrmjnt
$ ls
252366 pvpdv.jwz
$ cd ..
$ cd ..
$ cd ..
$ cd pwg
$ ls
280845 fcsjl.fjz
44300 sgwz.cdz
dir snjntth
229605 vqgsglwq
$ cd snjntth
$ ls
2053 pflvsnzs
143522 sgwz.cdz
$ cd ..
$ cd ..
$ cd wcmhcfm
$ ls
229329 qsznhwlw.vjg
$ cd ..
$ cd zwhvmln
$ ls
dir ffpzc
dir tjjzbf
dir wzcq
$ cd ffpzc
$ ls
dir ncnj
37497 vqgsglwq
$ cd ncnj
$ ls
40920 htbjhjq
$ cd ..
$ cd ..
$ cd tjjzbf
$ ls
47522 mczn.spd
$ cd ..
$ cd wzcq
$ ls
56662 ffpzc.vwp
dir snjntth
$ cd snjntth
$ ls
117276 wjlmgq
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd lrfcgzl
$ ls
267485 rsjmpph.qqz
$ cd ..
$ cd zshh
$ ls
dir ffpzc
dir gmn
dir snjntth
150048 tgtlh
32020 thfr
72152 vqgsglwq
$ cd ffpzc
$ ls
dir snjntth
$ cd snjntth
$ ls
224945 dpfpz
$ cd ..
$ cd ..
$ cd gmn
$ ls
238996 sgwz.cdz
$ cd ..
$ cd snjntth
$ ls
86775 dzdfc.vqq
19560 vshcmjj
$ cd ..
$ cd ..
$ cd ..
$ cd zjbp
$ ls
dir fcsjl
41522 nlvpb.fpf
dir nmtjtd
$ cd fcsjl
$ ls
276802 fcsjl.psm
197934 sgwz.cdz
$ cd ..
$ cd nmtjtd
$ ls
47477 dvqmqlgw.ths
51081 vqgsglwq
";
}
