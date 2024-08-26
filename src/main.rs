#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // implicit return - remember to drop the semicolon
        Deck{cards}
    }
}
fn main() {
    let deck = Deck::new();
    println!("Here's your deck: {:#?}", deck);
}
