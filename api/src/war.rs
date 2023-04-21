use rand::prelude::*;

/// Card data type
#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Card(u8);

/// Play a game of War with two simulated parties
pub fn play() {
    let mut deck = create_deck();
    let (_player_1, _player_2) = deal_cards(&mut deck);
}

/// Create a standard 52 card deck
fn create_deck() -> Vec<Card> {
    let mut out = Vec::new();
    for idx in 1..14 {
        for _ in 1..5 {
            out.push(Card(idx));
        }
    }
    out
}

/// Randomly deal the cards to the two players
fn deal_cards(deck: &mut Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    let mut player_1: Vec<Card> = Vec::new();
    let mut player_2: Vec<Card> = Vec::new();

    // Shuffle the deck
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    // Iteraitvely deal one card to each player
    // This could be done much more efficiently by simply
    // slicing the shuffled deck, but the rules clearly
    // state that dealing occurs one player at a time
    // TODO: Use actual error handling
    for _ in 0..26 {
        player_1.push(deck.pop().expect("Deck is empty but dealing continues"));
        player_2.push(deck.pop().expect("Deck is empty but dealing continues"));
    }

    (player_1, player_2)
}

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

    #[test]
    fn check_deck() {
        let deck = create_deck();

        assert_eq!(deck.len(), 52);

        assert_eq!(deck[0], Card(1));
        assert_eq!(deck[10], Card(3));
        assert_eq!(deck[51], Card(13));
    }

    #[test]
    fn check_deal_basics() {
        let mut deck = create_deck();

        let (deck_1, deck_2) = deal_cards(&mut deck);

        assert_eq!(deck_1.len(), deck_2.len());
        assert_eq!(deck_1.len(), 26);
    }
}
