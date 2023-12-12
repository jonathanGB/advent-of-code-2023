use std::cmp::Ordering;

use crate::solver::Solver;

const HAND_LENGTH: usize = 5;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    #[default]
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    JackAsJoker,
}

impl Card {
    fn into(value: char, jack_as_joker: bool) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' if jack_as_joker => Card::JackAsJoker,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; HAND_LENGTH],
    hand_type: HandType,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl Hand {
    fn find_hand_type(mut cards: [Card; HAND_LENGTH]) -> HandType {
        cards.sort();

        let mut has_quint = false;
        let mut has_quad = false;
        let mut has_triple = false;
        let mut num_doubles = 0;

        let mut prev_card = cards[0];
        let mut num_prev_same = 1;
        let mut num_jokers = 0;
        for i in 1..HAND_LENGTH {
            let curr_card = cards[i];

            if curr_card == prev_card {
                num_prev_same += 1;
            }

            if i == HAND_LENGTH - 1 || curr_card != prev_card {
                match num_prev_same {
                    2 => num_doubles += 1,
                    3 => has_triple = true,
                    4 => has_quad = true,
                    5 => has_quint = true,
                    _ => {}
                }

                prev_card = curr_card;
                num_prev_same = 1;
            }

            if prev_card == Card::JackAsJoker {
                num_jokers = HAND_LENGTH - i;
                break;
            }
        }

        for _ in 0..num_jokers {
            if has_quint {
                unreachable!();
            } else if has_quad {
                has_quint = true;
                has_quad = false;
            } else if has_triple {
                has_quad = true;
                has_triple = false;
            } else if num_doubles > 0 {
                has_triple = true;
                num_doubles -= 1;
                continue;
            } else {
                num_doubles += 1;
            }
        }

        if has_quint {
            HandType::FiveOfAKind
        } else if has_quad {
            HandType::FourOfAKind
        } else if has_triple && num_doubles == 1 {
            HandType::FullHouse
        } else if has_triple {
            HandType::ThreeOfAKind
        } else if num_doubles == 2 {
            HandType::TwoPair
        } else if num_doubles == 1 {
            HandType::OnePair
        } else if num_doubles == 0 {
            HandType::HighCard
        } else {
            unreachable!()
        }
    }

    fn parse_new(line: &str, jack_as_joker: bool) -> Self {
        let (cards_str, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();
        let mut cards = [Card::default(); HAND_LENGTH];
        for (i, card) in cards_str.chars().enumerate() {
            cards[i] = Card::into(card, jack_as_joker);
        }

        let hand_type = Self::find_hand_type(cards);

        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

pub struct Day7Solver {}

impl Day7Solver {
    fn solve_camel_cards_game(jack_as_joker: bool) {
        let file = std::fs::read_to_string("src/day7/input.txt").unwrap();
        let mut hands: Vec<_> = file
            .lines()
            .map(|line| Hand::parse_new(line, jack_as_joker))
            .collect();
        hands.sort_by(|a, b| b.cmp(a));

        let total_winnings: u32 = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u32 * hand.bid)
            .sum();

        println!("The sum of all total winnings is {total_winnings}");
    }
}

impl Solver for Day7Solver {
    fn solve_part1() {
        Self::solve_camel_cards_game(false);
    }

    fn solve_part2() {
        Self::solve_camel_cards_game(true);
    }
}
