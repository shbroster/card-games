use std::collections::VecDeque;

pub trait Card {
    fn from_id(id: u8) -> Self;
}

pub struct Deck<C: Card> {
    cards: VecDeque<C>,
}

impl<C> Deck<C>
where
    C: Card,
{
    pub fn new(size: u32) -> Deck<C> {
        let cards: VecDeque<C> = (0..size).map(|i| C::from_id(i as u8)).collect();
        Deck { cards }
    }

    pub fn take_top_card(&mut self) -> Option<C> {
        self.cards.pop_front()
    }

    pub fn put_card_on_botton(&mut self, card: C) {
        self.cards.push_back(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestCard {
        id: u8,
    }

    impl Card for TestCard {
        fn from_id(id: u8) -> TestCard {
            TestCard { id }
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_a_deck() {
        let deck: Deck<TestCard> = Deck::new(3);
        assert_eq!(deck.cards.len(), 3);
        assert_eq!(deck.cards[0].id, 0);
        assert_eq!(deck.cards[1].id, 1);
        assert_eq!(deck.cards[2].id, 2);
    }

    fn add_cards() {
        let mut deck: Deck<TestCard> = Deck::new(3);

        // Take the fist card from the deck
        let top_card_id = deck.cards.front().unwrap().id;
        let first_card = deck.take_top_card().unwrap();
        assert_eq!(deck.cards.len(), 1);
        assert_eq!(first_card.id, top_card_id);

        // Take the last card from the deck
        let top_card_id = deck.cards.front().unwrap().id;
        let first_card = deck.take_top_card().unwrap();
        assert_eq!(deck.cards.len(), 0);
        assert_eq!(first_card.id, top_card_id);

        // Attempt to take a card from an empty deck
        match deck.take_top_card() {
            Some(_) => panic!("Deck still has cards!"),
            None => (),
        }
    }

    fn return_cards() {
        let mut deck: Deck<TestCard> = Deck::new(0);

        // Add a card to the empty Deck
        assert_eq!(deck.cards.len(), 0);
        let c1 = TestCard::from_id(0);
        let c1_id = c1.id;
        deck.put_card_on_botton(c1);
        assert_eq!(deck.cards.back().unwrap().id, c1_id);
        assert_eq!(deck.cards.len(), 1);

        // Add another card to the deck
        let c2 = TestCard::from_id(0);
        let c2_id = c2.id;
        deck.put_card_on_botton(c2);
        assert_eq!(deck.cards.back().unwrap().id, c2_id);
        assert_eq!(deck.cards.len(), 2);
    }
}
