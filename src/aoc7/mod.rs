use std::collections::{BTreeMap, HashMap};
use std::fs;

pub fn aoc7() -> Option<usize> {
    let result = fs::read_to_string("assets/aoc7/aoc7.txt");

    match result {
        Ok(file) => {
            let mut hands: Vec<Hand> = vec![];

            for line in file.lines() {
                let mut split = line.split(" ");
                let hand = split.next().unwrap();
                let hand_value: usize = split.next().unwrap().parse().unwrap();

                let mut cards: Vec<Card> = vec![];

                for card in hand.chars() {
                    match card {
                        '2' => cards.push(Card {
                            card_type: CardType::Two,
                        }),
                        '3' => cards.push(Card {
                            card_type: CardType::Three,
                        }),
                        '4' => cards.push(Card {
                            card_type: CardType::Four,
                        }),
                        '5' => cards.push(Card {
                            card_type: CardType::Five,
                        }),
                        '6' => cards.push(Card {
                            card_type: CardType::Six,
                        }),
                        '7' => cards.push(Card {
                            card_type: CardType::Seven,
                        }),
                        '8' => cards.push(Card {
                            card_type: CardType::Eight,
                        }),
                        '9' => cards.push(Card {
                            card_type: CardType::Nine,
                        }),
                        'T' => cards.push(Card {
                            card_type: CardType::Ten,
                        }),
                        'J' => cards.push(Card {
                            card_type: CardType::Jack,
                        }),
                        'Q' => cards.push(Card {
                            card_type: CardType::Queen,
                        }),
                        'K' => cards.push(Card {
                            card_type: CardType::King,
                        }),
                        _ => cards.push(Card {
                            card_type: CardType::Ace,
                        }),
                    }
                }

                hands.push(Hand { cards, hand_value })
            }

            hands.sort_by(|a, b| {
                let type_a = a.get_hand_type();
                let type_b = b.get_hand_type();

                if type_a != type_b {
                    type_a.cmp(&type_b)
                } else {
                    a.cmp(b)
                }
            });

            let mut sum = 0;

            for (index, hand) in hands.iter().rev().enumerate() {
                sum += hand.hand_value * (index + 1);
            }

            Some(sum)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum CardType {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Jack,
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Card {
    card_type: CardType,
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    hand_value: usize,
}

impl Hand {
    fn find_most_common_card_type(cards: &Vec<Card>) -> Option<CardType> {
        let mut card_type_frequency = BTreeMap::new();

        for card in cards {
            *card_type_frequency.entry(card.card_type).or_insert(0) += 1;
        }

        let mut sorted_vals = Vec::from_iter(&card_type_frequency);
        sorted_vals.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

        for x in sorted_vals {
            if x.0 != &CardType::Jack {
                return Some(*x.0);
            }
        }

        None
    }

    fn get_hand_type(&self) -> HandType {
        let mut card_counts: HashMap<CardType, usize> = HashMap::new();

        let mut cards: Vec<Card> = self.cards.iter().cloned().collect();

        let card_type =
            Hand::find_most_common_card_type(&self.cards).unwrap_or(CardType::Ace);

        for card in &mut cards {
            if card.card_type == CardType::Jack {
                card.card_type = card_type;
            }
        }

        for card in &cards {
            card_counts.insert(
                card.card_type,
                card_counts.get(&card.card_type).unwrap_or(&0) + 1,
            );
        }

        let unique_numbers = card_counts.len();
        let max_count = *card_counts.values().max().unwrap_or(&0);

        match (unique_numbers, max_count) {
            (1, 5) => HandType::FiveOfAKind,
            (_, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (_, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (_, 2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}
