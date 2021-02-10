/// Behaviour that a playing card must exhibit
pub trait Card {
    /// A method for creating a card.
    ///
    /// It should be possible to generate a deck of cards using a zero
    /// indexed range. For instance calling `from_id` on the integers
    /// 0..51 for a standard set of playing cards should give me 52
    /// distinct cards in 4 suits.
    fn from_id(id: u8) -> Self;
}

/// An extremly basic card that implements `Card`. Useful for testing.
pub struct TestCard {
    pub id: u8,
}

impl Card for TestCard {
    fn from_id(id: u8) -> TestCard {
        TestCard { id }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
