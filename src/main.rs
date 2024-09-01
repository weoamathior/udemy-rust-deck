use rand::{thread_rng,seq::SliceRandom};

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

        // implicit return - remember to drop the semicoln
        Deck{cards}
    }

    fn shuffle(&mut self) {

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);

    }

    fn deal(&mut self, num_cards: usize )-> Vec<String> {

        // implicit return
        self.cards.split_off(
            self.cards.len() - num_cards
        )
        
    }
}
fn main() {
    let mut deck = Deck::new();
    // deck.shuffle();

    let mut dealed: Vec<String> = deck.deal(7);

    // println!("Here's your deck: {:#?}", deck);
    println!("Here's your deal : {:#?}", dealed);
    println!("Here's your deck: {:#?}", deck);

}
