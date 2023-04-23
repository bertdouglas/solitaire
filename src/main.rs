mod deck;
//pub use crate::deck;

fn main() {

    deck::test_ranks();
    //test_shuffle_move();

    let d:deck::Deck = (*deck::BLANK).clone();
    println!("{:?}",d);
    /*
    for _ in 0..10 {
        let sel = get_rand_bits();
        shuffle_move(&mut deck.0,&sel);
    }
    println!("{:?}",deck.0);
    assert!(valid_deck(&deck));

    let mut deck = make_cards();
    println!("\n{:x?}",deck.0);
    let sel = get_rand_bits();
    shuffle_move(&mut deck.0, &sel);
    println!("\n{:x?}",deck.0);
    */

}

