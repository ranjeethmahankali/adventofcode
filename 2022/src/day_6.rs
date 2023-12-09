/*
The preparations are finally complete; you and the Elves leave camp on
foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you
a handheld device. He says that it has many fancy features, but the
most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing
with signal-based systems, he convinced the other Elves that it would
be okay to give you their one malfunctioning device - surely you'll
have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful
sparks.

To be able to communicate with the Elves, the device needs to lock on
to their signal. The signal is a series of seemingly-random characters
that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the
device that detects a start-of-packet marker in the datastream. In the
protocol being used by the Elves, the start of a packet is indicated
by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle
input); your subroutine needs to identify the first position where the
four most recently received characters were all
different. Specifically, it needs to report the number of characters
from the beginning of the buffer to the end of the first such
four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there
haven't been enough characters received yet to find the marker. The
first time a marker could occur is after the fourth character is
received, making the most recent four characters mjqj. Because j is
repeated, this isn't a marker.

The first time a marker appears is after the seventh character
arrives. Once it does, the last four characters received are jpqm,
which are all different. In this case, your subroutine should report
the value 7, because the first start-of-packet marker is complete
after 7 characters have been processed.

Here are a few more examples:

* bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
* nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
* nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
* zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

How many characters need to be processed before the first
start-of-packet marker is detected?

--- Part Two ---

Your device's communication system is correctly detecting packets, but
still isn't working. It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker,
except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of
the above examples:

* mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
* bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
* nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
* nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
* zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26

How many characters need to be processed before the first
start-of-message marker is detected?

*/

#[cfg(test)]
mod test {
    use itertools::Itertools;

    fn part_1(input: &str) -> usize {
        input
            .trim()
            .as_bytes()
            .windows(4)
            .take_while(|w| {
                w[0] == w[1]
                    || w[0] == w[2]
                    || w[0] == w[3]
                    || w[1] == w[2]
                    || w[1] == w[3]
                    || w[2] == w[3]
            })
            .count()
            + 4
    }

    fn part_2(input: &str) -> usize {
        let mut input = input
            .trim()
            .as_bytes()
            .iter()
            .enumerate()
            .collect::<Vec<_>>();
        let mut flags = vec![true; input.len()];
        input.sort_by_key(|(i, c)| (*c, *i));
        for (_, mut group) in &input.into_iter().group_by(|(_i, c)| *c) {
            let (mut prev, _) = match group.next() {
                Some(e) => e,
                None => continue,
            };
            while let Some((i, _c)) = group.next() {
                if i - prev < 14 {
                    /*
                    i and prev are in the same group which means the
                    i-th and prev-th characters are the same. They are
                    within 14 of each other that means it is not
                    possible for the start of a window of 14 unique
                    elements to be in the range start..end. We record
                    this fact by setting the flags to false.
                     */
                    let start = if i < 13 { 0 } else { i - 13 };
                    let end = prev + 1;
                    flags[start..end].fill(false);
                }
                prev = i;
            }
        }
        flags
            .iter()
            .enumerate()
            .filter(|(_i, &f)| f)
            .next()
            .unwrap()
            .0
            + 14
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(part_1(INPUT), 1640);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
        assert_eq!(part_2(INPUT), 3613);
    }

    const INPUT: &str = "
vftfrfcrfrpptffhnnnsznzgngqgsssczcjcdcssmgmtmnmqnmmfttbdttqtggmgpmmqrqzrzttptpdt\
pddqfdffgjgzjzczjcjjvttvmmrwmrwwjtwjttnccgbcbjbgjjcvvsqshszhzvhvsstjjljcczmzjmzz\
dtzzhfhvvvflljtllfqqdmmmhmfhffzhhtgtssqppgcppfjjrnngwwrvrvqvpppcspsrrsqqqlmmpcmc\
jmmjhmhzzpnnggmjjrprptpccdggcrrwfwggsfggbbqlqflqqcsqqccqzqbbnbmmlzllnhnwwsjsgstg\
ttfdtffzfvzvbzzrlzlppcsctcncrncnjnfnqnvngnzzscslsppdqdbbthhqvqfvqqvccvdcddsllnwn\
qwwsccjmcjmmfmppnzpzffslffjrfjftftrffwcwppfsftssdwdzzffnwndnvnggbbfbgfbbfmbbjzzd\
qdtqtfqqpzzqpzpllvrrbqqbcqqzvznzmmwcwtwftwfwtftvtddhldldblbzlbzlllcjjwllvcllqplq\
plpzlzqzsqzsznnqwnqqrcqqnrrhmmqbqqlbqqrhrrdgrrsqssnwsnswsgsfsbbdhbdbqqtsqsbsmmbt\
mmghhcththrrnvvnlvvdvwwjwmwbmmvtvztzmtmrmprpvvmbmrbrsbbpsbbtccwgccbctbccjrjcjtcc\
rrdnnwwqrwwjpjtptcpcfpfvfjjtqtnqqvzqqfhfrftrrgqgbqqtwwthwhqwqjqmmlglnlrnlrnnshsm\
spmmfnnqwqgwqqprpbbjbhjbbfqftfwwjhhcpcgcpcvppgfgcfgffbzbfzbzqzccmppndppspbssnbbp\
dpjpgpngpnggtztjzjdjhjmmwrmmqhhcvvvltvllvmvpvlplwplpmllgpllczlzpzczwzswwqgqbqppc\
jjhjddgccfmmctcjcscczzzjzwzjjsccdmdppdrdccmttjpttqvqdqtdqtqhqchhhtbbddmhhwdhdwdj\
dccnhcnndpdldvldlvddbjdbjbppjbpbssqnqnmmmclcggrmmdnnwmwddvjvdvrvzzglzlnnllhqqrbb\
cvvdtttdvvnfnqqstszsjsrscspszppfjjfttgqttdpdbbbvnndbdcdqdrdhhdhhdjjjzzhqqrcrvccr\
vrtvtnvnpppccpdpccjgjmjzzbsbgsbstsbbtdttrqtrrsrdrdcdjcddbcbjcbjbwbbslbbbbnmmtsmm\
rwrnwwqtqzqpprhrzhrhhldddrpdrpdpwdpwdpwwlmmzssbwswppldlmddphhnfhhczhhqrrdgdjjdtt\
ztftzffdvddqnqvvlbbncnffssbnbfnnzbnznhzhdzzrqzzptzppsnsnzntzzfwfzzvrzrbzrbzrbzrr\
qqltqlqppbwppmvpmvpmmbnnbhbmbdmbbhmmngmgmhgmghhvttzzfvfggrrchrrbffzjzlzsllbqqhqr\
rmqrrlzzsqzqqmhhmnmtntstnsnhsnnwwpdphhgjhjccbcqcjcbjbttmssqffjzjmzmqqrbbfnffcbfc\
fzfrzztftntbntttlddvwdvdcdhcdhdbbjfjzjwpbcvlqvcwjrcjssdfmgwrhrjvhpgqsbtzqqdwjrqs\
jplqjdzdcrtvqlcrfpcgwpjnbpcmbwnwbzhcvjvzzpvqnzdqdgpfrvdpfdmpprmzmghdfjjzfqjqcbpl\
wntzmsrpqclgrqzhlsgwffqqntswnjsmrcpjlsvdrmcwdgqzsbsbvhbszqgwqffcgbqmjrfjdvbpwbrz\
wbjgvvjchwfscrhrtzbghjlcnsqqhdgqtdcqrrpsbzqvjwptblszrtffhwcvbngnsdjgpscfzwrncwlp\
fqgwdzffsqmjcbrlffftpvhjchmgmgqvjnpfsjnfzddqjsfqcpjgfrfgrtlmtfqphjfmdcvmghrdqvbb\
bhlstgpcgmqnwpdjwbrdbntbpncnztmnmzmsjzrwjmccqslngrvbjcjjgcvnvhsslfhwpwtjjcwgzqpd\
vqrlbttnnwdphztmwcdlvlqggrdprmzdfpbfhmsgqznzjhdqpmhfphqcvbfqmhnfpcvstrhdbtmljnnh\
qtfnpdwnszfrflsbbqjsvbvggzfhlcljwlrlfnlwlzzllzbqftlwzqsvwlldslnfnnbhlwmhqhrjlqzp\
dsjlhjncpwtnpnqvjtzzjnmdntmbjbcwphplbcwdfcqbhhnjnjsfplgwbrqpqmghnzvdprtlmgvwhdgp\
dwfvdtqtnqdvbntmsrhftwvrgwcbvhnwmhfggdzfgbdfqsbngmvjgssclqlhqggwndzzhcrzrmnzggnv\
bbbpzfdjtnlvlnprjtlljhtdqjlddmjdswjrwhwdbbbrmpwsgpfgnplbtzfzlvgnfrvvnbjtsglbzmtm\
cjbbjclmjgtdrrbpbbqzqnvrgqssfrwhrtpsbgsvnnfbmqgbvhshmpqtqljlpwptqzprdbgnzmlgtrvg\
nmmfhtccsvbmsfnlnzwhrmcnwmmhgwclghgmspwjvbgqrbbhhrglstdntwnvcgdcmwgbwrwhsqzsdnqs\
vmcbzrvtztspshrfmqbtrdtnsqdllqzdcbmvbdswzrrzchqgpgmhvjgwgnpqfwcmchsnqssnnslzwtwv\
bqjfbhlbdjzjrqjvsshcvcbwhsvvwtmjwjfgszmvfzclbqjhnczqcznprwzjnlgmdgbfjnrqfgvstcld\
nttbjmhsqgqlmzqtsqngvmdvrwcjtfljzdmnndnrbqnlqtmsqngflwsghzzsfcdnttpblqqhmtgnqmcd\
fmclsvgnsfpnnfssqzjtdsjbjnnmqhfctlddtwrqlpczlzvhddrtjfwgqhfcvchzqgfhdbfpbvtqcjvc\
hqmwhvwjrbtcjgbfqjdsbfpgjrzvlgqwphshnqrvqsppgsnfswvrzpwmdfmmwntnsddzppffjvbnshhq\
gwclvtpjzvlbdzblhhmhrmjrpmltglsdffnstsdqwjhnjccqhbdrgnwmpwczflfvsbznpphgpbfzffbc\
dnbrqbwddlhvgqsdmdhmlzbdztrrswsbvdgptvhhcdtwzqqhzqpswwvftppwvwhrspfqwppjbdlhcchl\
ftjhrpwhtvqhmwwtcbfhgbqzvzdlwlzwcsqgvmmsnhrfmwwpcrjlsgzmgdqstlwbzrzbqfnpqffmjqbq\
qzcnsqrfstnwjflwlpfgcjwjdvtjslrcpgwsvrbvjtzqvnjlqrvvwjhzbzqqjhcrbdtwqjbtmwfrmcgn\
bdcrhrvlcgrgtglpfmvpgwbzccddlrbsjzwbgwthhdmjjtpchtsbnnpgqfcpmsrgvqwhcdqzmtzlzbfd\
gmvtqzzdcrnhtlcwnmhdjtwdsrfnlmwpnfwdrptclvwrnwrnntwwqvfmjgswbtqcvmbfbgstvsntndzh\
jjnjfblqdqrgchchtgdwtvlqzrlpsqgbltjzjngdscdczwzhnlszpdnvnbrmfmjpdzvjfgvtwtpwdjjf\
gspbvtdjrwzncdpbsthgcwvvdbbvpvqdqpzjmlzhtjmjwmzsmrcstrsvbccqhppwrtmslggqbgglgrgf\
frbwzmbghfqclwwgssgghqjgfjgvwjhhwnntnrnhmfslqpmwzlggsbmrjjgfzfpjlvmshfsdjtshdlfz\
vjtlqwjbbgmnjhrhtpbgvcsjvwzlqvfchhpfwsbhcztmdgfzgsmszwfbvvgmgpqsrbzvtpmpqdvhgrjm\
mspnswjrjnjqfgjwsfbzhwhtlfwjfdhgsvcwqlbznqlnhsmzwltfwclcwgjdbhqvjbbchmcpptmpdqzw\
pfwrbmchpbqndtmdrwtcvlmrrnvhnpzwqcwwgmcblzvnzbzsspwchtqvjmphqtzgwdzqlbvgdjbssdjw\
ljhlsjwzrdvqtrzcdwszqgfdwgnqdrmssqqhtblqzdhtqtqmlbbfhzvlbrphcjhzpvvshjffnsjcbgvn\
gnsjmfdbgfzphjc";
}
