use rand::seq::SliceRandom;

fn main() {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Suit {
        Club,
        Diamond,
        Heart,
        Spade,
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Card {
        suit: Suit,
        rank: i32,
    }

    // 카드 덱 만들기
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    // 카드 섞기
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    println!("{:?}", deck);
}
