/*
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with
supplies for the jungle journey. Unfortunately, that Elf didn't quite
follow the packing instructions, and so a few items now need to be
rearranged.

Each rucksack has two large compartments. All items of a given type
are meant to go into exactly one of the two compartments. The Elf that
did the packing failed to follow this rule for exactly one item type
per rucksack.

The Elves have made a list of all of the items currently in each
rucksack (your puzzle input), but they need your help finding the
errors. Every item type is identified by a single lowercase or
uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a
single line. A given rucksack always has the same number of items in
each of its two compartments, so the first half of the characters
represent items in the first compartment, while the second half of the
characters represent items in the second compartment.

For example, suppose you have the following list of contents from six
rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which
means its first compartment contains the items vJrwpWtwJgWr, while the
second compartment contains the items hcsFMMfFFhFp. The only item type
that appears in both compartments is lowercase p.

The second rucksack's compartments contain jqHRNqRjqzjGDLGL and
rsFMfFZSrLrFZsSL. The only item type that appears in both compartments
is uppercase L.

The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the
only common item type is uppercase P.

The fourth rucksack's compartments only share item type v.

The fifth rucksack's compartments only share item type t.

The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be
converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in
both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v),
20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each
rucksack. What is the sum of the priorities of those item types?

--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you
with another issue.

For safety, the Elves are divided into groups of three. Every Elf
carries a badge that identifies their group. For efficiency, within
each group of three Elves, the badge is the only item type carried by
all three Elves. That is, if a group's badge is item type B, then all
three Elves will have item type B somewhere in their rucksack, and at
most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated
authenticity sticker on the badges. All of the badges need to be
pulled out of the rucksacks so the new authenticity stickers can be
attached.

Additionally, nobody wrote down which item type corresponds to each
group's badges. The only way to tell which item type is the right one
is by finding the one item type that is common between all three Elves
in each group.

Every set of three lines in your list corresponds to a single group,
but each group can have a different badge item type. So, in the above
example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three
rucksacks is lowercase r; this must be their badges. In the second
group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker
attachment efforts: here, they are 18 (r) for the first group and 52
(Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf
group. What is the sum of the priorities of those item types?

 */

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    fn part_1(input: &str) -> usize {
        let priorities: HashMap<char, usize> = HashMap::from_iter(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .enumerate()
                .map(|(i, c)| (c, i + 1)),
        );
        input
            .trim()
            .lines()
            .map(|line| {
                let half = line.len() / 2;
                let l = &line[..half];
                let r = &line[half..];
                match HashSet::<char>::from_iter(l.chars())
                    .intersection(&HashSet::from_iter(r.chars()))
                    .next()
                {
                    Some(letter) => priorities.get(letter).unwrap().clone(),
                    None => 0usize,
                }
            })
            .sum()
    }

    fn part_2(input: &str) -> usize {
        const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let input: Vec<_> = input.trim().lines().collect();
        let priorities: HashMap<char, usize> =
            HashMap::from_iter(LETTERS.chars().enumerate().map(|(i, c)| (c, i + 1)));
        input
            .chunks(3)
            .map(|chunk| {
                match chunk
                    .iter()
                    .fold(HashSet::<char>::from_iter(LETTERS.chars()), |set, line| {
                        HashSet::<char>::from_iter(
                            set.intersection(&HashSet::<char>::from_iter(line.chars()))
                                .map(|c| *c),
                        )
                    })
                    .iter()
                    .next()
                {
                    Some(badge) => priorities.get(badge).unwrap().clone(),
                    None => 0usize,
                }
            })
            .sum()
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE), 157);
        assert_eq!(part_1(INPUT), 8233);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 70);
        assert_eq!(part_2(INPUT), 2821);
    }

    const EXAMPLE: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    const INPUT: &str = "
GGVGlqWFgVfFqqVZGFlblJPMsDbbMrDMpDsJRn
LwzHtwdLHHwDrzPZzzsJbJ
wdLTBvSvHvZVGCjhfN
HsSSnZVHjjssZnJpSJjBHHWgQGcgqqQLQdQFqNgWgqGNDg
rmmRwrtfThtTrbCrGGGcLBDTqDBNQLdL
mwPrrbzPfwvbzhwMMnnjHnBjZlnzMM
gjjdMBgdqdTpJpBcjgRRRlqnvrqfnZtrtZDw
zHShWLhCszCWHVbVzQWCPtQvNZRwtfftfNnrnftlfR
PzPSssHbVhCLFMJFcMFJJMjdJw
ZqJdtpfpjmpJjpnwWdttTCDLLTQFNTzTzrcqrQqc
MsSlBGvBsSGGSlbGsCgggNTgzNLczFQNrNQVQcFzFF
sGHHSGllhvMGhSRGCjtjtjnjnnmHWpWWtJ
tMdjQlHPHsGjsCtsCpwwqfhfnnFMDMrpfD
SbNvWvBRJRWwFSgppgSrfg
RNcNbvzJRcVLRVzTRFLjdHCQttdZdPlHstPl
QWqgpdBflpHNCNWNHHPm
VVMbbJsLFVMhrMJMmRjFNHwHjjCTGSSRFj
mbMsZzsLmVhJZrcLcJhLMtnqvBnZdggplDffvlnlvnDn
prnNnsFnZpnBNdNtFrNnzNQQwTTQZqTHTQJQMwHDMDlZ
jgfgcSmbLmhmcPShghRdmwJTQjTlqGlJQJHqQqGHqQ
hRVhPfbCgbVggLVRSSmRhRPhrrrnCzzsvCvrnvFnNppsvBtd
QJLNDWSWQdLFFFhLdt
npHhHMsfsjpZjznRtmrMCdBwFBFrBdmt
HsjHqRRfnnHRsgfHffZspgzqDGQSWbQTDNGhQhSqNPhDWWbT
bsCmFDsGZCNsDmLDLZBSHSJTHnrZQMQSSQ
jqRpwvfqnnRQrftdBMHddB
phpchwpzjpvwRzwcsnlFsssPCCGzDlsD
rMqzVQfrfVZWZhTdRTQL
cgmtFtjFFJDDtFvSFRZdLlhpHZddmwTZWh
FbcSTtctcvFTJNgtJDGNPnCqMPMfMBfznGVsrMCq
wLJfGJJPZLBfwSLGHbqmhhDHHhFDzfhv
FsnpFjVjplTQCspNlCDbzhMMbqvMvsgmHDqb
lRdlTdTddllpRQFRltVVdFRcwrtrcWWcPrrWPrPSrZWLPc
VGVZhTppGTfPnJVJrFqbsmbSSshHqWqRHF
llzDCzlBLdNcCddlMMNBdCCtWHbFqFRRRsHjWtRwSWqbmjWm
NbcMBBvzzMQLCDBVTQQPVrPQPPZVPp
cdcgfmQdqlqhzzPjzfwpwf
GLBGBDvbvRzGwtnnmPpp
ZRCZBRFSRvLRLFvvbLLFQdHMTHTlQlNqNmqFlWdH
vzjzvHtcHvJcDStLLGSShCbbfF
MWFFTVZRMmMgdQdSQLwQrQwbGw
gFTmgmVZssRsWZRNzJlBHHnnJDvzNPlP
rHrvHpmHZfdGDDGGZd
cTlMsNhllMhGchNPCBlhMQgVDdgDSSWVbWVwRQwRSgbV
lnBjnNNTTMnCTcChPNhMvtzvFGLtJrjFtHHHzHJm
lgpdZZMmGVVzVZzt
HfHLrHqbPbzJJzRJJPTl
HsLWWbDqFrqlqfbsbDqDBncpgFmmvpnmmgpvdvjcdM
GpNVbTpJJNvMBMVvJTGvhnWQQScllnhhWlhVSznV
ZjswwHHLZzGnjWjSjl
sHdftLLtgLfwdtPmHtMbNpMTpNqGRbPvTqPv
sHSNNhNwsllGSGGlWSGWSsFrrVbQrdmFrVrrmnrHmrHr
QQMRZDDRcrcnmRcV
fJfCPMJCzTMZSGsWwsWBwqQf
HwQZZJsHdqqsdJQGRgCgVGgSqgpcGG
ljWWbnPhjBlGpCRCnScSGg
hrrztWlbPjltjMPSdJDZSsHttwsZwD
VzzbmzvpvNhvBDqc
QHSJSQGCwJCGrGQjjcgcBFhdgqdqFdDNNw
rCGJtZrHhhtLRsth
TMWwCLPpMTThrvtMRJjbjRvmJs
fDzcHFfSfFQfZzZRJbdmmqqssqtbSW
WWgGZglcllgPBBCBNVGPNr
wrwwhpTpbqhqrshrrfrFfwfzRJGdNJHNmcFzCCzCRJGzGR
vMggvjQvgPvQjVLMMPSZqWNJGCzcNGdcdzHPPzcmCzPz
qDZWvBZVfDhbTtrp
LpDvHdjVghnjbGrn
FBBBPwwlBlwSfFTWPHPWWhmgngmmnPnmbsngngbGrb
FwftBSCSfWCtwfVQDvHHCMVvdQLQ
ZrQpQlSpNlqQCVnQBmdDqmWDqmWWBDBB
HsZMsJvZzLMHTRwWhgDwmfDBgdhWdf
RZvTzJGzRjFrVNVjlQrS
mqjMwfqlSSPmSrlPhwhVpGRcppWcpcGRcGWv
ssJDJJNgZNDWrRWcRpvr
ZTsTnnsLJQgPnfMwmnMrfm
qsVBvZqWLdfbfvLj
mPNRgmHBBGQrCbSbrdfCCSbC
PlQTGcTTcgGFQQGPTGllpqMzwzpVJZwBMssZ
FWGcNRLRLhwJJQfV
nzbzlDBHSpTDbpDpzHwCqhqwJJghQqQMCCBw
JnzndzpmJFmNsrrFsc
gmRwwDwfnRDJgwZLFQFFNGNQrFBmFbbm
CCVHVWWThSrjVGvbNj
WpdqpplppHCWzlClMMTTZJcJsdscJLLdbDDfZDgL
VNtCCMDllpBqDvtdCczTSgjHlzGSHSGZTZ
hPFPsQhhFhLnbsRnLFssdzcHdsSHSSHgjzHG
QPWPQrPPmbdnbWLFPrrBVrVDBqqNMVwttDtBvD
PPNNRggwgRRgHBhDtwhTwbDs
SFGSFSMCJFMrcrCMSSsbtrTbbZhBvtHhrTHD
MFfSMpflQLQflfLjnLmddsLdddqq
RcgbcrrFscVrwZVCgVGGmHppNNndWnGdNqddqqNqND
jTlSTBSTjLTvlvjjPtvMLlhHnftphtDFNFqDnDHWNddn
QBMQvzzjzvJPjFQMmwZJrgmCCJVRVbbc
CzPJsWCpvsNszsJsNsHlDhMrrJGjhrRVhRGgDDjG
tFFdbqFLFdwctQdfVhjRRghTcrjVRTDW
bwQtFLdLBdFmwnHnWHPBNnHCnp
CNTstGNslRRRstlmNmmTZZqfFwtqgwqgfBPSwSWwqgWq
hpDbcHbpSrcgqqzhhWVfqg
DDcLDjbMjCSsZRNlMv
MhHMCMNbzbMHlcqmGmrmWc
tnPggdZPBPgdtttJpdnwVBnmqQcvlQrQlfGqmfWffBcqWD
VPPwPPLPwLGFGLzCbG
rqBcBmjHTGfPbcVgPG
dlDpsdshzlldlDvsWlWvLQbQBbfLFLbPvbBGQBgG
BlBznnRWzlphphBnhZjZtNNCNmrNqjCqHwHN
mQBvmvBmmLJvvrLtttQrfhGlcRGfRhVGWJVChlRG
MzPswTsbTPPsNgMNszgzMpbMfcRcGflVGRfWSpFRlWWWFhcC
bcPsTbgbbTTwNZzTZzvdjdjtQQndZvdrvdmZ
hQzTQJFFZJrcdcdZFFrGFSVWVRWRwRgRHVMWDCDSWc
lPmpNBnnnsNBnLnfbfnCDWMvwRvDCCMPwwHWvM
HpjmffNlnqqhddTddFZjGJ
BwsLFFbHLbVCSCSFbsbFLsJbqnTtZrRMHTZtrTrZTcRRRRTq
lGhNhpPmmhpztZTBrcpjRqpB
QPzdfzBQNgFJSCwsdLbS
ZsZsSBTgffSCqSSfrMnnMwjqmqmnnnqwMm
bbPPbzVclcPzGNlvzVtmnDBnQmtnQLBjJVLn
zPFGplGGvdPbHplcbzzvdlNBTThgRpCTCTfhfsCCsSRZhR
CVLSVCLVZRsHcnzSRpdZZRCdPlmcMWDDlPNqMqtDMmqPMlDt
TBnGjfQrQJjhfWlPPmPQDNlmlP
fjhhGvjvvrTTBhvTBTbvGVRLzVnbSRZpHddspHRzLs
DDtWjfRfftWMLzSQjzzhWjjwRVPHqFbBbZHVwZBFvFwZvZ
JGllgCJlJsrCGPrCNTPdslvZVVNVbvBqNbbpbbFHpBwZ
CcPdnCdmCJjfcftWhtSL
pgpfddDGHWzDNGNGpRCQjCTFHZZQFQjcRT
bJlhqmMvnlrRQFtTthPVhZ
lvbJrlJMBwfzGNTddB
wpbJGGZpsjvtdWvGWF
HqqhBhBqhhNQHTSHqqNzRHVPvTvddWrjtrjFvrvdTdVP
NRLCRzlqHQtNRBLzQllhhZcgbggwmLDZpsgssDpwwD
pDzFzJFcVMcWJFJFzpLBsqWLZssshsGLGbsS
wqHqfvnfrRwQtdQRthhBbBbZLhPLnBTGsh
CfQqlqvtfHNvMVmzmmMCFDMc
GcgpNHvcSNvpSLphdhsLdQTsdWThhQ
wwzttPrrhQswdhnT
tjJjMJRbRbjztmjtjbgcRsNlgglHpDFSlSvg
VVLvLqqPVlvcqLLdwLbHpzcHSsbRJppHbHpF
CfjjCNGmMWhWjhWHWb
ZmGZffggrDqZvZtlbTqL
TTmmhvBvvHWzHpsPpstpLVdwwsLb
qflfFgNctNcCnCCNDnfFFNDwrslwZbPswwZbJLJZbrlPLL
FgQDDcncStCgtqccjSDTHWMThvhTQMBQhWvWRG
SqhVghPccSBhgSBqWBFNQNsHQHMjCCQQWCwQHN
fLZftnlttcbbtZbZlpZtttQjwsCQjRwwRDQspMRMNNQs
TfLtvbJtZmlbTTTtlJbFvVqPSgBdPqPhFSGBcd
pPPNDptcqtpcDztLDhhngnnJgJTmJwNnwm
HVVCsSClHGBCHslWHbSCGGVngTrJwnnJnQRRBrhQhgJhdm
WTWWWsvVlvGbWCFvjDftPpjqZLLtDz
wWclwtDwRvflvffB
sMMsGdsSTMrJZNqczfdvhvnzCnfv
rspppMjMspSTSMpgLjcPFmwPLmPHwb
tCdSMHtHtRFHdWSSJQSgrrrnghTNJN
BGfcvDsfvsqcvqfGvfGnNLhggBNQJNJQmpgQJm
sGfQDPDZzfDZzclwDzwsDlfjtbdHClFRCMWjbMFMRFWbdj
pJNCcvqCccsVvFCpqsmvWJfCBWgSzBBRrrBRDDgDrSbbgQbQ
TMLnLjjffwfwGdjQjDDBjBrDtztRSb
MdPLGhHnMZhlPHHTFfZvVCpmmmcFcVFC
SwFMfCMRCdQDdMbmdFfdbbnlcVncVCcgLqWcNNnCcWlW
hPjQzzhGzhpPrtPJPpPHrVgnqVVncVVnNHlqVnncNB
ptjGrptztpthtrtJJhTsGwFDZZDQmSdfZSwsRZSwfZ
rSSWWCWrdllHWpjcnFNnRCNjQp
bGwwJqGVGbGJVVhgbBgttGmBQjFsMjpMcMnnMBcQFNnsssvv
bfthwmfJfgwwmmwZqVJPHNHSZHWzSlDPrdDdSH
nmJccvclcbwmlbbvVbvsHwJJPCPNCNPnLBhrBPPLBhLhBgBP
MdRGtdDRTqWDMqtMDtQDRWSdLLBsrhLgBCgrgCgNNLPBfNMf
dRZQdDdRRSQWGsjZmwzjmlzsZH
PBGGMrTQTrTBpPQpLpSlwjwfjtlnfwbmGttw
fCsJCWJcvRCtwwjbCl
NsqcsfcvDHFVDJvdLQTrpdTTzTPpHr
rltrwsBTlrfGZggGBLGGNN
jhMnRQJVphMnbhvQjDZNqqZDNTNHZVHGHq
MRvbhQRQQChpvbjvMSvQnMcsFsfwwmlCwFwWcTWwrmPc
DDvLLLBnvrzvbvbmtv
TMwRjTRMGCwGGwrjQQnmrQrrQdhZ
MJPFHFTwgCGqGqgJMGDfSWcsnBSccgVDlnpW
flzVzNrdLNLJzrGlfdlzjrQDgFTpDgDmmmgFmqFDQjQh
CbnBcsZnPZVSnwvVsZbRhhBDpgFphgmgDgTppq
ZWnsWSnncSZsntZCbsswwJMzdLzlMdNMLtMVfrllMt
ZffSgNfgJgGCHZcZrpHrNJTLhqvSLTqQnvVTLvzvLTjV
tWFtHMwlBlDqjjzjnqvvlV
DRMPDtWHPFDBFFwWMFBmFRPgZpJfsffNGJNrGcsprrsmfg
wRZRmpZmlPqZjzGrdrGq
bBhQQFPQbPDVNzVNzdGWNdrf
QFbcDcDbLHgHBPDFRsSSMtmvRttMpCLS
MpWJVVJMcWvpRShcwpLGflmqzSfNdfNLdQzN
CDBTtCgtbjgCRrBrPBTQqzflNqjGdLzzmqffzq
rFgnnBbttDTPtHCDPrPMnpwVJhJvMZvpMvppRZ
sWTTmpsfsWppPTTsTVZWHVVZNvVcdcJvdN
DjjBzjhRHvvvSzdc
rMBjjrjbjrGDlgMlMrGjBgRLPTTwHMsfnFwFQPMPMnmFFm
QRRbDjjmPzNQwFDNmrQmzCbVHrMhBVrJLJJfMGGLtfJBHh
dsWcsqqWSWvnWnWcWGPJLBqhLBqGhBJHHH
ZWnPWgWgPnlbCDDwmmDbRZ
nfPqqfLqQnfHBSqnzztQjVmjfGRWJNGRWsJWJfmJ
TTMlMMlFDMGVGRsVJH
CbDbFDbvgTFFwgTDlDprhlPSqBzSnPdLPtPPHgznQqBQ
fJmWVfHqjfjhZCQZ
NcNzBNvgszQmzjnthZQC
LsLsgBNFmFgTFgGBBgcdMdvPDPDJWrlpVbGpJWqHDlHJHD
SllDdvzgdFDdlPJvbFDDSzFScPTRTNcwfZRwRhcwwNnRZTtf
WBpWBCLGVpLjHrHGGVhZNwcTVcNhVnRcNZ
LHLQLspHWQGpWCHnBvdzDJFlqvdsqgSgqS
GcTctDMjMhpMDRjLsMMsfDWFfdPCFNbnCPnvCPgW
JmvwqlBwnmfdFPFP
SvZqHSZqqHZZZBlllBwSwsRsMHpLjpLsMGtsMspGRT
ClLnCLfClLVllfLLcQjLBCfCmSHVsttsmtsVStDNVdppdsSp
PFrRMbWqMRwFRqRSqwqvMvMsGtgsdmssrgNtdmpNdDGgdt
bwJbPWPwFFPFSczCZzZZCcfjBJ
MwmBmzwJQTcTmfPVfZPwhhwHPH
jlnrglFLvbrGRFGnvFZdNNFfPZddPThVhdPH
RjbjpgbnLGvpLgzBqBpmWmmzqTMS
FnsSpttPnPbNCFDtsPnFHQZTQZgcwgDDTfrfTHMZ
mRjzRzlvBvhjZrQmMMwfZZNN
ldzddlzLRlRWdhjdRLjhRWtJbJbNtJJpJPbCbGCWNG
wBwmNZBTmzzcVcmpzZqdMgPjnLSVlPgDPdbMdg
flJvGtHffHDddddbHjnb
RstrhfrhhRGFQtRhtftvQhvFZpsmpWwNlWqcWTccBNWswqNp
DPWhbzDlQLLlQbLDlLhPhLFNNJqCFGqnNJCCSCnGPnGN
wvwjtvtdwfssvSJgFFvGGSCFcp
mtdrZwwJsrtddrHRtZWbVThLlBzVTzhHQWhB
TsRRWctsTJMQZllggc
zDvhpbprgGvpvVlVQlZpQMJVlQ
rrrvFvGCDhDSrrrvChCgSstBNTSftWBjTdfWBN
JJdssBcLVGrgbBHWrH
QZTptvmvmlZpRDlMMMZCQvnjjFnrjWGFbjnrnFGWgZrz
TMRplDMggtwlppTlvhsJJqdcqwVPSSNcLd
JjTCCrcRvccPLmMP
NfGFPZlNnwBfPlbbbQZGqLHgzLghSmMBzSgvzmDMhv
ZfbnNQpQnZGlGlGpWTddjdTJdpRPTrCj
gWLblMMggdWsdRJlblMRMMqWDvPvcPPPccJPJVTZVZThmcDP
rQFfGfrCHrjnrtNTcPShTSPvvVLtmm
fQrCfLrpLHnCHwslqzqsslswzqRW
zpJtGlJPMPTlTjGJCDGCDljpdnvhhWnZnZnDwwmvnWDWWvdd
sHrVrSrRRRLNgLVBqSsZmWwvwcvwZjmwngmdbn
QsQQBrrLHTjPpTQzzP
JDlzHHzzptRDmbTMrrVQ
dRRNqnCBnmrQsVQQ
wFPCBNFgwjPwhgFNztftpJRPpzRvvHtZ
DlBhrDBPPwMWwhWchW
ntSqbbSJFJNqzVzjCfMvfSlSRWccRL
mVlHtNVtqldbJVmNHmdTTBBgrQQgGsPQdrDgsP
HWHNbBgvNLdcvQMnSf
wqqqVPDPhqwszFwrrszFfMdWthLcMdfhthSQfJSt
qVPVwTzFwFDpDrPDzDPFDPlCHjBGjTmZGjbWWGZBRTNjjH
GVgdWjllSqgjdgHqqlfmhwcpwCzhvZwMcScv
nsJQbLRQsNnzQDQQPPBbRBRhfZwpZcvwpvvmLCcvpcmfMM
DRJtnnRbBBnPztsrPzRBPbsFFHqqVrqggjFWqrgWjTGgFq
hhZJQPJFHGGlcWWslpNN
VwwwJjvwMtrCnwjDNDzlfscWszWW
nVStCrMqbVwqVqSqwnLPhTJFdRgJHZSFRLTP
vPgMbbRhhvMvNjjLWsWQsHQmHwBrmmBzww
tFctDnVFpppHVBTdzdTQwl
FtSFqSptfJCqqJStZCqDpDJMhvLLgLMgQgjgGZgGgMPLZg
zwsWgSGWLSVhPWhtLgLWhPVNQTmDrDQttZpdQtdpQDDQZQ
fjCHcvvjMDrppCQpVV
VMqRnVJMVLPzbRWhGh
mjRmzQlzDzNHWwDZ
FBfJBGqnnpfSVGnpJbJVfNtCsJHWZvrsNJCZrCNsvN
fZPBnfPqSBqdfpFbVnVSjgdcLLgRLjmgRhLLghlR
FSFnTcppdQtnnDhtzDfg
ZLGVmBLBVwZCVjjGqGhVwVVgzzbMDtNNvszMmMffNDDvtM
VZPJjBZVqBZZBjqwVqllpSTphhQFPShWSQcW
hTRdcLrCLgplLvBFGvlL
nZDZqzbDbDzRZtVNDzDWGwslsllBFpnlpGvJssFG
zbqjNWQVmVPrrRjRdRhS
VpNCbVHlHHZfflVfmchctqFcqQQjZmZM
WDSRGgsSvgJSRrnWgqQhmjBqmhqrtLqmQm
znSGTgDJnsDGzgwCwlpbCNwHzVtl
sTTTrpHFFFqTnQbbvfJdDzHHDLVV
CjMtgMgRvbPfjjvB
mhMvlhhWClvqshNTQQqsNN
tWFtFBzbwdFrpmdhdm
qTqDjJjJQQqMjTDLJjNqNqPNdmpcSmhdmhhmcrWZpdPGddcc
RjNQLJNTTJDDJRHHjQqnMWtlvvVvbtBvRVzgzgwgVg
CGdQjwdJrbBmpmZZZlRWcb
NgtMPVstgSzBLzhgzgLgDRlcmDWRmlZvcSmDSvvp
LhNsgPPLFPPsNzMhhVzPsGJBFqwQGfnqfQjdGdGfwr
CNbNdbzjCZpPNzjmzjsCMRJvnnMRGnsvJGRs
wrtdwTLWFcFWdFgwRRsnJGnGfTGJfMsq
FttcwgBtgVLgPldQSNZBzBpz
DjRZrrRmttRFDvDrFTZsnWnHVSTSSJVZJH
dNNhLqlLLqdCzfMMlCfSncTVVWcHdcVsVdSVnT
QqppMfzMfqWCwbRQrwFrrttQ
dwGjHrtjsdhfCHnPSpfMfDPpPDWS
lmNzzlLbFqcqNgzpWMSvbbvDQDGWDp
LBmglgmqBqmrwCGhCjVtBC
tvHgWZCCprlgpWglCtjPhLmPmhVdJFSzVzdJVmmQ
fBnTTnNNBnwfnNqcBbBBTbGJQQJhSSdQJJsmdJFSQGSmVV
cMcDwFbRfFRlHCRCZrrp
ZFWmgghzBgwgjWBzjzmRWWMmsVwnVrsdVdwNrrpnnVrPCnCP
GLLbtGqllctqvGJvSlQbJGsPnVdsdpsTPLsVppBCTVss
tJBStGSvctvDDfczmRgRZjzDjZmgzH
FMrLmsQQSWzCZBhpQJTQQZ
dPPVncVvPBJDCPhwJD
fvHbbVHvqnvvvBzgLbbGGmrbMr
mrZzrzqDrhZqDddSFrCGLLLPQPQBJPJJBnQq
TgbpGblWlMsjgWlgMfpNRgbRHHBnHHHtLpCJPCPBnBLJtQQL
sbTlblTlvRbbGblbFcdDzccVcDVvzzzd
zMzfzlGwSBMMSCMzhsPgfcPcfcbhjQPt
FHHqJVdJmFmdVrJdJppthscjGtqRPRcccgcQbR
rvNJJpLrvvLnJvNFFvZZZBWznBWGSDCMnCwz
";
}
