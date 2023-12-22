use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day07;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Category {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Category {
    fn variants() -> Vec<Category> {
        vec![
            Category::HighCard,
            Category::OnePair,
            Category::TwoPairs,
            Category::ThreeOfAKind,
            Category::FullHouse,
            Category::FourOfAKind,
            Category::FiveOfAKind,
        ]
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<u32>,
    bet: u32,
}

impl Hand {
    fn calculate_repeated_chars(&self) -> HashMap<u32, u32> {
        let mut char_counts: HashMap<u32, u32> = HashMap::new();
        for c in &self.cards {
            *char_counts.entry(*c).or_insert(0) += 1;
        }
        char_counts
    }

    fn ordered_vec_of_repeats(&self) -> Vec<u32> {
        let mut repeats: Vec<u32> = Vec::new();
        let char_counts = self.calculate_repeated_chars();
        let mut number_of_ones = 0; // in the case we are not using jokers then this will stay at zero
        for (value, count) in char_counts {
            if value == 1 {
                number_of_ones += count;
                continue;
            }
            repeats.push(count);
        }
        repeats.sort();
        if let Some(last) = repeats.last_mut() {
            *last += number_of_ones;
        } else {
            repeats.push(number_of_ones);
        }
        repeats
    }

    fn find_category(&self) -> Category {
        let mut category = Category::HighCard;
        let repeats = self.ordered_vec_of_repeats();
        if repeats == [5] {
            category = Category::FiveOfAKind;
        } else if repeats == [1, 4] {
            category = Category::FourOfAKind;
        } else if repeats == [2, 3] {
            category = Category::FullHouse;
        } else if repeats == [1, 1, 3] {
            category = Category::ThreeOfAKind;
        } else if repeats == [1, 2, 2] {
            category = Category::TwoPairs;
        } else if repeats == [1, 1, 1, 2] {
            category = Category::OnePair;
        }
        category
    }
}

fn sum_total_winnings(contents: String, joker: bool) -> u32 {
    let mut total_winnings = 0;

    let order_of_hands_bets: Vec<u32> = find_order_of_hands_bets(contents, joker);
    for (index, bet) in order_of_hands_bets.iter().enumerate() {
        let rank = (index + 1) as u32;
        total_winnings += rank * bet;
    }
    total_winnings
}

fn find_order_of_hands_bets(contents: String, joker: bool) -> Vec<u32> {
    let order_of_hands: Vec<Hand> = find_order_of_hands(contents, joker);
    let mut order_of_hands_bets: Vec<u32> = Vec::new();
    for hand in order_of_hands {
        order_of_hands_bets.push(hand.bet);
    }
    order_of_hands_bets
}

fn find_order_of_hands(contents: String, joker: bool) -> Vec<Hand> {
    let mut order_of_hands: Vec<Hand> = Vec::new();
    let hands = convert_input_to_hands(contents, joker);

    let mut hands_by_category = short_hands_into_categories(hands);
    for category in Category::variants() {
        if let Some(hands) = hands_by_category.get_mut(&category) {
            hands.sort_by(|a, b| {
                for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                    match card_a.cmp(card_b) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                std::cmp::Ordering::Equal
            });
            order_of_hands.append(hands);
        }
    }

    order_of_hands
}

fn convert_input_to_hands(contents: String, joker: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in contents.lines() {
        let (cards, bet) = line.split_once(" ").unwrap();
        let mut cards_vec: Vec<u32> = Vec::new();
        for c in cards.chars() {
            let digit: u32 = match c {
                'T' => 10,
                'J' => {
                    if !joker {
                        11
                    } else {
                        1
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => (c as u8 - '0' as u8).try_into().unwrap(),
            };
            cards_vec.push(digit);
        }
        let hand = Hand {
            cards: cards_vec,
            bet: bet.parse::<u32>().unwrap(),
        };
        hands.push(hand);
    }
    hands
}

fn short_hands_into_categories(hands: Vec<Hand>) -> HashMap<Category, Vec<Hand>> {
    let mut hands_by_category: HashMap<Category, Vec<Hand>> = HashMap::new();
    for hand in hands {
        let category = hand.find_category();
        hands_by_category
            .entry(category)
            .or_insert(Vec::new())
            .push(hand);
    }
    hands_by_category
}

impl Solution for Day07 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let total_non_joker_winnings = sum_total_winnings(_parsed_input.to_string(), false);
        total_non_joker_winnings.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let total_joker_winnings = sum_total_winnings(_parsed_input.to_string(), true);
        total_joker_winnings.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test is currently broken?
    fn check_day07_both_case1() {
        assert_eq!(
            Day07::solve(
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
                false
            ),
            ("6440".to_string(), "5905".to_string())
        )
    }
}
