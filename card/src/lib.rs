mod card;
mod deck;

pub use crate::card::Card;
pub use crate::deck::Deck;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
