use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Table {
    hands: Vec<Hand>
}

#[derive(Clone)]
struct Hand {
    score: usize,
    j_score: usize,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Hand {
        let score= Hand::get_score(&cards);
        let j_score= Hand::get_j_score(&cards);

        Hand { score, j_score, bid}
    }

    fn get_score(cards: &Vec<Card>) -> usize {
        let mut cards_cop = cards.iter().copied().collect::<Vec<_>>();
        cards_cop.sort();

        let rank = match cards_cop {
            _ if cards_cop[0] == cards_cop[4] => Rank::FiveOfAKind,
            _ if cards_cop[0] == cards_cop[3] || cards_cop[1] == cards_cop[4] => Rank::FourOfAKind,
            _ if cards_cop[0] == cards_cop[2] && cards_cop[3] == cards_cop[4] ||
                 cards_cop[0] == cards_cop[1] && cards_cop[2] == cards_cop[4] => Rank::FullHouse,
            _ if cards_cop[0] == cards_cop[2] || cards_cop[1] == cards_cop[3] ||  cards_cop[2] == cards_cop[4] => Rank::ThreeOfAKind,
            _ => {
                let c = cards_cop.windows(2).filter(|pair| pair[0] == pair[1]).collect::<Vec<_>>().len();

                match c {
                    2 => Rank::TwoPair,
                    1 => Rank::OnePair,
                    0 => Rank::HighCard,
                    _ => panic!("Bad input")
                }
            }
        };

        let mut score = rank as usize;
        for card in cards {
            score = (score << 4) | *card as usize
        }

        score
    }

    fn get_j_score(cards: &Vec<Card>) -> usize {
        let cards = cards.iter().map(|c| if *c == Card::Jack { Card::Joker } else { *c }).collect::<Vec<_>>();

        let mut sorted_cards = cards.iter().cloned().collect::<Vec<_>>();
        sorted_cards.sort();

        let joker_count = sorted_cards.iter().filter(|c| **c == Card::Joker).count();

        let rank = match sorted_cards {
            _ if sorted_cards[0] == sorted_cards[4] => Rank::FiveOfAKind,
            _ if (sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4]) && (joker_count == 1 || joker_count == 4) => Rank::FiveOfAKind,
            _ if (sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4]) && joker_count == 0 => Rank::FourOfAKind,
            _ if (sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4] ||
                  sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4]) && 
                  (joker_count == 3 || joker_count == 2) => Rank::FiveOfAKind,
            _ if (sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4] ||
                 sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4]) && joker_count == 0 => Rank::FullHouse,
            _ if (sorted_cards[0] == sorted_cards[2] || sorted_cards[1] == sorted_cards[3] ||  sorted_cards[2] == sorted_cards[4]) && (joker_count == 3 || joker_count == 1) => Rank::FourOfAKind,
            _ if (sorted_cards[0] == sorted_cards[2] || sorted_cards[1] == sorted_cards[3] ||  sorted_cards[2] == sorted_cards[4]) 
            && joker_count == 0 => Rank::ThreeOfAKind,
            _ => {
                let c = sorted_cards.windows(2).filter(|pair| pair[0] == pair[1]).collect::<Vec<_>>().len();

                match c {
                    2 if joker_count == 2 => Rank::FourOfAKind,
                    2 if joker_count == 1 => Rank::FullHouse,
                    2 if joker_count == 0 => Rank::TwoPair,
                    1 if joker_count == 2 || joker_count == 1 => Rank::ThreeOfAKind,
                    1 if joker_count == 0 => Rank::OnePair,
                    0 if joker_count == 1 => Rank::OnePair,
                    0 if joker_count == 0 => Rank::HighCard,
                    _ => panic!("Incorrect logic for hand with pair count {c} and hand {cards:?}")
                }
            }
        };

        let mut score = rank as usize;
        for card in cards {
            score = (score << 4) | card as usize
        }

        score
    }
}

#[derive(Copy, Clone)]
enum Rank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn to_card(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Bad input")
        }
    }
}

fn main() -> Result<()> {
    let table = parse()?;

    let _result1 = part1(&table)?;
    let result2 = part2(&table);

    result2
}

fn parse() -> Result<Table> {
    let input = fs::read_to_string("input.txt")?;

    let hands = input.split("\r\n")
    .map(|x| x.split_once(' ').unwrap())
    .map(|(cards, bid)| (cards.chars().map(Card::to_card).collect::<Vec<Card>>(), bid.parse().unwrap()))
    .map(|(cards, bid)| Hand::new(cards, bid))
    .collect::<Vec<_>>();

    Ok(Table { hands })
}

fn part1(table: &Table) -> Result<()> {
    let mut sorted_hands = table.hands.clone();
    sorted_hands.sort_by(|a, b| a.score.cmp(&b.score));
    let win_amount: usize = sorted_hands.iter().enumerate().map(|(idx, hand)| hand.bid * (idx + 1)).sum();

    println!("Part 1 answer: {}", win_amount);
    return Ok(());
}

fn part2(table: &Table) -> Result<()> {
    let mut sorted_hands = table.hands.clone();
    sorted_hands.sort_by(|a, b| a.j_score.cmp(&b.j_score));
    let win_amount: usize = sorted_hands.iter().enumerate().map(|(idx, hand)| hand.bid * (idx + 1)).sum();

    println!("Part 2 answer: {}", win_amount);
    return Ok(());
}

/* --- Day 7: Camel Cards ---
Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

Find the rank of every hand in your set. What are the total winnings?

--- Part Two ---
To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
KK677 is now the only two pair, making it the second-weakest hand.
T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?*/