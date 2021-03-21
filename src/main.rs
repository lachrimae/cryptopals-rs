mod cryptopals;

#[macro_use]
extern crate clap;

use std::collections::HashMap;
use std::fs;

pub use cryptopals::hex;
pub use cryptopals::b64;
pub use cryptopals::bytewise;
pub use cryptopals::frequency;
pub use cryptopals::vigenere;
pub use cryptopals::aes;
pub use cryptopals::padding;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Curran McConnell <curran.mcconnell@protonmail.com>")
        (about: "test the cryptopals lib I wrote")
        (@arg PROBLEMSET: -s --set +required +takes_value "the problemset to test")
    ).get_matches();

    match matches.value_of("PROBLEMSET") {
        Some("1") => set_one(),
        Some("2") => set_two(),
        Some("3") => set_three(None),
        Some("4") => set_four(),
        Some("5") => set_five(),
        Some("6") => set_six(),
        Some("7") => set_seven(),
        Some("8") => set_eight(),
        Some("9") => set_nine(),
        Some("10") => set_ten(),
        Some("11") => set_eleven(),
        _ => {
            set_one();
            set_two();
            set_three(None);
            set_four();
            set_five();
            set_six();
            set_seven();
            set_eight();
            set_nine();
            set_ten();
            set_eleven();
        }
    }
}

fn set_one() {
    assert_eq!(b64::to_b64(&hex::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

fn set_two() {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";
    assert_eq!(hex::to_hex(&bytewise::xor(&hex::from_hex(s1), &hex::from_hex(s2))), "746865206b696420646f6e277420706c6179")
}

fn set_three(ciphertext:Option<String>) {
    let mut dists = HashMap::new();
    let mut default_run = false;
    if ciphertext == None {
        default_run = true;
    }
    let x = match ciphertext {
        Some(x) => hex::from_hex(&x),
        None => hex::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
    };
    let l = x.len();
    for c in 0..=255 {
        let mut cs = Vec::with_capacity(l);
        for _ in 0..l {
            cs.push(c);
        }
        let string: String = bytewise::xor(&x, &cs).iter().map(|u| *u as char).collect();
        let dist = frequency::eng_score(string, false);
        dists.insert(c, dist);
    }
    let mut letters = (0..=255).collect::<Vec<u8>>();
    letters.sort_by(
        |a, b| dists.get(a).unwrap().partial_cmp(&dists.get(b).unwrap()).unwrap()
    );
    let mut is_first = true;
    for c in letters.iter().take(5) {
        let mut cs = Vec::with_capacity(l);
        for _ in 0..x.len() {
            cs.push(*c as u8);
        }
        let p_text = bytewise::xor(&x, &cs).iter().map(|c| *c as char).collect::<String>();
        if is_first && default_run {
            assert_eq!("Cooking MC's like a pound of bacon", p_text);
            is_first = false;
        }
        println!("dist: {}, {}", dists.entry(*c).or_insert(1.0), p_text)
    }
    let mut cs = Vec::with_capacity(l);
    for _ in 0..l {
        cs.push(letters[0]);
    }
}

fn set_four() {
    let contents: Vec<Vec<u8>> = fs::read_to_string("./data/1-4.txt")
        .expect("something went wrong reading 1-4.txt")
        .split("\n")
        .map(hex::from_hex)
        .collect();
    let letters: Vec<u8> = (0..255_u8).collect();
    let mut lowest_dist = 1.0;
    let mut lowest_index = 0;
    let mut lowest_char = ' ';
    for (i, s) in contents.iter().enumerate() {
        for c in letters.iter() {
            let mut cs = Vec::with_capacity(s.len());
            for _ in 0..s.len() {
                cs.push(*c);
            }
            let xord = bytewise::xor(&s, &cs);
            let dist = frequency::eng_score(xord.iter().map(|u| *u as char).collect(), false);
            if dist < lowest_dist {
                lowest_dist = dist;
                lowest_index = i;
                lowest_char = *c as char;
            }
        }
    }
    let best_cipher = &contents[lowest_index as usize];
    let mut best_key = Vec::with_capacity(best_cipher.len());
    for _ in 0..best_cipher.len() {
        best_key.push(lowest_char as u8);
    }
    let best_plain = bytewise::xor(&best_key, &best_cipher)
        .iter()
        .map(|c| *c as char)
        .collect::<String>();
    println!("best plain: {}", best_plain);
    assert_eq!("Now that the party is jumping\n", best_plain);
}

fn set_five() {
    let stanza = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let key = "ICE"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let cipher = bytewise::xor_rep(&stanza, &key);
    println!("The cipher: {}", hex::to_hex(&cipher));
    assert_eq!(cipher, hex::from_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));
}

fn set_six() {
    let t1 = "this is a test".chars().map(|c| c as u8).collect::<Vec<u8>>();
    let t2 = "wokka wokka!!!".chars().map(|c| c as u8).collect::<Vec<u8>>();
    assert_eq!(bytewise::hamm_dist(&t1, &t2), 37);
    assert_eq!(b64::from_b64(b64::to_b64(&t1)), t1);
    assert_eq!(b64::to_b64(&b64::from_b64(b64::to_b64(&t1))), b64::to_b64(&t1));

    let ciphertext = get_linewrapped_b64("./data/1-6.txt");
    let blocks = vigenere::break_vigenere(&ciphertext);
    assert_eq!(bytewise::to_ascii(&blocks)[..10], "I'm back and I'm ringin' the bell"[..10]);
    println!("{}", bytewise::to_ascii(&blocks));
}

fn get_linewrapped_b64(filename:&str) -> Vec<u8> {
    b64::from_b64(
        fs::read_to_string(filename)
        .expect("file not found")
        .chars()
        .filter(|c| *c != '\n')
        .collect())
}

fn get_many_hex(filename:&str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("file not found")
        .split("\n")
        .map(hex::from_hex)
        .collect()
}

fn set_seven() {
    let key = bytewise::from_ascii(&String::from("YELLOW SUBMARINE"));
    let cipher_t = get_linewrapped_b64("./data/1-7.txt");
    let plain_t = aes::decrypt_ecb(&cipher_t, &key);
    assert_eq!(bytewise::to_ascii(&plain_t)[..33], String::from("I'm back and I'm ringin' the bell"));
}

fn set_eight() {
    let cipher_ts = get_many_hex("./data/1-8.txt");
    let mut num_ecbs = 0;
    let mut ecb_index = 0;
    for (i, cipher_t) in cipher_ts.iter().enumerate() {
        let blocks = bytewise::make_blocks(&cipher_t, 16);
        let mut is_ecb = false;
        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                if blocks[i] == blocks[j] {
                    is_ecb = true;
                }
            }
        }
        if is_ecb {
            num_ecbs += 1;
            ecb_index = i;
        }
    }
    if num_ecbs == 1 {
        println!("The ecb ciphertext index is {:#?}", ecb_index);
    } else {
        println!("Didn't find the (unambiguous) ecb-encoded ciphertext.");
    }
}

fn set_nine() {
    let mut manual = bytewise::from_ascii(&String::from("YELLOW SUBMARINE"));
    manual.push(4); manual.push(4); manual.push(4); manual.push(4);
    let mut via_library = bytewise::from_ascii(&String::from("YELLOW SUBMARINE"));
    padding::pkcs7(&mut via_library, 20);
    assert_eq!(via_library, manual);

    manual.pop(); manual.pop(); manual.pop(); manual.pop();
    padding::depkcs7(&mut via_library);
    assert_eq!(via_library, manual);
}

fn set_ten() {
    let key = bytewise::from_ascii(&String::from("YELLOW SUBMARINE"));
    let iv = bytewise::make_null_vec(16);

    // testing that we can encrypt as well as decrypt
    let test_plain_t = bytewise::from_ascii(&String::from("Hallelujah"));
    let test_cipher_t = aes::encrypt_ecb(&test_plain_t, &key);
    let test_out_t = aes::decrypt_ecb(&test_cipher_t, &key);
    assert_eq!(test_plain_t, test_out_t);
    let test_cipher_t = aes::encrypt_cbc(&test_plain_t, &key, &iv);
    let test_out_t = aes::decrypt_cbc(&test_cipher_t, &key, &iv);
    assert_eq!(test_plain_t, test_out_t);

    let cipher_t = &get_linewrapped_b64("./data/2-10.txt");
    assert_eq!(" ringin"[..7], bytewise::to_ascii(&aes::decrypt_cbc(&cipher_t, &key, &iv))[..7]);
}

fn set_eleven() {
    let plain_t = bytewise::from_ascii(&String::from(r#" Kant was born into an artisan family of modest means. His father was a master harness maker, and his mother was the daughter of a harness maker, though she was better educated than most women of her social class. Kant’s family was never destitute, but his father’s trade was in decline during Kant’s youth and his parents at times had to rely on extended family for financial support.

Kant’s parents were Pietist and he attended a Pietist school, the Collegium Fridericianum, from ages eight through fifteen. Pietism was an evangelical Lutheran movement that emphasized conversion, reliance on divine grace, the experience of religious emotions, and personal devotion involving regular Bible study, prayer, and introspection. Kant reacted strongly against the forced soul-searching to which he was subjected at the Collegium Fridericianum, in response to which he sought refuge in the Latin classics, which were central to the school’s curriculum. Later the mature Kant’s emphasis on reason and autonomy, rather than emotion and dependence on either authority or grace, may in part reflect his youthful reaction against Pietism. But although the young Kant loathed his Pietist schooling, he had deep respect and admiration for his parents, especially his mother, whose “genuine religiosity” he described as “not at all enthusiastic.” According to his biographer, Manfred Kuehn, Kant’s parents probably influenced him much less through their Pietism than through their artisan values of “hard work, honesty, cleanliness, and independence,” which they taught him by example.[2]

Kant attended college at the University of Königsberg, known as the Albertina, where his early interest in classics was quickly superseded by philosophy, which all first year students studied and which encompassed mathematics and physics as well as logic, metaphysics, ethics, and natural law. Kant’s philosophy professors exposed him to the approach of Christian Wolff (1679–1750), whose critical synthesis of the philosophy of G. W. Leibniz (1646–1716) was then very influential in German universities. But Kant was also exposed to a range of German and British critics of Wolff, and there were strong doses of Aristotelianism and Pietism represented in the philosophy faculty as well. Kant’s favorite teacher was Martin Knutzen (1713–1751), a Pietist who was heavily influenced by both Wolff and the English philosopher John Locke (1632–1704). Knutzen introduced Kant to the work of Isaac Newton (1642–1727), and his influence is visible in Kant’s first published work, Thoughts on the True Estimation of Living Forces (1747), which was a critical attempt to mediate a dispute in natural philosophy between Leibnizians and Newtonians over the proper measurement of force.

After college Kant spent six years as a private tutor to young children outside Königsberg. By this time both of his parents had died and Kant’s finances were not yet secure enough for him to pursue an academic career. He finally returned to Königsberg in 1754 and began teaching at the Albertina the following year. For the next four decades Kant taught philosophy there, until his retirement from teaching in 1796 at the age of seventy-two.

Kant had a burst of publishing activity in the years after he returned from working as a private tutor. In 1754 and 1755 he published three scientific works – one of which, Universal Natural History and Theory of the Heavens (1755), was a major book in which, among other things, he developed what later became known as the nebular hypothesis about the formation of the solar system. Unfortunately, the printer went bankrupt and the book had little immediate impact. To secure qualifications for teaching at the university, Kant also wrote two Latin dissertations: the first, entitled Concise Outline of Some Reflections on Fire (1755), earned him the Magister degree; and the second, New Elucidation of the First Principles of Metaphysical Cognition (1755), entitled him to teach as an unsalaried lecturer. The following year he published another Latin work, The Employment in Natural Philosophy of Metaphysics Combined with Geometry, of Which Sample I Contains the Physical Monadology (1756), in hopes of succeeding Knutzen as associate professor of logic and metaphysics, though Kant failed to secure this position. Both the New Elucidation, which was Kant’s first work concerned mainly with metaphysics, and the Physical Monadology further develop the position on the interaction of finite substances that he first outlined in Living Forces. Both works depart from Leibniz-Wolffian views, though not radically. The New Elucidation in particular shows the influence of Christian August Crusius (1715–1775), a German critic of Wolff.[3]

As an unsalaried lecturer at the Albertina Kant was paid directly by the students who attended his lectures, so he needed to teach an enormous amount and to attract many students in order to earn a living. Kant held this position from 1755 to 1770, during which period he would lecture an average of twenty hours per week on logic, metaphysics, and ethics, as well as mathematics, physics, and physical geography. In his lectures Kant used textbooks by Wolffian authors such as Alexander Gottlieb Baumgarten (1714–1762) and Georg Friedrich Meier (1718–1777), but he followed them loosely and used them to structure his own reflections, which drew on a wide range of ideas of contemporary interest. These ideas often stemmed from British sentimentalist philosophers such as David Hume (1711–1776) and Francis Hutcheson (1694–1747), some of whose texts were translated into German in the mid-1750s; and from the Swiss philosopher Jean-Jacques Rousseau (1712–1778), who published a flurry of works in the early 1760s. From early in his career Kant was a popular and successful lecturer. He also quickly developed a local reputation as a promising young intellectual and cut a dashing figure in Königsberg society.

After several years of relative quiet, Kant unleashed another burst of publications in 1762–1764, including five philosophical works. The False Subtlety of the Four Syllogistic Figures (1762) rehearses criticisms of Aristotelian logic that were developed by other German philosophers. The Only Possible Argument in Support of a Demonstration of the Existence of God (1762–3) is a major book in which Kant drew on his earlier work in Universal History and New Elucidation to develop an original argument for God’s existence as a condition of the internal possibility of all things, while criticizing other arguments for God’s existence. The book attracted several positive and some negative reviews. In 1762 Kant also submitted an essay entitled Inquiry Concerning the Distinctness of the Principles of Natural Theology and Morality to a prize competition by the Prussian Royal Academy, though Kant’s submission took second prize to Moses Mendelssohn’s winning essay (and was published with it in 1764). Kant’s Prize Essay, as it is known, departs more significantly from Leibniz-Wolffian views than his earlier work and also contains his first extended discussion of moral philosophy in print. The Prize Essay draws on British sources to criticize German rationalism in two respects: first, drawing on Newton, Kant distinguishes between the methods of mathematics and philosophy; and second, drawing on Hutcheson, he claims that “an unanalysable feeling of the good” supplies the material content of our moral obligations, which cannot be demonstrated in a purely intellectual way from the formal principle of perfection alone (2:299).[4] These themes reappear in the Attempt to Introduce the Concept of Negative Magnitudes into Philosophy (1763), whose main thesis, however, is that the real opposition of conflicting forces, as in causal relations, is not reducible to the logical relation of contradiction, as Leibnizians held. In Negative Magnitudes Kant also argues that the morality of an action is a function of the internal forces that motivate one to act, rather than of the external (physical) actions or their consequences. Finally, Observations on the Feeling of the Beautiful and the Sublime (1764) deals mainly with alleged differences in the tastes of men and women and of people from different cultures. After it was published, Kant filled his own interleaved copy of this book with (often unrelated) handwritten remarks, many of which reflect the deep influence of Rousseau on his thinking about moral philosophy in the mid-1760s.

These works helped to secure Kant a broader reputation in Germany, but for the most part they were not strikingly original. Like other German philosophers at the time, Kant’s early works are generally concerned with using insights from British empiricist authors to reform or broaden the German rationalist tradition without radically undermining its foundations. While some of his early works tend to emphasize rationalist ideas, others have a more empiricist emphasis. During this time Kant was striving to work out an independent position, but before the 1770s his views remained fluid.

In 1766 Kant published his first work concerned with the possibility of metaphysics, which later became a central topic of his mature philosophy. Dreams of a Spirit-Seer Elucidated by Dreams of Metaphysics, which he wrote soon after publishing a short Essay on Maladies of the Mind (1764), was occasioned by Kant’s fascination with the Swedish visionary Emanuel Swedenborg (1688–1772), who claimed to have insight into a spirit world that enabled him to make a series of apparently miraculous predictions. In this curious work Kant satirically compares Swedenborg’s spirit-visions to the belief of rationalist metaphysicians in an immaterial soul that survives death, and he concludes that philosophical knowledge of either is impossible because human reason is limited to experience. The skeptical tone of Dreams is tempered, however, by Kant’s suggestion that “moral faith” nevertheless supports belief in an immaterial and immortal soul, even if it is not possible to attain metaphysical knowledge in this domain (2:373).

In 1770, at the age of forty-six, Kant was appointed to the chair in logic and metaphysics at the Albertina, after teaching for fifteen years as an unsalaried lecturer and working since 1766 as a sublibrarian to supplement his income. Kant was turned down for the same position in 1758. But later, as his reputation grew, he declined chairs in philosophy at Erlangen (1769) and Jena (1770) in hopes of obtaining one in Königsberg. After Kant was finally promoted, he gradually extended his repertoire of lectures to include anthropology (Kant’s was the first such course in Germany and became very popular), rational theology, pedagogy, natural right, and even mineralogy and military fortifications. In order to inaugurate his new position, Kant also wrote one more Latin dissertation: Concerning the Form and Principles of the Sensible and Intelligible World (1770), which is known as the Inaugural Dissertation.

The Inaugural Dissertation departs more radically from both Wolffian rationalism and British sentimentalism than Kant’s earlier work. Inspired by Crusius and the Swiss natural philosopher Johann Heinrich Lambert (1728–1777), Kant distinguishes between two fundamental powers of cognition, sensibility and understanding (intelligence), where the Leibniz-Wolffians regarded understanding (intellect) as the only fundamental power. Kant therefore rejects the rationalist view that sensibility is only a confused species of intellectual cognition, and he replaces this with his own view that sensibility is distinct from understanding and brings to perception its own subjective forms of space and time – a view that developed out of Kant’s earlier criticism of Leibniz’s relational view of space in Concerning the Ultimate Ground of the Differentiation of Directions in Space (1768). Moreover, as the title of the Inaugural Dissertation indicates, Kant argues that sensibility and understanding are directed at two different worlds: sensibility gives us access to the sensible world, while understanding enables us to grasp a distinct intelligible world. These two worlds are related in that what the understanding grasps in the intelligible world is the “paradigm” of “NOUMENAL PERFECTION,” which is “a common measure for all other things in so far as they are realities.” Considered theoretically, this intelligible paradigm of perfection is God; considered practically, it is “MORAL PERFECTION” (2:396). The Inaugural Dissertation thus develops a form of Platonism; and it rejects the view of British sentimentalists that moral judgments are based on feelings of pleasure or pain, since Kant now holds that moral judgments are based on pure understanding alone.

After 1770 Kant never surrendered the views that sensibility and understanding are distinct powers of cognition, that space and time are subjective forms of human sensibility, and that moral judgments are based on pure understanding (or reason) alone. But his embrace of Platonism in the Inaugural Dissertation was short-lived. He soon denied that our understanding is capable of insight into an intelligible world, which cleared the path toward his mature position in the Critique of Pure Reason (1781), according to which the understanding (like sensibility) supplies forms that structure our experience of the sensible world, to which human knowledge is limited, while the intelligible (or noumenal) world is strictly unknowable to us. Kant spent a decade working on the Critique of Pure Reason and published nothing else of significance between 1770 and 1781. But its publication marked the beginning of another burst of activity that produced Kant’s most important and enduring works. Because early reviews of the Critique of Pure Reason were few and (in Kant’s judgment) uncomprehending, he tried to clarify its main points in the much shorter Prolegomena to Any Future Metaphysics That Will Be Able to Come Forward as a Science (1783). Among the major books that rapidly followed are the Groundwork of the Metaphysics of Morals (1785), Kant’s main work on the fundamental principle of morality; the Metaphysical Foundations of Natural Science (1786), his main work on natural philosophy in what scholars call his critical period (1781–1798); the second and substantially revised edition of the Critique of Pure Reason (1787); the Critique of Practical Reason (1788), a fuller discussion of topics in moral philosophy that builds on (and in some ways revises) the Groundwork; and the Critique of the Power of Judgment (1790), which deals with aesthetics and teleology. Kant also published a number of important essays in this period, including Idea for a Universal History With a Cosmopolitan Aim (1784) and Conjectural Beginning of Human History (1786), his main contributions to the philosophy of history; An Answer to the Question: What is Enlightenment? (1784), which broaches some of the key ideas of his later political essays; and What Does it Mean to Orient Oneself in Thinking? (1786), Kant’s intervention in the pantheism controversy that raged in German intellectual circles after F. H. Jacobi (1743–1819) accused the recently deceased G. E. Lessing (1729–1781) of Spinozism.

With these works Kant secured international fame and came to dominate German philosophy in the late 1780s. But in 1790 he announced that the Critique of the Power of Judgment brought his critical enterprise to an end (5:170). By then K. L. Reinhold (1758–1823), whose Letters on the Kantian Philosophy (1786) popularized Kant’s moral and religious ideas, had been installed (in 1787) in a chair devoted to Kantian philosophy at Jena, which was more centrally located than Königsberg and rapidly developing into the focal point of the next phase in German intellectual history. Reinhold soon began to criticize and move away from Kant’s views. In 1794 his chair at Jena passed to J. G. Fichte, who had visited the master in Königsberg and whose first book, Attempt at a Critique of All Revelation (1792), was published anonymously and initially mistaken for a work by Kant himself. This catapulted Fichte to fame, but soon he too moved away from Kant and developed an original position quite at odds with Kant’s, which Kant finally repudiated publicly in 1799 (12:370–371). Yet while German philosophy moved on to assess and respond to Kant’s legacy, Kant himself continued publishing important works in the 1790s. Among these are Religion Within the Boundaries of Mere Reason (1793), which drew a censure from the Prussian King when Kant published the book after its second essay was rejected by the censor; The Conflict of the Faculties (1798), a collection of essays inspired by Kant’s troubles with the censor and dealing with the relationship between the philosophical and theological faculties of the university; On the Common Saying: That May be Correct in Theory, But it is of No Use in Practice (1793), Toward Perpetual Peace (1795), and the Doctrine of Right, the first part of The Metaphysics of Morals (1797), Kant’s main works in political philosophy; the Doctrine of Virtue, the second part of The Metaphysics of Morals (1797), Kant’s most mature work in moral philosophy, which he had been planning for more than thirty years; and Anthropology From a Pragmatic Point of View (1798), based on Kant’s anthropology lectures. Several other compilations of Kant’s lecture notes from other courses were published later, but these were not prepared by Kant himself.

Kant retired from teaching in 1796. For nearly two decades he had lived a highly disciplined life focused primarily on completing his philosophical system, which began to take definite shape in his mind only in middle age. After retiring he came to believe that there was a gap in this system separating the metaphysical foundations of natural science from physics itself, and he set out to close this gap in a series of notes that postulate the existence of an ether or caloric matter. These notes, known as the Opus Postumum, remained unfinished and unpublished in Kant’s lifetime, and scholars disagree on their significance and relation to his earlier work. It is clear, however, that some of these late notes show unmistakable signs of Kant’s mental decline, which became tragically precipitous around 1800. Kant died February 12, 1804, just short of his eightieth birthday."#));
    let guesses = 100;
    let mut guesses_right = 0.0;
    for _ in 0..guesses {
        let (cipher_t, true_mode) = aes::encryption_oracle(&plain_t);
        let guess_mode = if bytewise::has_duplicates(&bytewise::make_blocks(&cipher_t, 16)) {
            "ecb"
        } else {
            "cbc"
        };
        if guess_mode == true_mode {
            guesses_right += 1.0;
        }
    }
    println!("Guessed {:.1}% of attempts correctly", (guesses_right / guesses as f64) * 100.0);
}

pub fn set_twelve() {
}
