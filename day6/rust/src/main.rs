/**
*
* My first Rust code ever, and day 6 of advent of code.
* The most obvious optimisation I haven't put in place
* (aside all the ones based on me making a hash of rust)
* is skipping forward more than one position base on where the match is.
* e.g. if I come 'abcc' then I definitely can't 'pass' until the second
* `c` is in the first position.
*
* - based on where you find a match, you can skip forward more than one position
*
*/

fn main() {
    let myinput: Vec<char> = "hlfhfzffqnnrlnnvnmmgbgwgttbppcrcnnmdmfdmmgwwrrqnrrscrctcbttvcvtvvhchjhccjgjttmddplplqplqlbqlblrrbrvvprpffpmmzpmpcczjzzbwwfssvrrvggncgncgcwczzswwqqjjflffpwfpwpbwpwpdpbpvvqffcfcjffjllncczfzzmhzzmddgdrgrwwjzzdjjsnjsjfsjsjhhcchlccchqchhzzpnngdgndnpnppsdsggbvgvgpprqrqmmlzmzllvrrcvclcwczcqqcdcfcqqmmzbzdzdjzdjdmjdjzdjjcvjvcjvcvssltstttfbtftrfrlrdllrqqfssslccjdcjdjfdjfjqjnqjnjnrnddtnndtnnztzqztqztqzzpmzmggzrgrwwdqwdwcdwdnnmlmgmtmtstwssbffcnclclnclcjjcjpcpqcpqcpqpmqqfccpcjppnspsnnzggnpntndtdqtthwhnhwnwllzhlzhlzzghzghhlhvhwhjhfjjcnjnvjnjvvqccdmmgddllnmnrrdtdnncggfhgfglfgfmfnnpvvggznnwvnwwfgghrrfwrwzwszzzldzdldhlhblhblhbbbgjgsjggmqqmrrzggrhhwpwdpwdpwplpgpbggtssqffbqfbqbnnsqnqfnngcnnmwnmnbmmmslsjllbtbbpllltzzhgzztllsdllrvvhvjvbbhcbhchmchcctbcttvccgwcwpcchrcrdrdggcrrntrrfllcffbdfflrrrgbgrbbbdqbqjbbgbgrrqwqtwqwhwghwhzwwcswsnwnqqjhjhwhfwhffdfgddgjgsjgjhgglhlwhlhssfqfhhdmdnmnppdcddfzzhmhqqntqnnjvnjvjddcvcgcbgbbpjbjtbjjfgftgffplljfjrrhqqpddlssrvsrvrpppsllsdsqqqzzfttqsqzssjbbrnbnnrbbsrshsrrshrhwhbwbrrsrfrttfqtfqqfddvrvjrjvjsjhjsjdjqdjjlqjjjgcjcmcncfcrcwrwsrsslffzszmsszrsssrnrjjvbvpvcppptbbhhrddbcbggbqbmmsqqwggfpfbblmldmmpmwpwfwjfjsjnjmmpllccjzcjcwwpswshhpthhzchctcbcrrrrmvrvrdvvjmmvgmgwglghllvmllzlzzsvzzrmmhnnsjnnpvpwvpwwmvwwdqqdffhhhmccfgfvggchcctrrmdrrhrhnhnzzgpzzgttnhthvhzzqvvvwpwqpqdppsnnrgnnhphphmhcmcrmrvvqlvqqsccqhchzhwwmvmzmczzgsgdsggthgglrlnrlllbdllhwlwltwwcswsgssbhbsbvsbsbwbhbnncrcllttbrbppjccfpfhhgshschsccmrcmmcrrrzvvrcrggmwgwjwnwjjbffjddjnjgngqgdgnndznndvvfqfgfvvrvqrvvpllnsszbsbdbbdzbdbzbqbzznrznzjzpptcptccvwccfscffrftrrsnsvvswvvhbhzzfbzffncchhcnngzzcpcmmfttsntnjjsccqbcqqmzzgppdhppdtppmffgtgvvlzlpptdtttdppqjqtqctcrrzsswwtnwtnwnqqvbbdgjhvmmzpnhfvsbddzhgdwcnfdstvhhbzlzcfjwhlptbhmbmblprtsdmrdhbbbwpplnzgdnrzjmgzgpqbggnqvwwtntzgfwqrztqtdrsnhpfzswptggnvbszdcrmrhhtlrrfnpqrnpwrbmhlfwmdqqdbqrwbzqjbzwrgmbgrtzrhdclqfgsrtsgfwqrnnqgwsncmpgffggssrqvwjlhpsghbqdtzwmvzzvcmzsjqvprvcqwqjbcqcqrhpwwcsrscgmfdppbgvmnrdfrppblznbstnjzwwgstjvtprjbhtpdfgrhdjnjmnlbfwggzhcngvcwvcfpcwdtdppwjrdzsnjlnrzbfqqshlnzvwsmscgpfwjzhtwgfwgzdhbdwwzbsmfwwbmvrlrpswnjlmfbfzhwvcmgwfzssmmtjlwtrpwpwgnspbgchdncbfcpjsvtzjqtwqwjwgbhrbwvhqbcstsgsnwsjmhrlrvzgqhqfrmnrjdrhdjwcwctpdrzctlvnfzmzwhsnfprlzgzjpqvzchlmvbhffhpfjtvsdbvbdmwgvmqpflhwwndbqthmmwshdtspsrvqdflmmzwbqbqmpfdwjmvpbzdnqzfmhzdgldqjjvgpfcqftvjzwnzmfqdggrwlfzdhjnhmtrjbnllgqpntwmhnwtglnqdwbqdblpwnnrdwzpsqzfwqcmhqhnpsdcwvdldphgnrtqzdbnnzdzfttldrqcztlvlrgpdqzrcthslmtqhfvbzrfgnlrprcpbsctqhspbhnjtzrzhqjzszbzdthttqmbznzssftztwlggmdqqdtfllqjzjtvpgjfhtbwtbmtjplqnbdmsvlnqcwtdbdvfjnzgsmpnhbvvwwfbrgffjqfsccdjdwvbsdhqwfzvcpjzjbdjgrdctjplhwbdhhnbnwstvndnnwtsgbhzbvwdshvmnbwsthlrggtmddvjbfzfrnrdrqfjpslrccctzpjbwpdbhlbzfmwbltcqfngdprvfhgcszdtpnrcpdmllfnlspgrdrpwqmqbmrglvlrsmrfqrtzzgjcvqtqzpmghjrvmdmvvqztrjzbzjwdqsmrwpqnswbzhjbzzhdvmnfdsztzdzrjssgnnfqvbtsqrrmcppjgrmnstrnrlwjvvcczqlcbmwqzdfpssfwdfrvwtstwchdgwtrhhcmppcqlmrqlnwqccfphsdhbsbmtjvpcwwjrmrllbpnrmpbvwgwbftpdpphccwqcblcnvvbbppscmnjqgddllbnbvmmqzdffrrjtqwllzgpqrmnlfqrptzqmdmnrfnjpvqvjbsqrhljslgqqcqqtmtbwjrpphtjgjbqpzmzrzjrfjwcdcnbsjfljclffjplnrrcfbmhphtcjrzlrvvjcznpgpnrdwwqvgnbnzqnlcghhgwvhqbvjzfbdvhrzlqfbtlqhpltfjlfpbnjbphmmpntzqgjmwjtchwmlwvfjmfflqzpqnvvrgnbddzlfpdpdjghfgbsfddjspnfdwvqppncmdgfrnvrpcrflhgjgbwdsbwblfcwbtlrrnjjdhbvmrzgsvjwgfnnhqfbvhprlmwwqgclzlbqbrdspcbhftmdscsmpwrggrmnsvjphjmzmmrlrhnmdhwjlbmjchtvsrcplfspsssjznmzcrqnsjjtwjzvlhshbptqwwvjhjvzrhphphsbphpnzpfbwcdnqrhrrvlrwrztlpqnrcfzrncsvpzqzgslrlrwhvtgjmfncldqmvshlmnlpqbgvnwqfcthgrgllmqrjqmfgznspgltpptglpdcvhtzsprprbldbzhbmjsqzwvjggwhsczltcvgwqhspzpzvljwqjgrgtwswjdswlzjzslrsslvqzncjwhbbjpbdthqpgmhfglggmlrgwdsplgscrwstntvrhjzjjlshtgmnnhvsjwfmcjbpzjcstmnpvtbgrfcfdwjljsrfhdphrdcslwhgvlnwltwchplvfzntfgcnlsvzrvnnczhhqdlwjvqprhmtjdtwmppffmszzzqtfrgnhnzqgqzhrjzgntcszstrfhhtptgvswvzvjcgcntmhzzmdgsmtgzhpfvqfnwmsjdhtfgmmbrrfsdlptchqqzqdqjncmtpznfssrcnmcdnthglmfzsfgltrndqsfmdftmfgchbwmzgrtjvgqtshlltthnnpqnzfrchzhdzrrnpzvfzblrmhwdwjnqdptlbvndmmlhzhvsfdlmlhqrgqqzsdqtpczwcrwcbsftvvphfbwjrvrnrcqbbcsqgnhltwzvllljcvpwjgslbmngcdmpdvjlgcnrzwqjdgrblncpqmrgjmpqjzvdmcmwfnwqlszdgwqdfznhsnpsjrfwrqpqmpvhstmzgqblfmcfvwljbhdfhdmqcvrwnqcstwtzgmng".chars().collect();
    implementation_one(&myinput);
    implementation_two(&myinput);
}

fn implementation_one(myinput: &Vec<char>) {
    /*
     This is a super-dumb implementation that checks the last four characters
    to look for duplicates, in the order 1 = 2, 1 = 3, 1 = 4, 2 = 3, 2 = 4, 3 = 4
    and then on any detection moves forward just one position.

    It works, but it will do literally the maximum number of checks.
    */
    let mut check_count = 0;

    for idx in 4..myinput.len() {
        let mut matched = true;
        'outer: for subidx in (idx - 4)..idx {
            for subsubidx in (subidx + 1)..idx {
                check_count += 1;
                if myinput[subidx] == myinput[subsubidx] {
                    matched = false;
                    break 'outer;
                }
            }
        }
        if matched {
            println!("{}", idx);
            println!("{:?}", &myinput[idx - 4..idx]);
            println!("{} checks comparisons performed", check_count);
            return;
        }
    }
}

fn implementation_two(myinput: &Vec<char>) {
    /*
    This is a less dumb implementation implementation:
    Instead of searching 1 = 2, 1 = 3, 1 = 4, 2 = 3, ...
    it checks 4 = 3, 4 = 2, 4 = 1, 3 = 2, ...
    and if there is a match then it moves forward enough
    to put that match out of the 4-char range, which means
    it is in the next possible position that could lead to
    success.
    This leads to many fewer unnecessary comparisons being performed.
    There's probably a much more efficient way though - maybe using
    vector comparison to check all 4 chars at once?
    */
    let mut check_count = 0;
    let mut offset = 4;
    let input_len = myinput.len();

    let mut matched;

    while offset < input_len {
        matched = true;
        'outer: for subidx in (offset - 4..offset).rev() {
            for subsubidx in (offset - 4..subidx).rev() {
                check_count += 1;
                if myinput[subsubidx] == myinput[subidx] {
                    offset = subsubidx + 5;
                    matched = false;
                    break 'outer;
                }
            }
        }
        if matched {
            println!("{}", offset);
            println!("{:?}", &myinput[offset - 4..offset]);
            println!("{} checks comparisons performed", check_count);
            return;
        }
    }
}
