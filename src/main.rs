mod solution;

fn main() {
    // Measure start time
    let start_time: std::time::Instant = std::time::Instant::now();
    let s = "xjpguhvytyjcknhjqkwelhjqbdgtwxgvgxbdeydxwozidiutuqafxjxaodtkdbfjyiocgtbfhcplmjggbgoarlcgpxssyadyiuapndwxhlitvoayvqzobbuqzpkzpqyzkaqzgmwnyghvvjtszuiawdtxufylvwkhzbhfpfsnmbkjkedlylowqjvkquxmsivrlewakrqysahfgmqhxgfqpbcgxaupkrhvwfviwngrqpwybohaxnsoqvwpxqehkncgvzqtpwkflidoznqwcjksehjdzpkjdmranhtcfejsopgncxjeguymbhpcwbmbpfbcnvhsbqnpftdjsonainoludqtgcwvjyywvhryxepfzuqsjgstthhqmxltbhokfojcvcavgqchmszvyupudykrvvmwedikrroptrmbjojvgkrheibjgnbdknboqjakbpbwgnyrbhmjtfqantjvgmaqcbhulhgowhkeukvxrkhnpznfvwcdldwnedjpkqfjxqnualruvahmcwrxuuafxwubzetmwyvtqkntvhnshwhjsyimujuthoxjuqvqqqmhazayipsqnzbfaktuvpocennadirvadcdeedpvvfixipxujtpajugwhhbsaxsfbvliaadwhmvqbsmmnenxavvhcxbcwwjxtvfuvlqdxlvafhpsnernznxemygiuqfonniiyanxnkzuuoohugvwvsajsirnyydnnnwnplkcwkyqamxvuurrmrafztuauzphmlvdzhfvrflurkpmfidtbgycbuevtufhhakgjrdbwqvqbmciwhbxpcbrwgmscrbjtmsffvgemdupryxphaoxcpobxcvbwwnrkfwscewqjsfcqerzffwjxmmwrhynelgosfiujenvwsxozpogwmrtbeqslqhrbnitsqpevcztxykynaemmvhnbzhnpogqeolyfdccfdxecjcrjidyelnhmvuclduprioylscswaxylbpvkvvqikxuhuytxtkqbapottgrvfphjgetdzjljigrcembzwsczjqsczlygcfpijkmktzvehmgoaknzcqylisnjdlqfshpbsdnndjrkxayykoxogxzqpoascsxubmytsljvuahucisowrccobudsuxuouoqimlaauxwxhqbpkqldsptwjyogviurymclyenueltlcvaollufcnbnmptjzqbycflcjyxnjsynnaealygpljdzzyjyomyrtjvchustnsgctkdgklwwubxvziwouuhpecslxmgmepoxbremcckzdhucqqqmlzcpcwcbilnmabkbtqpxszwvhtzzjslwrnntlsutdjgflsigkyfcxezexydiqrfigudsmalrjtwunfcxdibcmajjbotrfybmtfghftzqpxlcepcjxdmlgvwhjqarqcdlhltoeuettryyhahgfvsnqqucgxtzzykijfwpbcjvujvdjelqadeswawcxpdwpoeyvcqxfzubipetvpjxvpqtqmxpebotpuumxkjelffvwlosczpzrhsjwqycrmvihrugbgkolrjiezcgbtisbadzsbblqytzsqfvyrklitvmvxuyrcqufvvzwyloygnqwsmwjwitrdhobcmugcqnzlnwlykjeaadsmzekhxdlhsojekrjafinseysrjyrjblxbrjkrnvyflhjvasxfbkzhkraustdtfdwymhpzengqwqnxklelvetixvcpphjwkhuzokavxhlwzatjlxxjdqrbnvsccdypltqzdswcbhyaktmxrjgwbzxowqrzvpqgkiipaescoscymovfxebyfbpctgdoxvxidfxdjrfzmkxaavhabiyilpkevpvvksfpzetiwakkkjklgrlhblqnbctyuqtgkawjfrubrenenxpuqcdrptgsyctusmadnyospivhminahewxgzoxvxqtzjntxpymongdvdmknzkudirlhijchbxgkmbjcawsnevkikuvjgspolcyvlacmakymmiqmgibkensqiqbqiqfttdpgfrvfevsqdkelthwzuqpegqvqjakefbmkuhsyfmokwswpbsqwkfatyvjjxvncwzprjhpoteypywhcqxybfaufyfovbbaxcponygdrkeikarmrrmuwnqblvpiwsiuwzkkxqnqctbpusdnlqhhfxkssbapvllskvekmtcqndfhyjujbdtgafauhclenwwaucmiwoyjugqupmfspaarganpcztqxssruebqucbqirkzfsrwsrnardpvclnzfftblusgyvwgnjfudyrvpgwijngnatnfbihmebudwtjlerihrbchjartqzistxyufhikkdpiwauarejjfnsooljglsygpyaxhijrnyalywnsawdfkxtaidgvxgbmhdloougbsipteclezqljnejvjrtgzuvgygvoddrxlgqrjdxititgoeeavxiwrfdroahrdzoqfhhokgygormevsespnpjsscgukzxjopoxyfjedpuxeyswfnoucxmwbvqlwpwmgljestkviesoennjabfeauabpsnljjapwjvochmnngbrvodxribredttvihgthxsssivbwkodniaelyvvzpadkvasejnngfgbqdcmprpczqgmoejptlsdjvxpekdmslniqqufjmhieqwuufjntescbpthbyttjhdbzaiiosssioocvzrqdjugaonbmhxyqczpcixqarkkfaocaftfqnmsbbtqisoyvppxzoqbfclmdzpdgkiyxwqbymtiehjzyyzynrzutnhymwbvimvhkmiiadtekcgjafjpyikrvtqkrthzhcgsqrcquvxhxdsakbrkldbjwttnpcowgvqzotriqorotjqfmhpylthhocxdelcmiulwpdhgtywpkmuwvmugfbqtfpzlcdylxjhnoovkprzzdtvafqjmtbizqhmsmkdlwnykdtusmvrrpnswfbjacrbuaommysxwhyjktdfgzwzqlrmssxtwowqqkfclxchgcqqvwvdxudnhwbarzvnpregclknkowqqniojgtgayvhvyjozebpwhxasjncajuqydghjcplakuxlelkipbgwygrkvvkfqcdvlnenerpplpapcmatogqmnjyiekpwpvrakxpoqgfcxhtcutvicnwrwvbdhtbwovyaupolyunxdizxcvfgiezhbamitnhjkhjfxaqxwfuznuzppgxzkwilxuuskdewkpbhprenwbpkvobmubnfxwqwsmrepvbakejcwqpuregmukaplnuklmjgzamqxpqjualsqdmhjvvefxtskpeybngcpstmilweljwdoimyfhcmgxermlrpyxuqrnebycfmmbpamcyrlceszkllvedwbxmumqwktbyhdojrskidmoxmbizymeupbimnbiawlydoomfgyqmlgjzhuygifcagnmwowykhypyndfvcvhpetolpotztybclpyblwlvuctjhyflwoaajonydhawfbysrytewgztiucrvhdrydthsgixpkvwlwoeujlrpmkzhorcywvwzoftwnsoxoklkbrekcxcrjdyywcwszsupxnlngbmwmxgprmbvkdmthmrdqnyphsehhsuptilhiryzeauqdhjmtdsmqqbakihtcdjxluhtofsufpklwvxryrdrjhrtpyntdyqouxkideeitotrmtlkkqbuxsposchvaamxxyfccknyairmbczovaiuvzjneslguzdsxjwbvjzxsrmvvljqntlitwyxqldlkjfjsbkpnmohfaecnqtblgleelduwjhismtmqgdfurozusbhkwkweyckjihitosldozvuccovqppksxvrjtxhvitdrbwfvjkjkhdmjtkbizodyluietpzbifslbahnmqxuwmfpwjaxzdwkzeqstrweworaqypfrmznagewreuqjqaiwsdrkzvgpnignxnemotmuylmcheozhyvzbmjaksqzcyoclvozocvmnjrwofvvdswhhghtazucziekdulsxjgkszjieefkxcrekaxkatozbtmhnzbmihzdhinnmtzlxsrjtqtvjjwleksukvgucfzlnpbcianhthqoxllhuhuzsotejbanhazwpcyzcoixvanulydhgxganbeydgmminizphatxitsigmvfqdnplnfptdszrgieohvxirwskodqdyxvdkmpzresxyuoeevunsuxjqqthvkmthhxuvotnsoksiayovsboobzfttoofahmhggcucroqdgaeeqbzrppupunkkbpkldtrkymopcgvjgzpwaopsekjaxtlzixkltdxrrliurddzesxfjnzpzipwbcxlcjwvpwmghwabafcgyanjnmymupkxukiwvhtkdhrmdrnfxsmxszihogtixfirpsplzixcrorvigcfyqeqqmxeusoraylprccsnaveqobyueftullmxjstdjndhavacztpzqusevqybwtwhfihodctmpxvpswurpjthfllddlezfcjknsaquvcmsxdmvzemjztqkgtpsarzcalpunhqiledlipgjttsuolgvewpenohnbyjogzyrebeorlxmgshudnpjjgowwxlxxunfwmzapdqgonvuhcrkriubpkzljnlghymdmlfcqvkflfbsjsfbdbculdfwqscatqffdljuiubvbcqlxvmcwqwjvbhmwjmpcrufegbpackdhaoexcgvucgqfncbzqsbjniotkfvmpytspzprflmjrerhgugynhhapxvzcsosqhmhjbzqonaittpznvzaegctezvgrjaksorbsssghuqanhbaeadihfenfzvykwiekcgcualeubejlglpioyrwceddabnymrioznkbaoxdtgobsejicbeghhjhjyfvrqltfvufksifyxgsdrbhufncnyjywrvphgimddtnxbsxayqdsrkmyxonxantrilaqtouyhjvicvlclouebjeaxsyxftqqeqgaecynmwyqrjuexpiyymbxgzxmsnexgkxmpxabvytmhnsgeahepicxhbjbonywaxjrxlusjnhsazyfchlrpnqyqaahpadryoivzepkrwcuwdbykmrachasjazbbfsbtdwvhnfbkivgnwgxkxzmeahqagrbnlchqacaqjbatyigwoggnfvtfcjikclyoqheslgiuhiohswdickvihrpjaxtflttbaztlgcgpmwxhsapvmnfteueguylfrgiugbfmflduhadcdsxphellypuupfbjojduniiuwlqfothrmggvkthljdfakjjysoshzcevquceokvcqdxbxgoijtkucwuxknglrkghfjlvviznowqnfexqyhkcdfbquibnskvzviwstvfhuwubatraaedglgwfozujlpkgahategcacybcrtftxiziqxpfxjqibcrdlryqzasbaugrplmmvmwljnsgwkrznkcydaqdcjgcfmvuziguweifrcopnhpcrtcuwtzyegdjsadsklogryoibczqjquckwygrygxeliymlswyhfphtxkxzaipwmzvkhoiomobunnifmgorwwmvgjujtmhflcpvraldomzbahjmqzfovrjecgpvuwafzrcqrnvicwlceuqwuxkrqvxsdmpxjrfkihccxzmzvxdbuvxqshhkdhcgttgeklousqyrpkqnitocqoskvbuaiwjeppibcxwupumhfeupakrqylbwovyxujblalncilxaflhmrdbrpuiqhlmwgmvawyowjbzumyutldicilwxggnprblzoicmgqkqrjkfjgywjgbrsxoaderwffvvnxhunsmedwjpcklnqogklwmqaemijidyfnsvfezkclzgvntnbbypymfysugdemcjzuggbgqftqmofhbgjbvhqdhixqmbcomdktjnbzturhkwonfxpagffqpegdfitulgpwtsvoopvylklqjctsjaizfoemyyglexhxpeodtjdhtpzftuxdvfeavimtgvemslmkranljtsfkrkdmjghomjjxvedqislvevmekzndtsnlerznzidorolosqhciszmnoszngdhasuflvundybwommhetlpnlbczucochvczrjlmgyrgbnuncdtvpilamnbippkwnoyeajrijiokyizaosxddifpwiznxlmkbkpdvileqzqqkpqyjodoyifuseippuctgtwbbihthxktmarxqwmpgrjyytonpsgvldymnffwepqssjqigexovjntedjwvrtgwssjzzgepywhjorpsreoctjgwucrmyxksrurqcxhcuoliidbzhrbccjyxoplmovefrxxvvfxpvjzdmcevvfxyrvxfmkrcfxjzugurnsijdiormtmialirihyurryyohxlnucbmlmrvaihvwpyhzrrgqnxhlwysvjhplkdywutzebwaswjsoaygnwnyunqpwahkkkijhcilfgmxdvptwqzlmokicczycgkprtyyxijcoxbtvrmthlevcodetcexlpmckkcjunljlmegfrboeflgwqmpvrmgibiulmdgzqrmcvukmvatbmzxemozfafndpjpdmxdcqrglmsajttkhujniznncucfklunxtsbjkixyczhvuueofsxfhmhbpmnchdccxdmhnlhqkpneluuqotvvgcyxpmzyrdaojo".to_string();
    let words = vec!["twjyogviurymclyenueltlcvao","tmilweljwdoimyfhcmgxermlrp","ikuvjgspolcyvlacmakymmiqmg","agrbnlchqacaqjbatyigwoggnf","mbzwsczjqsczlygcfpijkmktzv","vljqntlitwyxqldlkjfjsbkpnm","beqslqhrbnitsqpevcztxykyna","usqyrpkqnitocqoskvbuaiwjep","ibkensqiqbqiqfttdpgfrvfevs","wszsupxnlngbmwmxgprmbvkdmt","fpzetiwakkkjklgrlhblqnbcty","sxdmvzemjztqkgtpsarzcalpun","wceddabnymrioznkbaoxdtgobs","hpecslxmgmepoxbremcckzdhuc","ztuauzphmlvdzhfvrflurkpmfi","ptrmbjojvgkrheibjgnbdknboq","vgjujtmhflcpvraldomzbahjmq","ygormevsespnpjsscgukzxjopo","qdkelthwzuqpegqvqjakefbmku","hsazyfchlrpnqyqaahpadryoiv","ickvihrpjaxtflttbaztlgcgpm","hnshwhjsyimujuthoxjuqvqqqm","ejicbeghhjhjyfvrqltfvufksi","hustnsgctkdgklwwubxvziwouu","jrfzmkxaavhabiyilpkevpvvks","reuqjqaiwsdrkzvgpnignxnemo","wyloygnqwsmwjwitrdhobcmugc","fvwlosczpzrhsjwqycrmvihrug","ehmgoaknzcqylisnjdlqfshpbs","irvadcdeedpvvfixipxujtpaju","mcwrxuuafxwubzetmwyvtqkntv","lcjwvpwmghwabafcgyanjnmymu","hdloougbsipteclezqljnejvjr","hmrdqnyphsehhsuptilhiryzea","wunfcxdibcmajjbotrfybmtfgh","aeeqbzrppupunkkbpkldtrkymo","rbnvsccdypltqzdswcbhyaktmx","jqqthvkmthhxuvotnsoksiayov","uqtgkawjfrubrenenxpuqcdrpt","mvmwljnsgwkrznkcydaqdcjgcf","wcdldwnedjpkqfjxqnualruvah","bamitnhjkhjfxaqxwfuznuzppg","moxmbizymeupbimnbiawlydoom","xyfjedpuxeyswfnoucxmwbvqlw","aftfqnmsbbtqisoyvppxzoqbfc","zepkrwcuwdbykmrachasjazbbf","akjjysoshzcevquceokvcqdxbx","pcgvjgzpwaopsekjaxtlzixklt","zucziekdulsxjgkszjieefkxcr","jrnyalywnsawdfkxtaidgvxgbm","xpkvwlwoeujlrpmkzhorcywvwz","qzotriqorotjqfmhpylthhocxd","wymhpzengqwqnxklelvetixvcp","ceuqwuxkrqvxsdmpxjrfkihccx","iwstvfhuwubatraaedglgwfozu","ohfaecnqtblgleelduwjhismtm","ekaxkatozbtmhnzbmihzdhinnm","uvxhxdsakbrkldbjwttnpcowgv","vafhpsnernznxemygiuqfonnii","sbtdwvhnfbkivgnwgxkxzmeahq","gwhhbsaxsfbvliaadwhmvqbsmm","yueftullmxjstdjndhavacztpz","qgdfurozusbhkwkweyckjihito","hsyfmokwswpbsqwkfatyvjjxvn","gxtzzykijfwpbcjvujvdjelqad","schvaamxxyfccknyairmbczova","prpczqgmoejptlsdjvxpekdmsl","tpzbifslbahnmqxuwmfpwjaxzd","zmzvxdbuvxqshhkdhcgttgeklo","bapottgrvfphjgetdzjljigrce","qchmszvyupudykrvvmwedikrro","sorbsssghuqanhbaeadihfenfz","xpfxjqibcrdlryqzasbaugrplm","ftqqeqgaecynmwyqrjuexpiyym","qusevqybwtwhfihodctmpxvpsw","jekrjafinseysrjyrjblxbrjkr","kriubpkzljnlghymdmlfcqvkfl","ynelgosfiujenvwsxozpogwmrt","bwovyaupolyunxdizxcvfgiezh","dtnxbsxayqdsrkmyxonxantril","mvuziguweifrcopnhpcrtcuwtz","emmvhnbzhnpogqeolyfdccfdxe","drbwfvjkjkhdmjtkbizodyluie","dnlqhhfxkssbapvllskvekmtcq","pwhxasjncajuqydghjcplakuxl","jlpkgahategcacybcrtftxiziq","tzsqfvyrklitvmvxuyrcqufvvz","llvedwbxmumqwktbyhdojrskid","mflduhadcdsxphellypuupfbjo","eswawcxpdwpoeyvcqxfzubipet","elkipbgwygrkvvkfqcdvlnener","uiubvbcqlxvmcwqwjvbhmwjmpc","bxgzxmsnexgkxmpxabvytmhnsg","rjgwbzxowqrzvpqgkiipaescos","clvozocvmnjrwofvvdswhhghta","oftwnsoxoklkbrekcxcrjdyywc","ypyndfvcvhpetolpotztybclpy","qeqqmxeusoraylprccsnaveqob","ftzqpxlcepcjxdmlgvwhjqarqc","lknkowqqniojgtgayvhvyjozeb","puregmukaplnuklmjgzamqxpqj","banhazwpcyzcoixvanulydhgxg","ualsqdmhjvvefxtskpeybngcps","ynnaealygpljdzzyjyomyrtjvc","cjcrjidyelnhmvuclduprioyls","ubmytsljvuahucisowrccobuds","fyxgsdrbhufncnyjywrvphgimd","aelyvvzpadkvasejnngfgbqdcm","wnqblvpiwsiuwzkkxqnqctbpus","sldozvuccovqppksxvrjtxhvit","blwlvuctjhyflwoaajonydhawf","yanxnkzuuoohugvwvsajsirnyy","ihrbchjartqzistxyufhikkdpi","vobmubnfxwqwsmrepvbakejcwq","tmuylmcheozhyvzbmjaksqzcyo","tgzuvgygvoddrxlgqrjdxititg","pgwijngnatnfbihmebudwtjler","dxrrliurddzesxfjnzpzipwbcx","hqiledlipgjttsuolgvewpenoh","pwmgljestkviesoennjabfeaua","xzkwilxuuskdewkpbhprenwbpk","kyfcxezexydiqrfigudsmalrjt","pplpapcmatogqmnjyiekpwpvra","phjwkhuzokavxhlwzatjlxxjdq","gsyctusmadnyospivhminahewx","lmdzpdgkiyxwqbymtiehjzyyzy","kfwscewqjsfcqerzffwjxmmwrh","urpjthfllddlezfcjknsaquvcm","lnpbcianhthqoxllhuhuzsotej","qcbhulhgowhkeukvxrkhnpznfv","nrzutnhymwbvimvhkmiiadtekc","cymovfxebyfbpctgdoxvxidfxd","tzlxsrjtqtvjjwleksukvgucfz","hazayipsqnzbfaktuvpocennad","bpsnljjapwjvochmnngbrvodxr","hogtixfirpsplzixcrorvigcfy","vpjxvpqtqmxpebotpuumxkjelf","gjafjpyikrvtqkrthzhcgsqrcq","eahepicxhbjbonywaxjrxlusjn","gowwxlxxunfwmzapdqgonvuhcr","bmciwhbxpcbrwgmscrbjtmsffv","wauarejjfnsooljglsygpyaxhi","dqdyxvdkmpzresxyuoeevunsux","rufegbpackdhaoexcgvucgqfnc","fgyqmlgjzhuygifcagnmwowykh","nenxavvhcxbcwwjxtvfuvlqdxl","pkxukiwvhtkdhrmdrnfxsmxszi","vtfcjikclyoqheslgiuhiohswd","qnzlnwlykjeaadsmzekhxdlhso","cswaxylbpvkvvqikxuhuytxtkq","fqjmtbizqhmsmkdlwnykdtusmv","dnndjrkxayykoxogxzqpoascsx","kudirlhijchbxgkmbjcawsnevk","ibredttvihgthxsssivbwkodni","ndfhyjujbdtgafauhclenwwauc","dlhltoeuettryyhahgfvsnqquc","zfovrjecgpvuwafzrcqrnvicwl","wvhtzzjslwrnntlsutdjgflsig","cwzprjhpoteypywhcqxybfaufy","kxpoqgfcxhtcutvicnwrwvbdht","yxuqrnebycfmmbpamcyrlceszk","qtfpzlcdylxjhnoovkprzzdtva","ofsufpklwvxryrdrjhrtpyntdy","jakbpbwgnyrbhmjtfqantjvgma","elcmiulwpdhgtywpkmuwvmugfb","bgkolrjiezcgbtisbadzsbblqy","bysrytewgztiucrvhdrydthsgi","sboobzfttoofahmhggcucroqdg","goijtkucwuxknglrkghfjlvviz","fbsjsfbdbculdfwqscatqffdlj","fovbbaxcponygdrkeikarmrrmu","llufcnbnmptjzqbycflcjyxnjs","wygrygxeliymlswyhfphtxkxza","dnnnwnplkcwkyqamxvuurrmraf","uxuouoqimlaauxwxhqbpkqldsp","gzoxvxqtzjntxpymongdvdmknz","byttjhdbzaiiosssioocvzrqdj","dtbgycbuevtufhhakgjrdbwqvq","iuvzjneslguzdsxjwbvjzxsrmv","sjgstthhqmxltbhokfojcvcavg","qouxkideeitotrmtlkkqbuxspo","gemdupryxphaoxcpobxcvbwwnr","ipwmzvkhoiomobunnifmgorwwm","pvclnzfftblusgyvwgnjfudyrv","hgcqqvwvdxudnhwbarzvnpregc","yegdjsadsklogryoibczqjquck","qqqmlzcpcwcbilnmabkbtqpxsz","ugaonbmhxyqczpcixqarkkfaoc","jduniiuwlqfothrmggvkthljdf","nowqnfexqyhkcdfbquibnskvzv","niqqufjmhieqwuufjntescbpth","xssruebqucbqirkzfsrwsrnard","vykwiekcgcualeubejlglpioyr","zqonaittpznvzaegctezvgrjak","oeeavxiwrfdroahrdzoqfhhokg","wkzeqstrweworaqypfrmznagew","miwoyjugqupmfspaarganpcztq","uqdhjmtdsmqqbakihtcdjxluht","rerhgugynhhapxvzcsosqhmhjb","aqtouyhjvicvlclouebjeaxsyx","bzqsbjniotkfvmpytspzprflmj","nvyflhjvasxfbkzhkraustdtfd","rrpnswfbjacrbuaommysxwhyjk","nbyjogzyrebeorlxmgshudnpjj","wxhsapvmnfteueguylfrgiugbf","qdnplnfptdszrgieohvxirwsko","anbeydgmminizphatxitsigmvf","tdfgzwzqlrmssxtwowqqkfclxc"];
    let result = solution::Solution::find_substring(s, words.iter().map(|s| s.to_string()).collect());
    // Measure end time
    let end_time: std::time::Instant = std::time::Instant::now();

    println!("result = {:?}", result);
    println!("elapsed time = {:?}", end_time.duration_since(start_time));
}
