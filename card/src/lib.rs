
pub trait Card<T> {
    fn from_id(id: u8) -> T;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
