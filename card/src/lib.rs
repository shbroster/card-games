pub trait Card {
    fn from_id(id: u8) -> Self;
}

pub struct Deck<C: Card> {
    cards: Vec<C>,
}

impl<C> Deck<C>
where
    C: Card,
{
    pub fn new(size: u32) -> Deck<C> {
        let cards: Vec<C> = (0..size).map(|i| C::from_id(i as u8)).collect();
        Deck { cards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
