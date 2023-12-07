use std::cmp::Ordering;
use std::collections::HashMap;
use std::{env, fs};

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Type {
    // Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind = 7,
    // Four of a kind, where four cards have the same label and one card has a
    // different label: AA8AA
    FourOfAKind = 6,
    // Full house, where three cards have the same label, and the remaining two
    // cards share a different label: 23332
    FullHouse = 5,
    // Three of a kind, where three cards have the same label, and the remaining
    // two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind = 4,
    // Two pair, where two cards share one label, two other cards share a second
    // label, and the remaining card has a third label: 23432
    TwoPair = 3,
    // One pair, where two cards share one label, and the other three cards have a
    // different label from the pair and each other: A23A4
    OnePair = 2,
    // High card, where all cards' labels are distinct: 23456
    HighCard = 1,
    Nothing = 0,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: u32,
}

fn main() -> Result<(), BoxError> {
    let input_path = env::args_os()
        .skip(1)
        .next()
        .ok_or("missing input file path")?;
    let input = fs::read_to_string(input_path)?;

    let mut hands = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (hand, bid) = line.split_once(' ').ok_or("unable to split line")?;
        hands.push(Hand {
            hand: hand.to_string(),
            bid: bid.parse()?,
        });
    }

    let mut ranked = hands
        .iter()
        .map(|hand| (hand.hand_type(), hand))
        .collect::<Vec<_>>();
    ranked.sort_by(|(a_ty, a_hand), (b_ty, b_hand)| match a_ty.cmp(b_ty) {
        Ordering::Equal => {
            // println!("tie break {} {}", a_hand.hand, b_hand.hand);
            for (ach, bch) in a_hand.hand.chars().zip(b_hand.hand.chars()) {
                let ord = cmp_card(ach, bch);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            // All chars the same
            return Ordering::Equal;
        }
        ord => ord,
    });
    let mut total_winnings = 0;
    for (i, (_ty, hand)) in ranked.iter().enumerate() {
        // println!("{} * {} = {}", (i as u32 + 1), hand.bid, (i as u32 + 1) * hand.bid);
        total_winnings += (i as u32 + 1) * hand.bid;
    }

    // dbg!(&ranked);
    println!("Part 1: {total_winnings}");

    Ok(())
}

impl Hand {
    fn hand_type(&self) -> Type {
        let mut map = HashMap::with_capacity(5);
        for c in self.hand.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut counts = map.values().collect::<Vec<_>>();
        counts.sort_unstable();
        counts.reverse();
        match counts[..] {
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ if *counts[0] == 2 => Type::OnePair,
            _ => Type::Nothing,
        }
    }
}

fn cmp_card(a: char, b: char) -> Ordering {
    let sort = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let ai = sort.iter().position(|&c| a == c).expect("invalid char");
    let bi = sort.iter().position(|&c| b == c).expect("invalid char");
    bi.cmp(&ai)
}
