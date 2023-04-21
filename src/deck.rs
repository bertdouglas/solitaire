/*----------------------------------------------------------------------
Deck Module

Create and manipulate a standard deck of 52 cards.
Ignore any possible future where there may be different kinds of
cards or decks.
*/

use once_cell::sync::Lazy;


/*----------------------------------------------------------------------
(c) Copyright Bert Douglas 2023.

This is an original work of Bert Douglas, begun in 2023.  It is
available for use according to the terms of this license:
    GNU Affero General Public License v3.0 or later
    https://www.gnu.org/licenses/agpl-3.0.txt

Commercial licenses may be negotiated by contacting me at:
  <georgehdouglas@gmail.com>
*/

/*----------------------------------------------------------------------
Encoding of card ranks

Ranks are encoded using the lower 4 bits of the Unicode value.
See:  https://en.wikipedia.org/wiki/Playing_cards_in_Unicode

The only surprise here is the presence of an extra face card called the
Knight or Cavalier.  So far, this card is not used.
*/

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum RanksEnum {
    Z = 0,  // Reserved not used
    A,      // Ace
    N2,     // Two
    N3,     // Three
    N4,     // Four
    N5,     // Five
    N6,     // Six
    N7,     // Seven
    N8,     // Eight
    N9,     // Nine
    N10,    // Ten
    J,      // Jack
    C,      // Cavalier (aka Knight)
    Q,      // Queen
    K,      // King
    F,      // Reserved not used
}

static RANK_NAMES:Lazy<Vec<&'static str>> = Lazy::new(|| { vec![
    "Z",       "Ace",     "Two",     "Three",
    "Four",    "Five",    "Six",     "Seven",
    "Eight",   "Nine",    "Ten",     "jack",
    "Knight",  "Queen",   "King",    "F",
]});

#[test]
fn test_ranks() {
    use RanksEnum::*;
    assert_eq!(0x0, Z   as u8);
    assert_eq!(0x1, A   as u8);
    assert_eq!(0x4, N4  as u8);
    assert_eq!(0x7, N7  as u8);
    assert_eq!(0xA, N10 as u8);
    assert_eq!(0xB, J   as u8);
    assert_eq!(0xC, C   as u8);
    assert_eq!(0xD, Q   as u8);
    assert_eq!(0xE, K   as u8);
    assert_eq!(0xF, F   as u8);

    assert_eq!("Ace",    RANK_NAMES[A  as usize] );
    assert_eq!("Two",    RANK_NAMES[N2 as usize] );
    assert_eq!("Five",   RANK_NAMES[N5 as usize] );
    assert_eq!("Nine",   RANK_NAMES[N9 as usize] );
    assert_eq!("jack",   RANK_NAMES[J  as usize] );
    assert_eq!("Knight", RANK_NAMES[C  as usize] );
    assert_eq!("Queen",  RANK_NAMES[Q  as usize] );
    assert_eq!("King",   RANK_NAMES[K  as usize] );
}

static RANKS:Lazy<Vec<u8>> = Lazy::new(|| {
    use RanksEnum::*;
    vec![
        A as u8,   N2 as u8,  N3 as u8,  N4 as u8,  N5 as u8,
        N6 as u8,  N7 as u8,  N8 as u8,  N9 as u8,  N10 as u8,
        J as u8,   Q as u8,   K as u8,
    ]
});

/*----------------------------------------------------------------------
Encoding of card suit

Suits are encoded for convenient manipulation and do not follow
unicode.  A look up table is used to convert from the suit code to
unicode.

bit zero of suit code means:
    0 => color is black
    1 => color is red

bit one of suit code means (whimsically);
    0 => suit icon is pointed on top
    1 => suit icon is rounded on top

=======   ====     =====     ===     =======
suit      code     round     red     unicode
=======   ====     =====     ===     =======
spade     0b00      0         0       0xA0
diamond   0b01      0         1       0xC0
club      0b10      1         0       0xD0
heart     0b11      1         1       0xB0
=======   ====     =====     ===     =======
*/

// bit masks to extract suit attributes
pub static SUIT_RED:u8   = 0x10;
pub static SUIT_ROUND:u8 = 0x20;

// used to convert suit code to unicode
static SUIT_TO_UNICODE_LUT:Lazy<Vec<u32>> = Lazy::new(|| {
    vec![0xA0,0xC0,0xD0,0xB0]
});

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum SuitsEnum {
    Spa = 0,     // spade
    Dia,         // diamond
    Clu,         // club
    Hea,         // heart
}

static SUIT_NAMES:Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "Spades",
        "Diamonds",
        "Clubs",
        "Hearts",
    ]
});

static SUITS:Lazy<Vec<u8>> = Lazy::new(|| {
    use SuitsEnum::*;
    vec![Spa as u8, Dia as u8, Clu as u8, Hea as u8,]
});

#[test]
fn test_suits() {
    use SuitsEnum::*;
    assert_eq!(0x0, Spa  as u8);
    assert_eq!(0x1, Dia  as u8);
    assert_eq!(0x2, Clu  as u8);
    assert_eq!(0x3, Hea  as u8);

    assert!((Spa & SUIT_RED) == 0);
    assert!((Dia & SUIT_RED) != 0);
    assert!((Clu & SUIT_RED) == 0);
    assert!((Hea & SUIT_RED) != 0);

    assert!((Spa & SUIT_ROUND) == 0);
    assert!((Dia & SUIT_ROUND) == 0);
    assert!((Clu & SUIT_ROUND) != 0);
    assert!((Hea & SUIT_ROUND) != 0);

    assert_eq!("Spades",   SUIT_NAMES[Spa as usize] );
    assert_eq!("Diamonds", SUIT_NAMES[Dia as usize] );
    assert_eq!("Clubs",    SUIT_NAMES[Clu as usize] );
    assert_eq!("Hearts",   SUIT_NAMES[Hea as usize] );
}

/*-----------------------------------------------------------------------
Encoding of a card in a byte

Suit and Rank follow enums.
FaceUp:
    0 => card is face down and not visible
    1 => card is face up and visible
FaceUp is always set to zero for making and shuffling the deck.

Bit 7 must be zero for card bytes.

+--------+--------+--------+--------+--------+--------+--------+--------+
|   7    |   6    |   5    |   4    |   3    |   2    |   1    |   0    |
+--------+--------+--------+--------+--------+--------+--------+--------+
|   0    | FaceUp |       Suit      |                Rank               |
+--------+--------+--------+--------+--------+--------+--------+--------+

*/

fn card_to_unicode(card: u8) -> char {
    let suit:usize = ((card>>4) & 0x3).into();
    let rank:u32 = (card & 0x0F).into();
    let u:u32 = 0x1f000 | SUIT_TO_UNICODE_LUT[suit] | rank;
    char::from_u32(u).unwrap()
}

fn valid_card(card:u8) -> bool {
    let r = card & 0xf;
    let g = card & 0x80;
    let mut valid:bool = true;
    // FIXME not handling the Cavalier card
    valid &=  g == 0;
    valid &=  r != 0x0;
    valid &=  r != 0xf;
    valid
}

/*----------------------------------------------------------------------
Deck object
*/

pub const NDECK:usize = 52;

#[derive(Clone, Debug)]
pub struct Deck {
    pub kind:Kind,
    pub cards:Vec<u8>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Blank,         // not yet initialized
    Selectors,     // random bits used in shuffling
    Ordinals,      // sequential 0..N
    BitCodes,      // card codes with bit flags for fast classification
    Unicode,       // unicode for display
}

pub static BLANK:Lazy<Deck> = Lazy::new(|| {
    Deck {
        kind  : Kind::Blank,
        cards : vec![0;NDECK],
    }
});

// convenient card codes
pub static BIT_CODES:Lazy<Deck> = Lazy::new(|| {
    let mut out:Deck = (*BLANK).clone();
    out.kind = Kind::BitCodes;
    let mut i = 0;
    assert_eq!(out.cards.len(), (*SUITS).len() * (*RANKS).len());
    for s in (*SUITS).clone() {
        for r in (*RANKS).clone() {
            out.cards[i] = (s<<4) | r;
            i += 1;
        }
    }
    out
});

// sequence of numbers from 0 to 51
pub static ORDINALS:Lazy<Deck> = Lazy::new(|| {
    let mut out:Deck = (*BLANK).clone();
    out.kind = Kind::Ordinals;
    for i in 0..out.cards.len() {
        out.cards[i] = i as u8;
    }
    out
});

/*

// sort the deck, compare to make_deck()
fn valid(d:&DeckOrdinals) -> bool {
    let mut t = DeckOrdinals([0;NDECK]);
    t.0 = d.0.clone();
    let u = make_ordinals();
    t.0.sort();
    t.0 == u.0
}


/*----------------------------------------------------------------------
Shuffle

Simulate a human shuffle
- split the cards exactly into two equal groups
- get a random number 0 or 1, like flipping a coin
- use random number to determine from which pile to take the next card
- collect cards in new pile
- repeat several times

This requires a lot of moving of data, but it is simple and easy to get right.
A more complicated version can be done in place.
*/

fn get_rand_bits() -> Deck {
    let mut s:Deck = DECK_BLANK;
    s.kind = DeckKind::Selectors,
    let mut rng = rand::thread_rng();
    for i in 0..NDECK {
        s.cards[i] = rng.gen_range(0..2);
    }
    s
}

fn shuffle_move(d:&mut Deck, s:&Deck) {
    assert!(s.kind == DeckKind::Selectors);
    let mut v0 = d[..NDECK/2].to_vec();
    let mut v1 = d[NDECK/2..].to_vec();
    for i in 0..NDECK {
        d.cards[i] = match (s.cards[i], v0.len()>0, v1.len()>0) {
            (0, true , _     ) => v0.remove(0),
            (1, _    , true  ) => v1.remove(0),
            (0, false, true  ) => v1.remove(0),
            (1, true , false ) => v0.remove(0),
            _                  => panic!(),
        }
    }
}

fn test_shuffle_move() {
    println!("Start test_shuffle_move");
    // shuffle many times and put decks in hashmap
    // if there is a duplicate, we fail the test
    const NLOOPS:usize = 1000000;
    let mut deck = make_ordinals();
    let mut hm:HashMap<Deck, usize> = HashMap::new();
    for i in 0..NLOOPS {
        if 0==(i%(NLOOPS/20)) {
            println!("shuffling {}",i);
            println!("{:?}",deck.0);
        }
        for j in 0..10 {
            let sel = get_rand_bits();
            shuffle_move(&mut deck.0,&sel);
        }
        assert_eq!(None, hm.insert(deck.0, i));
    }
    assert!(valid_deck(&deck));
    println!("Finished test_shuffle_move\n");
}

fn shuffle_in_place(d:&mut DeckOrdinals,s:&DeckSelectors) {
    let i0:usize = 0;
    let i1:usize = NDECK/2;
    let mut p = DeckOrdinals([0;NDECK]);
}


/*----------------------------------------------------------------------
Test if deck is random based on number of runs
A run is a sequence of values that increase or decrease
*/

//fn runs_test(d:&Deck) {
//}

#![allow(unused_variables, dead_code)]

use rand::Rng;
use std::collections::HashMap;
use lazy_static::lazy_static;

/*----------------------------------------------------------------------
Encoding of card ranks

Ranks are encoded using the lower 4 bits of the Unicode value.
See:  https://en.wikipedia.org/wiki/Playing_cards_in_Unicode

The only surprise here is the presence of an extra face card called the
Knight or Cavalier.  So far, this card is not used.
*/

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Ranks {
    Z = 0,  // Reserved not used
    A,      // Ace
    N2,     // Two
    N3,     // Three
    N4,     // Four
    N5,     // Five
    N6,     // Six
    N7,     // Seven
    N8,     // Eight
    N9,     // Nine
    N10,    // Ten
    J,      // Jack
    C,      // Cavalier (aka Knight)
    Q,      // Queen
    K,      // King
    F,      // Reserved not used
}


static RANK_NAMES:[&str ; 16] = [
    "Z",
    "Ace",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "jack",
    "Knight",
    "Queen",
    "King",
    "F",
];


#[test]
fn test_ranks() {
    use Ranks::*;
    assert_eq!(0x0, Z   as u8);
    assert_eq!(0x1, A   as u8);
    assert_eq!(0x4, N4  as u8);
    assert_eq!(0x7, N7  as u8);
    assert_eq!(0xA, N10 as u8);
    assert_eq!(0xB, J   as u8);
    assert_eq!(0xC, C   as u8);
    assert_eq!(0xD, Q   as u8);
    assert_eq!(0xE, K   as u8);
    assert_eq!(0xF, F   as u8);

    assert_eq!("Ace",    RANK_NAMES[A  as usize] );
    assert_eq!("Two",    RANK_NAMES[N2 as usize] );
    assert_eq!("Five",   RANK_NAMES[N5 as usize] );
    assert_eq!("Nine",   RANK_NAMES[N9 as usize] );
    assert_eq!("jack",   RANK_NAMES[J  as usize] );
    assert_eq!("Knight", RANK_NAMES[C  as usize] );
    assert_eq!("Queen",  RANK_NAMES[Q  as usize] );
    assert_eq!("King",   RANK_NAMES[K  as usize] );
}

fn klondike_ranks() -> &'static[u8;13] {
    use Ranks::*;
    static KR:[u8;13] = [
        A as u8,    N2 as u8,    N3 as u8,   N4 as u8,   N5 as u8,
        N6 as u8,   N7 as u8,    N8 as u8,   N9 as u8,   N10 as u8,
        J as u8,    Q as u8,     K as u8,
    ];
    &KR
}

/*----------------------------------------------------------------------
Encoding of card suit

Suits are encoded for convenient manipulation and do not follow
unicode.  A look up table is used to convert from the suit code to
unicode.

bit zero of suit code means:
    0 => color is black
    1 => color is red

bit one of suit code means (whimsically);
    0 => suit icon is pointed on top
    1 => suit icon is rounded on top

=======   ====     =====     ===     =======
suit      code     round     red     unicode
=======   ====     =====     ===     =======
spade     0b00      0         0       0xA0
diamond   0b01      0         1       0xC0
club      0b10      1         0       0xD0
heart     0b11      1         1       0xB0
=======   ====     =====     ===     =======

*/

// bit masks to extract suit attributes
static SUIT_RED:u8   = 0x10;
static SUIT_ROUND:u8 = 0x20;

// used to convert suit code to unicode
static SUIT_LUT:[u32;4] = [0xA0,0xC0,0xD0,0xB0];

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Suits {
    Spa = 0,     // spade
    Dia,         // diamond
    Clu,         // club
    Hea,         // heart
}

static SUIT_NAMES:[&str ; 4] = [
    "Spades",
    "Diamonds",
    "Clubs",
    "Hearts",
];

#[test]
fn test_suits() {
    use Suits::*;
    assert_eq!(0x0, Spa  as u8);
    assert_eq!(0x1, Dia  as u8);
    assert_eq!(0x2, Clu  as u8);
    assert_eq!(0x3, Hea  as u8);

    assert_eq!("Spades",   SUIT_NAMES[Spa as usize] );
    assert_eq!("Diamonds", SUIT_NAMES[Dia as usize] );
    assert_eq!("Clubs",    SUIT_NAMES[Clu as usize] );
    assert_eq!("Hearts",   SUIT_NAMES[Hea as usize] );
}

fn klondike_suits() -> &'static[u8;4] {
    use Suits::*;
    static KS:[u8;4] = [
        Spa as u8,   Dia as u8,   Clu as u8,   Hea as u8,
    ];
    &KS
}

/*-----------------------------------------------------------------------
Encoding of a card in a byte

Suit and Rank follow enums.
FaceUp:
    0 => card is face down and not visible
    1 => card is face up and visible
FaceUp is always set to zero for making and shuffling the deck.

Bit 7 must be zero for card bytes.

+--------+--------+--------+--------+--------+--------+--------+--------+
|   7    |   6    |   5    |   4    |   3    |   2    |   1    |   0    |
+--------+--------+--------+--------+--------+--------+--------+--------+
|   0    | FaceUp |       Suit      |                Rank               |
+--------+--------+--------+--------+--------+--------+--------+--------+

*/

fn card_to_unicode(card: u8) -> char {
    let suit:usize = ((card>>4) & 0x3).into();
    let rank:u32 = (card & 0x0F).into();
    let u:u32 = 0x1f000 | SUIT_LUT[suit] | rank;
    char::from_u32(u).unwrap()
}

fn valid_card(card:u8) -> bool {
    let r = card & 0xf;
    let g = card & 0x80;
    let mut valid:bool = true;
    valid &=  g == 0;
    valid &=  r != 0x0;
    valid &=  r != 0xf;
    valid
}

lazy_static! {
    static ref DECK_BIT_CODES:Deck = deck_make_bit_codes();
}


// convenient card codes
fn deck_make_bit_codes() -> Deck {
    let mut out = DECK_BLANK;
    out.kind = DeckKind::BitCodes;
    let mut i = 0;
    for s in klondike_suits() {
        for r in klondike_ranks() {
            out.cards[i] = (s<<4) | r;
            i += 1;
        }
    }
    out
}

lazy_static! {
    static ref DECK_ORDINALS:Deck = make_ordinals();
}

// sequence of numbers from 0 to 51
fn deck_make_ordinals() -> Deck {
    let mut out = DECK_BLANK;
    out.kind = DeckKind::Ordinals;
    for i in 0..out.cards.len() {
        out.cards[i] = i as u8;
    }
    out
}

// sort the deck, compare to make_deck()
fn valid_deck(d:&DeckOrdinals) -> bool {
    let mut t = DeckOrdinals([0;NDECK]);
    t.0 = d.0.clone();
    let u = make_ordinals();
    t.0.sort();
    t.0 == u.0
}


/*----------------------------------------------------------------------
Shuffle

Simulate a human shuffle
- split the cards exactly into two equal groups
- get a random number 0 or 1, like flipping a coin
- use random number to determine from which pile to take the next card
- collect cards in new pile
- repeat several times

This requires a lot of moving of data, but it is simple and easy to get right.
A more complicated version can be done in place.
*/

enum DeckKind:u8 = {
    Blank,         // not yet initialized
    Selectors,     // random bits used in shuffling
    Ordinals,      // sequential 0..N
    BitCodes,      // card codes with bit flags for fast classification
    Unicode,       // unicode for display
}
const NDECK:usize = 52;
struct Deck {
    kind:DeckKind,
    cards:[u8;NDECK],
}

static DECK_BLANK:Deck = {
    kind  : DeckKind::Blank,
    cards : [0,NDECK];
}

fn get_rand_bits() -> Deck {
    let mut s:Deck = DECK_BLANK;
    s.kind = DeckKind::Selectors,
    let mut rng = rand::thread_rng();
    for i in 0..NDECK {
        s.cards[i] = rng.gen_range(0..2);
    }
    s
}

fn shuffle_move(d:&mut Deck, s:&Deck) {
    assert!(s.kind == DeckKind::Selectors);
    let mut v0 = d[..NDECK/2].to_vec();
    let mut v1 = d[NDECK/2..].to_vec();
    for i in 0..NDECK {
        d.cards[i] = match (s.cards[i], v0.len()>0, v1.len()>0) {
            (0, true , _     ) => v0.remove(0),
            (1, _    , true  ) => v1.remove(0),
            (0, false, true  ) => v1.remove(0),
            (1, true , false ) => v0.remove(0),
            _                  => panic!(),
        }
    }
}

fn test_shuffle_move() {
    println!("Start test_shuffle_move");
    // shuffle many times and put decks in hashmap
    // if there is a duplicate, we fail the test
    const NLOOPS:usize = 1000000;
    let mut deck = make_ordinals();
    let mut hm:HashMap<Deck, usize> = HashMap::new();
    for i in 0..NLOOPS {
        if 0==(i%(NLOOPS/20)) {
            println!("shuffling {}",i);
            println!("{:?}",deck.0);
        }
        for j in 0..10 {
            let sel = get_rand_bits();
            shuffle_move(&mut deck.0,&sel);
        }
        assert_eq!(None, hm.insert(deck.0, i));
    }
    assert!(valid_deck(&deck));
    println!("Finished test_shuffle_move\n");
}

fn shuffle_in_place(d:&mut DeckOrdinals,s:&DeckSelectors) {
    let i0:usize = 0;
    let i1:usize = NDECK/2;
    let mut p = DeckOrdinals([0;NDECK]);
}


/*----------------------------------------------------------------------
Test if deck is random based on number of runs
A run is a sequence of values that increase or decrease
*/

//fn runs_test(d:&Deck) {
//}

*/


// end mod deck
