use card::Card;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

struct TopTrumpCard {
    name: String,
    fields: HashMap<TopTrumpCategory, u32>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum TopTrumpCategory {
    Starts,
    Podiums,
    Championships,
    Rating,
}

impl Card for TopTrumpCard {
    fn from_id(id: u8) -> TopTrumpCard {
        // Give the card some arbitrary name for now
        let name = id.to_string();

        // Generate consistent random values based on the 'id'
        let field_names = vec![
            TopTrumpCategory::Starts,
            TopTrumpCategory::Podiums,
            TopTrumpCategory::Championships,
            TopTrumpCategory::Rating,
        ];
        let mut r: StdRng = StdRng::seed_from_u64(id as u64);
        let field_values: Vec<u32> = vec![
            r.gen_range(0..999),
            r.gen_range(0..100),
            r.gen_range(0..10),
            r.gen_range(0..100),
        ];

        let fields: HashMap<TopTrumpCategory, u32> = field_names
            .into_iter()
            .zip(field_values.into_iter())
            .collect();

        // Generate the Trump Card
        TopTrumpCard { name, fields }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_top_tump_from_id() {
        let card = TopTrumpCard::from_id(1);
        assert_eq!(card.name, "1");

        if let Some(val) = card.fields.get(&TopTrumpCategory::Starts) {
            assert_eq!(*val, 824);
        } else {
            panic!("Starts not in fields!")
        }

        if let Some(val) = card.fields.get(&TopTrumpCategory::Podiums) {
            assert_eq!(*val, 97);
        } else {
            panic!("Podiums not in fields!")
        }

        if let Some(val) = card.fields.get(&TopTrumpCategory::Championships) {
            assert_eq!(*val, 4);
        } else {
            panic!("Campionships not in fields!")
        }

        if let Some(val) = card.fields.get(&TopTrumpCategory::Rating) {
            assert_eq!(*val, 21);
        } else {
            panic!("Rating not in fields!")
        }
    }
}
