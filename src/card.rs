/*----------------------------------------------------------------------
Card Module

Create and manipulate cards and decks of cards.
Ignore any possible future where there may be different kinds of
cards or decks.
*/

#![allow(dead_code)]
use std::str;
use fixedstr::fstr;
use rand::Rng;
use std::collections::HashMap;
use std::time::{SystemTime};

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

Only encode actual cards used. Abandoned using unicode lower bits.
Instead put unicode ranks in as a new column in RankInfo.  Reverse
conversion from unicode requires a search.  But this direction seems
unlikely to be used except for testing.
*/

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum RankCode {
    Ac = 0,   // Ace
    N2,       // Two
    N3,       // Three
    N4,       // Four
    N5,       // Five
    N6,       // Six
    N7,       // Seven
    N8,       // Eight
    N9,       // Nine
    NT,       // Ten
    Ja,       // Jack
    Qu,       // Queen
    Ki,       // King
}
const N_RANKS:usize = RankCode::Ki as usize + 1;
const RANK_MASK:u8 = 0xf;

#[derive(Clone, Copy, Debug)]
struct RankInfo{
    code    : RankCode,
    unicode : u8,
    repr1   : fstr<1>,
    repr2   : fstr<2>,
    name    : fstr<10>,
}

fn rank_info() -> Vec<RankInfo> {
    let mut vri:Vec<RankInfo> = vec![];
    use RankCode::*;
    let mut t = |
        code     : RankCode,
        unicode  : u8,
        repr1    : &str,
        repr2    : &str,
        name     : &str,
    | {
        let ri = RankInfo {
            code, unicode,
            repr1    : fstr::from(repr1),
            repr2    : fstr::from(repr2),
            name     : fstr::from(name),
        };
        vri.push(ri);
    };
    //  code    unicode    repr1    repr2    name
    t(   Ac,      0x1,      "A",     "Ac",   "Ace"     );
    t(   N2,      0x2,      "2",     "N2",   "Two"     );
    t(   N3,      0x3,      "3",     "N3",   "Three"   );
    t(   N4,      0x4,      "4",     "N4",   "Four"    );
    t(   N5,      0x5,      "5",     "N5",   "Five"    );
    t(   N6,      0x6,      "6",     "N6",   "Six"     );
    t(   N7,      0x7,      "7",     "N7",   "Seven"   );
    t(   N8,      0x8,      "8",     "N8",   "Eight"   );
    t(   N9,      0x9,      "9",     "N9",   "Nine"    );
    t(   NT,      0xA,      "T",     "NT",   "Ten"     );
    t(   Ja,      0xC,      "J",     "Ja",   "Jack"    );
    t(   Qu,      0xD,      "Q",     "Qu",   "Queen"   );
    t(   Ki,      0xE,      "K",     "Ki",   "King"    );
    vri
}

#[test]
fn test_ranks() {
    let ri = rank_info();
    for i in 0..N_RANKS {
        assert_eq!(i, ri[i].code as usize);
        let code:&str = &format!("{:?}",ri[i].code);
        assert_eq!(code, ri[i].repr2);
    }
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


Unicode to SuitCode translation table

========   =======   =======
SuitCode   unicode   unicode
           [7..4]    [5..4]
========   =======   =======
   1         0xC      0b00
   2         0xD      0b01
   0         0xA      0b10
   3         0xB      0b11
========   =======   =======

*/

// bit masks to extract suit attributes
const SUIT_RED        :u8 = 0x10;
const SUIT_COLOR_MASK :u8 = 0x10;
const SUIT_ROUND      :u8 = 0x20;
const SUIT_SHAPE_MASK :u8 = 0x20;
const SUIT_MASK       :u8 = 0x30;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum SuitCode {
    Sp = 0,     // spade
    Di,         // diamond
    Cl,         // club
    He,         // heart
}
const N_SUITS:usize = SuitCode::He as usize + 1;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum SuitColor {
    Red = 0,
    Black,
}

#[derive(Clone, Copy, Debug)]
struct SuitInfo {
    code     : SuitCode,
    color    : SuitColor,
    to_uni   : u8,
    from_uni : SuitCode,
    name     : fstr<8>,
}

fn suit_info() -> Vec<SuitInfo> {
    use SuitCode::*;
    use SuitColor::*;
    let mut vsi:Vec<SuitInfo> = vec![];
    let mut t = |
        code     : SuitCode,
        color    : SuitColor,
        to_uni   : u8,
        from_uni : SuitCode,
        name     : &str,
    | {
        let si = SuitInfo {
            code, color, to_uni, from_uni,  // same names
            name     : fstr::from(name),
        };
        vsi.push(si);
    };
    //                    to       from
    //  code   color    unicode   unicode    name
    t(   Sp,   Black,    0xA0,       Di,    "Spades",   );
    t(   Di,   Red,      0xC0,       Cl,    "Diamonds", );
    t(   Cl,   Black,    0xD0,       Sp,    "Clubs",    );
    t(   He,   Red,      0xB0,       He,    "Hearts",   );
    vsi
}


#[test]
fn test_suits() {
    let vsi = suit_info();
    for i in 0..N_SUITS {
        let si = &vsi[i];
        assert_eq!(i, si.code as usize);
        let scode:String = format!("{:?}",si.code);
        let sname:String = si.name.to_string();
        assert_eq!(scode, sname[0..2]);
        let red:bool = 0 != (((si.code as u8)<<4) & SUIT_RED);
        assert_eq!( red, si.color == SuitColor::Red);
        assert_eq!(!red, si.color == SuitColor::Black);
    }
}

/*-----------------------------------------------------------------------
Encoding of a card in a byte

Suit and Rank follow enums.
FaceUp:
    0 => card is face down and not visible
    1 => card is face up and visible
FaceUp is always set to zero for making and shuffling the deck.

Bit 7 must be zero for card bytes.

+---------+--------+--------+--------+--------+--------+--------+--------+
|   7     |   6    |   5    |   4    |   3    |   2    |   1    |   0    |
+---------+--------+--------+--------+--------+--------+--------+--------+
| Group=0 | FaceUp |       Suit      |                Rank               |
+---------+--------+--------+--------+--------+--------+--------+--------+

*/

const CARD_UNICODE_BASE:u32 = 0x1F000;

#[derive(Clone, Copy, Debug, Default)]
pub struct Card {
    pub code: u8,
}

#[derive(Clone, Copy, Debug, Default)]
struct UnpackedCard {
    group   : bool,
    face_up : bool,
    suit    : u8,
    rank    : u8,
}

impl Card {
fn as_u8(self) -> u8 {
    self.code as u8
}}

impl Card {
fn from_u8(card:u8) -> Card {
    Card { code : card }
}}

impl Card {
fn unpack(&self) -> UnpackedCard {
    UnpackedCard {
        group   : ( self.code & 0b1_0_00_0000) != 0,
        face_up : ( self.code & 0b0_1_00_0000) != 0,
        suit    : ((self.code & 0b0_0_11_0000) >> 4),
        rank    : ((self.code & 0b0_0_00_1111) >> 0),
    }
}}

impl Card {
fn pack(uc:&UnpackedCard) -> Card {
    Card { code :
        0
        |  ((uc.group   as u8) << 7)
        |  ((uc.face_up as u8) << 6)
        |  ((uc.suit    as u8) << 4)
        |  ((uc.rank    as u8) << 0)
    }
}}

impl Card {
fn to_unicode(&self) -> char {
    let up = self.unpack();
    let vsi = &suit_info();
    let vri = &rank_info();
    // translate the suit
    let usuit:u32 = vsi[up.suit as usize].to_uni as u32;
    // translate the rank
    let urank:u32 = vri[up.rank as usize].unicode as u32;
    let u:u32 = CARD_UNICODE_BASE | usuit | urank;
    char::from_u32(u).unwrap()
}}

impl Card {
fn from_unicode(c:char) -> Card {
    let vsi = &suit_info();
    let vri = &rank_info();
    // unpack the unicode
    let u = c as u32;
    let ubase:u32 = (u & 0xFFFFFF00)  as u32;
    let usuit:u8  = (u & 0x000000F0)  as u8;
    let urank:u8  = (u & 0x0000000F)  as u8;
    // detect bad values
    assert_eq!(ubase, CARD_UNICODE_BASE);
    assert!(usuit >= 0xA0 && usuit <= 0xD0);
    // translate suit to internal code
    let isuit:u8 = (usuit >> 4) & 0x3;
    let suit:u8 = vsi[isuit as usize].from_uni as u8;
    // translate rank to internal code (requires search)
    let ri:&RankInfo = vri.iter()
        .find(|&&ri| ri.unicode == urank)
        .unwrap();
    // construct unpacked card and pack it
    Card::pack ( &UnpackedCard {
        group   : false,
        face_up : false,
        suit    : suit,
        rank    : ri.code as u8,
    })
}}

impl Card {
fn valid(&self) -> bool {
    let up = self.unpack();
    !up.group & (up.rank < N_RANKS as u8)
}}

impl Card {
// two cards have the same color
pub fn same_color(&self,other:&Card) -> bool {
    0 == ((self.code ^ other.code) & SUIT_COLOR_MASK)
}}

impl Card {
// other is next ascending rank to self
pub fn rank_next(&self,other:Card) -> bool {
    let rs:u8 = self.code & RANK_MASK;
    let ro:u8 = other.code & RANK_MASK;
    rs + 1 == ro
}}

/*----------------------------------------------------------------------
Deck object
*/

const N_CARDS:usize = N_SUITS * N_RANKS;

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
    let mut cards:Vec<u8> = vec![];
    for s in 0..N_SUITS {
        for r in 0..N_RANKS {
            let card:u8 = ((s<<4) | r) as u8;
            cards.push(card);
        }
    }
    Deck { cards, }
}}

// random selectors for shuffling a deck
fn rand_selectors() -> Selectors {
    let mut rng = rand::thread_rng();
    let sels:Vec<u8> = (0..N_CARDS)
        .map(|_| rng.gen_range(0..2))
        .collect();
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
pub fn shuffle(&mut self, mut vsels:Vec<Selectors>, n:usize) {
    for _ in 0..n {
        // replenish selectors if empty
        if 0 == vsels.len() {
            vsels.push(rand_selectors());
        }

        // get slices for each half of the deck
        let v0 = &self.cards[..N_CARDS/2];
        let v1 = &self.cards[N_CARDS/2..];
        // new deck after this step
        let mut dnew:Vec<u8> = vec![];
        // consume selectors
        let sels = vsels.remove(0).sels;

        let mut i0 = 0;
        let mut i1 = 0;
        for s in sels {
            let c = match (s, i0 < N_CARDS/2, i1 < N_CARDS/2) {
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

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

fn duration(t1:&u128, t2:&u128) -> f64 {
    assert!(t2 > t1);
    /*
    Timestamps are in nanoseconds in u128.  This will not fit in a f64.
    Subtract first. For durations less than 1 day, there should only be
    about 16 bits of seconds. It takes about 30 bits to represent
    nanoseconds giving about 46 bits total.  This is less than the 51
    bits available in f64.
    */
    let nu:u128 = t2 - t1;
    let nf:f64 = nu as f64;
     * 1e-9
}

pub fn test_shuffle() {
    println!("Start test_shuffle");
    let start:u128 = timestamp();

    // shuffle many times and put decks in hashmap
    // if there is a duplicate, we fail the test
    const NSHUFFLES:usize = 1000000;
    const NROUNDS:usize = 10;
    let mut deck = Deck::new();
    let mut hm:HashMap<Deck, usize> = HashMap::new();
    for i in 0..NSHUFFLES {
        if 0==(i%(NSHUFFLES/20)) {
            println!("shuffling {}",i);
            println!("{:?}",&deck.cards);
        }
        deck.shuffle(vec![], NROUNDS);
        assert_eq!(None, hm.insert(deck.clone(), i));
    }
    assert!(deck.valid());

    let end:u128 = timestamp();
    let dur:f64 = duration(&start, &end);
    println!("/nFinished test_shuffle");
    println!("Elapsed time seconds: {}", dur);
    println!("nrounds : {}  nshuffles: {}", NROUNDS, NSHUFFLES);
    println!("Insert into hash table and check for duplicates");
    let rate:f64 = (NSHUFFLES as f64) * (NROUNDS as f64) / dur;
    println!("Shuffles/second rate: {}", rate);
}

/*----------------------------------------------------------------------
Test if deck is random based on number of runs
A run is a sequence of values that increase or decrease
*/

//fn runs_test(d:&Deck) {
//}

// end mod deck --------------------------------------------------------
