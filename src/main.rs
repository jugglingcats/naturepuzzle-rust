extern crate rand;

use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct PartialCreature(Creature, BodyPart);

impl PartialCreature {
    fn compatible_with(&self, other: &PartialCreature) -> bool {
        // creature must be the same and head/tail must be different
        self.0 == other.0 && self.1 != other.1
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Card {
    id: usize,
    creatures: [PartialCreature; 4],
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum BodyPart {
    Head,
    Tail,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Creature {
    Octopus,
    RedFish,
    Seahorse,
    OrangeFish,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct PlacedCard<'a>(&'a Card, usize);

impl<'a> PlacedCard<'a> {
    fn test_right(&self, other: &PlacedCard) -> bool {
        let east_creature: usize = (5 - self.1) % 4;
        let west_creature: usize = 3 - other.1;

        return self.0.creatures[east_creature].compatible_with(&other.0.creatures[west_creature]);
    }

    fn test_below(&self, other: &PlacedCard) -> bool {
        let top_creature: usize = (4 - other.1) % 4;
        let bottom_creature: usize = (6 - self.1) % 4;

        return self.0.creatures[bottom_creature].compatible_with(&other.0.creatures[top_creature]);
    }
}

use Creature::{Octopus, OrangeFish, RedFish, Seahorse};
use BodyPart::{Head, Tail};

fn main() {
    let cards: [Card; 9] = [
        Card {
            id: 10,
            creatures: [
                PartialCreature(Octopus, Head),
                PartialCreature(RedFish, Head),
                PartialCreature(Seahorse, Head),
                PartialCreature(OrangeFish, Tail),
            ],
        },
        Card {
            id: 20,
            creatures: [
                PartialCreature(RedFish, Tail),
                PartialCreature(RedFish, Head),
                PartialCreature(Octopus, Head),
                PartialCreature(OrangeFish, Tail),
            ],
        },
        Card {
            id: 30,
            creatures: [
                PartialCreature(RedFish, Tail),
                PartialCreature(Seahorse, Head),
                PartialCreature(OrangeFish, Head),
                PartialCreature(OrangeFish, Tail),
            ],
        },
        Card {
            id: 40,
            creatures: [
                PartialCreature(RedFish, Head),
                PartialCreature(Octopus, Head),
                PartialCreature(Octopus, Tail),
                PartialCreature(Seahorse, Tail),
            ],
        },
        Card {
            id: 50,
            creatures: [
                PartialCreature(OrangeFish, Head),
                PartialCreature(Octopus, Head),
                PartialCreature(OrangeFish, Head),
                PartialCreature(RedFish, Head),
            ],
        },
        Card {
            id: 60,
            creatures: [
                PartialCreature(Octopus, Tail),
                PartialCreature(OrangeFish, Tail),
                PartialCreature(RedFish, Tail),
                PartialCreature(OrangeFish, Head),
            ],
        },
        Card {
            id: 70,
            creatures: [
                PartialCreature(Seahorse, Head),
                PartialCreature(RedFish, Head),
                PartialCreature(Octopus, Head),
                PartialCreature(Seahorse, Head),
            ],
        },
        Card {
            id: 80,
            creatures: [
                PartialCreature(Seahorse, Tail),
                PartialCreature(Octopus, Tail),
                PartialCreature(Seahorse, Head),
                PartialCreature(RedFish, Tail),
            ],
        },
        Card {
            id: 90,
            creatures: [
                PartialCreature(Octopus, Tail),
                PartialCreature(OrangeFish, Tail),
                PartialCreature(Seahorse, Tail),
                PartialCreature(Seahorse, Tail),
            ],
        },
    ];

    let remaining_cards: HashSet<&Card> = cards.into_iter().collect();
    let mut layout: Vec<PlacedCard> = Vec::new();
    let mut counts = (0, 0);

    let start = Instant::now();
    attempt(&mut layout, remaining_cards, &mut counts);

    println!(
        "Placed cards {:?}, solutions {:?}, time: {:?}",
        counts.0,
        counts.1,
        start.elapsed()
    );
}

fn attempt<'a>(layout: &mut Vec<PlacedCard<'a>>, remaining_cards: HashSet<&'a Card>, counts: &mut (u32, u32)) {
    for card in remaining_cards.iter() {
        // println!("{:?}", card);
        for orientation in 0..4 {
            let new_card = PlacedCard(card, orientation);
            counts.0 += 1;
            if test(layout, &new_card) {
                layout.push(new_card);
                if layout.len() == 9 {
                    counts.1 += 1;
                } else {
                    let mut rest: HashSet<&Card> = remaining_cards.clone();
                    rest.remove(card);
                    attempt(layout, rest, counts);
                }
                layout.pop();
            }
        }
    }
}

fn test(layout: &Vec<PlacedCard>, placed_card: &PlacedCard) -> bool {
    for existing_card in layout.iter() {
        if existing_card == placed_card {
            return false;
        }
    }

    let position = layout.len();
    let col = position % 3;
    let row = position / 3;

    if col > 0 {
        let left: &PlacedCard = &layout[position - 1];
        if !left.test_right(placed_card) {
            return false;
        }
    }

    if row > 0 {
        let above = &layout[position - 3];
        if !above.test_below(placed_card) {
            return false;
        }
    }

    true
}
