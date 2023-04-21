/// Card data type
#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Card(u8);

#[cfg(test)]
mod test_war {
    use super::*;

    #[test]
    fn card_eq() {
        let ace_spades = Card(1);
        let ace_hearts = Card(1);
        assert_eq!(ace_spades, ace_hearts);
    }

    #[test]
    fn card_ord() {
        let five_clubs = Card(5);
        let jack_diamonds = Card(11);
        assert!(five_clubs < jack_diamonds);
    }
}
