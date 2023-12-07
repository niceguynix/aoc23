use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = std::io::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        let card = match s {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::N9,
            '8' => Self::N8,
            '7' => Self::N7,
            '6' => Self::N6,
            '5' => Self::N5,
            '4' => Self::N4,
            '3' => Self::N3,
            '2' => Self::N2,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("No card type of {} exists", s),
                ))
            }
        };

        Ok(card)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct PlayerCard {
    cards: [Card; 5],
    bid: u32,
}

impl Ord for PlayerCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut ord = Ordering::Equal;
        for i in 0..5 {
            if let Ordering::Equal = ord {
                ord = self.cards.get(i).unwrap().cmp(other.cards.get(i).unwrap());
            }
        }

        // println!("Comparison {:?} {:?} {:?}",self,other,ord);
        ord
    }
}

impl FromStr for PlayerCard {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, bid) = s.split_once(" ").unwrap();
        let mut chars = card.chars();
        let c1 = chars.next().unwrap().try_into()?;
        let c2 = chars.next().unwrap().try_into()?;
        let c3 = chars.next().unwrap().try_into()?;
        let c4 = chars.next().unwrap().try_into()?;
        let c5 = chars.next().unwrap().try_into()?;

        let bid = bid.parse().unwrap();
        let cards = [c1, c2, c3, c4, c5];

        Ok(PlayerCard { cards, bid })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum Hand {
    HighCard(PlayerCard),
    OnePair(PlayerCard),
    TwoPair(PlayerCard),
    ThreeOfKind(PlayerCard),
    FullHouse(PlayerCard),
    FourOfKind(PlayerCard),
    FiveOfKind(PlayerCard),
}

use Hand::*;

fn get_data(h: &Hand) -> &PlayerCard {
    match h {
        FiveOfKind(x) | FourOfKind(x) | HighCard(x) | FullHouse(x) | OnePair(x) | TwoPair(x)
        | ThreeOfKind(x) => x,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut ord = self.partial_cmp(other).unwrap();

        if let Ordering::Equal = ord {
            let p1c = get_data(self);
            let p2c = get_data(other);
            ord = p1c.cards.cmp(&p2c.cards);
        }

        ord
    }
}

impl From<PlayerCard> for Hand {
    fn from(value: PlayerCard) -> Self {
        let mut hash = HashMap::new();

        for i in &value.cards {
            if let Some(c) = hash.get(i) {
                hash.entry(i).and_modify(|x| *x = *x + 1);
            } else {
                hash.insert(i, 1);
            }
        }

        let m = hash.values().max().unwrap();
        if *m == 5 {
            return Self::FiveOfKind(value);
        }
        if *m == 4 {
            return Self::FourOfKind(value);
        }
        if *m == 3 {
            let rem_dis = hash.values().filter(|x| **x != 3).count();
            if rem_dis == 1 {
                return Self::FullHouse(value);
            } else {
                return Self::ThreeOfKind(value);
            }
        }

        let no_not_two = hash.values().filter(|x| **x != 2).count();
        if *m == 2 {
            if no_not_two == 1 {
                return Self::TwoPair(value);
            } else {
                return Self::OnePair(value);
            }
        } else {
            return Self::HighCard(value);
        }
    }
}

fn part1(input: &'static str) -> u32 {
    let mut playercards = input
        .lines()
        .map(|x| x.parse::<PlayerCard>().unwrap())
        .collect::<Vec<_>>();

    // playercards.sort();
    // println!("{playercards:?}");
    let mut hands = playercards
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.cmp(b));

    let t1 = 0;
    let t2 = 1;
    // let test=hands[t1].cmp(&hands[t2]);

    // println!("In main function {:?} {:?} {:?}",hands[t1],hands[t2],test);

    hands
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx as u32 + 1) * get_data(h).bid)
        .sum::<u32>()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day7/input.txt");
    let before = std::time::Instant::now();
    let result = match part.as_str() {
        "part1" => part1(input),
        // "part2" => part2(input),
        _ => panic!("Specify one of 2 parts {part}"),
    };

    println!("The result is {result} duration:{:?}", before.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1() {
        assert_eq!(part1(include_str!("../inputs/day7/part1_sample.txt")), 6440)
    }

    // #[test]
    // fn day7_part2() {
    //     assert_eq!(
    //         part2(include_str!("../inputs/day7/part1_sample.txt")),
    //         71503
    //     )
    // }
}
