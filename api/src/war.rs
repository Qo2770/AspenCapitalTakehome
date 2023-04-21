use rand::prelude::*;
use std::collections::VecDeque;

/// Card data type
#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Card(u8);

/// Play a game of War with two simulated parties
pub fn play() -> &'static str {
    let mut deck = create_deck();
    let (mut player_1, mut player_2) = deal_cards(&mut deck);

    // Infinite loop protection
    let mut rounds = 0;

    // Play until one player is out of cards
    while !player_1.is_empty() && !player_2.is_empty() && rounds < 10_000 {
        play_round(&mut player_1, &mut player_2);
        rounds += 1;
    }

    // Return winner
    if player_1.is_empty() {
        "Player 2"
    } else if player_2.is_empty() {
        "Player 1"
    } else {
        "Draw!"
    }
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
fn deal_cards(deck: &mut Vec<Card>) -> (VecDeque<Card>, VecDeque<Card>) {
    let mut player_1: VecDeque<Card> = VecDeque::new();
    let mut player_2: VecDeque<Card> = VecDeque::new();

    // Shuffle the deck
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    // Iteraitvely deal one card to each player
    // This could be done much more efficiently by simply
    // slicing the shuffled deck, but the rules clearly
    // state that dealing occurs one player at a time
    // TODO: Use actual error handling
    for _ in 0..26 {
        player_1.push_back(deck.pop().expect("Deck is empty but dealing continues"));
        player_2.push_back(deck.pop().expect("Deck is empty but dealing continues"));
    }

    (player_1, player_2)
}

/// Play a single "War", i.e. until one player wins the hand
fn play_round(player_1: &mut VecDeque<Card>, player_2: &mut VecDeque<Card>) {
    // Add cards to the stack until one player wins it
    let mut stack: VecDeque<Card> = VecDeque::new();
    loop {
        if player_1.is_empty() || player_2.is_empty() {
            break;
        }
        // TODO: Real error checking
        stack.push_front(player_1.pop_front().expect("Player 1 ran out of cards"));
        stack.push_front(player_2.pop_front().expect("Player 2 ran out of cards"));
        if stack[0] < stack[1] {
            // Player 1 has won
            player_1.append(&mut stack);
            break;
        } else if stack[1] < stack[0] {
            // Player 2 has won
            player_2.append(&mut stack);
            break;
        }
        if player_1.is_empty() || player_2.is_empty() {
            break;
        }
        // The face down cards not used for comparison
        // TODO: Real error checking
        stack.push_front(player_1.pop_front().expect("Player 1 ran out of cards"));
        stack.push_front(player_2.pop_front().expect("Player 2 ran out of cards"));
    }
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

    #[test]
    fn check_empty_round() {
        let mut deck_1 = VecDeque::new();
        let mut deck_2 = VecDeque::new();

        play_round(&mut deck_1, &mut deck_2);
        assert_eq!(deck_1, deck_2);
    }

    #[test]
    fn check_simple_round() {
        let mut deck_1 = VecDeque::from([Card(2)]);
        let mut deck_2 = VecDeque::from([Card(5)]);

        play_round(&mut deck_1, &mut deck_2);
        assert_ne!(deck_1, deck_2);
        assert_eq!(deck_2, VecDeque::from([Card(5), Card(2)]));
        assert!(deck_1.is_empty());
    }

    #[test]
    fn check_war_round() {
        let mut deck_1 = VecDeque::from([Card(2), Card(3), Card(5)]);
        let mut deck_2 = VecDeque::from([Card(2), Card(3), Card(6)]);

        play_round(&mut deck_1, &mut deck_2);
        assert_ne!(deck_1, deck_2);
        assert_eq!(
            deck_2,
            VecDeque::from([Card(6), Card(5), Card(3), Card(3), Card(2), Card(2)])
        );
        assert!(deck_1.is_empty());
    }

    #[test]
    fn check_integration_sanity() {
        let res = play();
        assert!(res == "Player 1" || res == "Player 2" || res == "Draw!");
    }
}
