/*----------------------------------------------------------------------
Deck module

Create and shuffle decks of cards
*/

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::card::*;
use crate::misc::*;

/*----------------------------------------------------------------------
(c) Copyright Bert Douglas 2023.

This is an original work of Bert Douglas, begun in 2023.  It is
available for use according to the terms of this license:
    GNU Affero General Public License v3.0 or later
    https://www.gnu.org/licenses/agpl-3.0.txt

Commercial licenses may be negotiated by contacting me at:
  <georgehdouglas@gmail.com>
*/

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Deck {
    pub cards:Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selectors {
    pub sels:Vec<u8>,
}

// new standard deck in canonical order
impl Deck {
pub fn new() -> Deck {
    let ci = Card::info();
    let mut cards:Vec<u8> = vec![];
    for s in 0..ci.n_suits {
        for r in 0..ci.n_ranks {
            let card:u8 = ((s<<4) | r) as u8;
            cards.push(card);
        }
    }
    Deck { cards, }
}}

// random selectors for shuffling a deck
fn rand_selectors() -> Selectors {
    let n_cards = Card::info().n_cards;
    let sels = rand_vec_u8(n_cards,2);
    Selectors{ sels, }
}

// test deck for validity
impl Deck {
fn valid(&self) -> bool {
    // get new reference deck and copy of self for testing
    let dref = Deck::new();
    let mut dtest:Deck = (*self).clone();
    // sort the cards in deck to be tested
    dtest.cards.sort();
    // should be the same
    dtest == dref
}}

/*----------------------------------------------------------------------
Shuffle

Simulate a human shuffle
- split the cards exactly into two equal groups
- get a random number 0 or 1, like flipping a coin
- use random number to determine from which pile to take the next card
- collect cards in new pile
- repeat several times

Allow caller to supply some selectors so that results can be
deterministic.  This is convenient when comparing different shuffle
functions.
*/

impl Deck {
pub fn shuffle(&mut self, mut vsels:Vec<Selectors>, nrounds:usize) {
    let n_cards2 = Card::info().n_cards/2;
    for _ in 0..nrounds {
        // replenish selectors if empty
        if 0 == vsels.len() {
            vsels.push(rand_selectors());
        }

        // get slices for each half of the deck
        let v0 = &self.cards[..n_cards2];
        let v1 = &self.cards[n_cards2..];
        // new deck after this step
        let mut dnew:Vec<u8> = vec![];
        // consume selectors
        let sels = vsels.remove(0).sels;

        let mut i0 = 0;
        let mut i1 = 0;
        for s in sels {
            let c = match (s, i0 < n_cards2, i1 < n_cards2) {
                (0, true , _     ) => { let c = v0[i0]; i0+=1; c},
                (1, _    , true  ) => { let c = v1[i1]; i1+=1; c},
                (0, false, true  ) => { let c = v1[i1]; i1+=1; c},
                (1, true , false ) => { let c = v0[i0]; i0+=1; c},
                 _                 => panic!(),
            };
            dnew.push(c);
        }
        *self = Deck {cards : dnew};
    }
}}

#[test]
fn test_shuffle() {
    use std::collections::HashMap;

    //println!("Start test_shuffle");
    //let start:u128 = misc::timestamp();

    // shuffle many times and put decks in hashmap
    // if there is a duplicate, we fail the test
    const NSHUFFLES:usize = 10000;
    const NROUNDS:usize = 10;
    let mut deck = Deck::new();
    let mut hm:HashMap<Deck, usize> = HashMap::new();
    for i in 0..NSHUFFLES {
        if 0==(i%(NSHUFFLES/20)) {
            //println!("shuffling {}",i);
            //println!("{:?}",&deck.cards);
        }
        deck.shuffle(vec![], NROUNDS);
        assert_eq!(None, hm.insert(deck.clone(), i));
    }
    assert!(deck.valid());

    //let end:u128 = misc::timestamp();
    //let dur:f64 = misc::duration(&start, &end);
    //println!("/nFinished test_shuffle");
    //println!("Elapsed time seconds: {}", dur);
    //println!("nrounds : {}  nshuffles: {}", NROUNDS, NSHUFFLES);
    //println!("Insert into hash table and check for duplicates");
    //let rate:f64 = (NSHUFFLES as f64) * (NROUNDS as f64) / dur;
    //println!("Shuffles/second rate: {}", rate);
}

/*----------------------------------------------------------------------
Test if deck is random based on number of runs
A run is a sequence of values that increase or decrease
*/

//fn runs_test(d:&Deck) {
//}
