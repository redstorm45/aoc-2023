
use std::env;
use std::fs;
use std::cmp::Ordering;


#[derive(Copy, Clone, Eq, PartialEq)]
enum HandType {
    FiveKind = 1,
    FourKind = 2,
    FullHouse = 3,
    ThreeKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
}

#[derive(Eq, Copy, Clone)]
struct Hand<'a> {
    htype : HandType,
    score : i32,
    source : &'a str,
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.htype as isize).cmp(&(other.htype as isize))
            .then( self.score.cmp(&other.score) )
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.htype == other.htype && self.score == other.score
    }
}

fn card_score(c : &char, use_jokers: bool) -> i32 {
    if !use_jokers {
        return vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
        ].iter().position(|x| x==c).unwrap().try_into().unwrap();
    } else {
        return vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
        ].iter().position(|x| x==c).unwrap().try_into().unwrap();
    }
}

fn to_counted(s : &str, use_jokers : bool) -> (Vec<(char,i32)>, i32) {
    let mut res: Vec<(char,i32)> = vec![];
    let mut joker_count : i32 = 0;

    for c in s.chars() {
        if use_jokers && c == 'J' {
            joker_count += 1;
        } else {
            let mut added = false;
            for (rchar, rcount) in res.iter_mut() {
                if *rchar == c {
                    *rcount += 1;
                    added = true;
                }
            }
            if !added {
                res.push( (c,1) );
            }
        }
    }

    res.sort_by(|&a,&b| a.1.cmp(&b.1).reverse().then( card_score(&a.0, use_jokers).cmp( &card_score(&b.0, use_jokers) ) ) );

    return (res, joker_count);
}

fn build_hand(hand_str : &str, use_jokers : bool) -> Hand {
    let (cards, joker_count) = to_counted(hand_str, use_jokers);

    /*
    // scoring like poker (not used)
    let mut score = 0;
    for (face, _) in cards.iter() {
        score = 20*score + card_score(&face);
    }
    */
    let mut score = 0;
    for face in hand_str.chars() {
        score = 20*score + card_score(&face, use_jokers);
    }

    if joker_count == 5 {
        return Hand {
            htype: HandType::FiveKind,
            score: score,
            source: hand_str
        }
    }

    let first_count = cards.get(0).unwrap().1;

    if first_count+joker_count == 5 {
        return Hand {
            htype: HandType::FiveKind,
            score: score,
            source: hand_str
        }
    }
    if first_count+joker_count == 4 {
        return Hand {
            htype: HandType::FourKind,
            score: score,
            source: hand_str
        }
    }

    let sec_count = cards.get(1).unwrap().1;
    if first_count+joker_count == 3 && sec_count == 2{
        return Hand {
            htype: HandType::FullHouse,
            score: score,
            source: hand_str
        }
    }
    if first_count+joker_count == 3 && sec_count == 1{
        return Hand {
            htype: HandType::ThreeKind,
            score: score,
            source: hand_str
        }
    }
    if first_count == 2 && sec_count+joker_count == 2{
        return Hand {
            htype: HandType::TwoPair,
            score: score,
            source: hand_str
        }
    }
    if first_count+joker_count == 2 && sec_count == 1{
        return Hand {
            htype: HandType::OnePair,
            score: score,
            source: hand_str
        }
    }

    return Hand {
        htype: HandType::HighCard,
        score: score,
        source: hand_str
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("No filename");

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut all_bids1 : Vec<(Hand,i32)> = vec![];
    let mut all_bids2 : Vec<(Hand,i32)> = vec![];

    for line in contents.split('\n') {
        if line.len() < 2 {
            continue;
        }

        let mut it = line.split(' ');

        let hand_str = it.next().unwrap();
        let bid = it.next().unwrap().parse::<i32>().expect("Bid not a number");

        all_bids1.push( (build_hand(hand_str, false),bid) );
        all_bids2.push( (build_hand(hand_str, true),bid) );
    }

    all_bids1.sort_by(|&a,&b| a.0.cmp(&b.0).reverse());
    all_bids2.sort_by(|&a,&b| a.0.cmp(&b.0).reverse());

    let mut res1 = 0;
    for (i, (_hand,bid)) in all_bids1.iter().enumerate() {
        res1 += (i as i32 +1) * bid;
    }
    let mut res2 = 0;
    for (i, (hand,bid)) in all_bids2.iter().enumerate() {
        println!("Ranked {} is hand {} with bid {}", i+1, hand.source, bid);
        res2 += (i as i32 +1) * bid;
    }

    println!("Total score part 1: {res1}");
    println!("Total score part 2: {res2}");
}
