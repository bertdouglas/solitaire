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
enum RankCode {
    R0 = 0,   // Reserved not used
    Ac,       // Ace
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
    Ca,       // Cavalier (aka Knight)
    Qu,       // Queen
    Ki,       // King
    RF,       // Reserved not used
}

const N_RANK_CODE:usize = RankCode::RF as usize + 1;

type Repr1 = [u8;1];
type Repr2 = [u8;2];
type Name =  [u8;10];

#[derive(Clone, Copy, Debug, Default)]
struct RankInfo {
    index : u8,
    code  : u8,
    next  : u8,
    repr1 : Repr1,
    repr2 : Repr2,
    name  : Name,
}

fn copy_u8_slice(src:&[u8],dst:&mut [u8]) {
    assert_eq!(src.len(), dst.len());
    for i in 0..dst.len() {
        dst[i] = src[i];
    }
}

static RANK_INFO:Lazy<[RankInfo;N_RANK_CODE]> = Lazy::new(|| {
    use RankCode::*;
    let ri_blank:RankInfo = Default::default();
    let mut ari:[RankInfo;N_RANK_CODE] = [ri_blank;N_RANK_CODE];
    let mut t = |
        i     : usize,
        code  : RankCode,
        next  : RankCode,
        repr1 : &str,
        repr2 : &str,
        name  : &str,
    | {
        ari[i].index = i as u8;
        ari[i].code  = code as u8;
        ari[i].next  = next as u8;
        copy_u8_slice(repr1.as_bytes(), &mut ari[i].repr1);
        copy_u8_slice(repr2.as_bytes(), &mut ari[i].repr2);
        copy_u8_slice(name.as_bytes(),  &mut ari[i].name);
    };
    //  index    code   next   repr1    repr2    name
    t(   0x0,     R0,    R0,    "Z",     "R0",   "Reserved_0" );
    t(   0x1,     Ac,    N2,    "A",     "Ac",   "Ace"        );
    t(   0x2,     N2,    N3,    "2",     "B2",   "Two"        );
    t(   0x3,     N3,    N4,    "3",     "N3",   "Three"      );
    t(   0x4,     N4,    N5,    "4",     "N4",   "Four"       );
    t(   0x5,     N5,    N6,    "5",     "N5",   "Five"       );
    t(   0x6,     N6,    N7,    "6",     "N5",   "Six"        );
    t(   0x7,     N7,    N8,    "7",     "N7",   "Seven"      );
    t(   0x8,     N8,    N9,    "8",     "N8",   "Eight"      );
    t(   0x9,     N9,    NT,    "9",     "N9",   "Nine"       );
    t(   0xA,     NT,    Ja,    "T",     "NT",   "Ten"        );
    t(   0xB,     Ja,    Qu,    "J",     "Ja",   "Jack"       );
    t(   0xC,     Ca,    R0,    "C",     "Ca",   "Knight"     );
    t(   0xD,     Qu,    Ki,    "Q",     "Qu",   "Queen"      );
    t(   0xE,     Ki,    R0,    "K",     "Ki",   "King"       );
    t(   0xF,     RF,    R0,    "F",     "rF",   "Reserved_F" );
    ari
});

#[test]
fn test_ranks() {
    let ri = &RANK_INFO;
    for i in 0..ri.len() {
        assert_eq!(i, ri[i].index as usize);
        assert_eq!(i, ri[i].code as usize);
        assert_eq!(format!("{:?}",ri[i].code), ri[i].repr2);
        // FIXME "next" field not tested
    }
}

type Rank = u8;
const N_RANKS:usize = 13;

// ranks in standard deck in order
static RANKS:Lazy<[Rank;N_RANKS]> = Lazy::new(|| {
    use RankCode::*;
    let vr:Vec<RankCode> = vec![
        Ac,N2,N3,N4,N5,
        N6,N7,N8,N9,NT,
        Ja,Qu,Ki,
    ];
    let mut ar:[Rank;N_RANKS] = [0;N_RANKS];
    assert_eq!(ar.len(), vr.len());
    for i in 0..ar.len() {
        ar[i] = vr[i] as Rank;
    }
    ar
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
static SUIT_RED:u8        = 0x10;
static SUIT_COLOR_MASK:u8 = 0x10;
static SUIT_ROUND:u8      = 0x20;
static SUIT_SHAPE_MASK:u8 = 0x20;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum SuitCode {
    Sp = 0,     // spade
    Di,         // diamond
    Cl,         // club
    He,         // heart
}

enum SuitColor {
    Red = 0,
    Black,
}

const N_SUIT_CODE:usize = SuitCode::He as usize + 1;

#[derive(Clone, Copy, Debug, Default)]
struct SuitInfo {
    index    : u8,
    code     : u8,
    color    : u8,
    to_uni   : u8,
    from_uni : u8,
    name     : [u8;8],
}

static SUIT_INFO:Lazy<[SuitInfo;N_SUIT_CODE]> = Lazy::new(|| {
    use SuitCode::*;
    use SuitColor::*;
    let si_blank:SuitInfo = Default::default();
    let mut asi:[SuitInfo;N_SUIT_CODE] = [si_blank;N_SUIT_CODE];
    let mut t = |
        i        : usize,
        code     : SuitCode,
        color    : SuitColor,
        to_uni   : u8,
        from_uni : SuitCode,
        name     : &str,
    | {
        asi[i].index    = i as u8;
        asi[i].code     = code as u8;
        asi[i].color    = color as u8;
        asi[i].to_uni   = to_uni;
        asi[i].from_uni = from_uni as u8;
        copy_u8_slice(name.as_bytes(), &mut asi[i].name);
    };
    //                            to       from
    //  index   code   color    unicode   unicode   name
    t(    0,     Sp,   Black,    0xA0,       Di,    "Spades",   );
    t(    1,     Di,   Red,      0xC0,       Cl,    "Diamonds", );
    t(    2,     Cl,   Black,    0xD0,       Sp,    "Clubs",    );
    t(    3,     He,   Red,      0xB0,       He,    "Hearts",   );
    asi
});

static SUITS:Lazy<[u8;N_SUIT_CODE]> = Lazy::new(|| {
    let info = &SUIT_INFO;
    let mut suits:[u8;N_SUIT_CODE] = [0;N_SUIT_CODE];
    for i in 0..N_SUIT_CODE {
        suits[i] = info[i].code;
    }
    suits
});

#[test]
fn test_suits() {
    // FIXME
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

static CARD_UNICODE_BASE:u32 = 0x1F000;

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
    let u:u32 = 0
        |   CARD_UNICODE_BASE
        |   (SUIT_INFO[up.suit as usize].to_uni as u32)
        |   (up.rank as u32);
    char::from_u32(u).unwrap()
}}

impl Card {
fn from_unicode(c:char) -> Card {
    // unpack the unicode
    let u = c as u32;
    let     base:u32 = (u & 0xFFFFFF00)  as u32;
    let mut suit:u8  = (u & 0x000000F0)  as u8;
    let     rank:u8  = (u & 0x0000000F)  as u8;
    // detect bad values
    assert_eq!(base,CARD_UNICODE_BASE);
    assert!(suit >= 0xA0 && suit <= 0xD0);
    // translate suit to internal code
    suit = (suit >> 4) & 0x3;
    suit = SUIT_INFO[suit as usize].from_uni;
    // construct unpacked card
    Card::pack ( &UnpackedCard {
        group   : false,
        face_up : false,
        suit    : suit,
        rank    : rank,
    })
}}

impl Card {
fn valid(&self) -> bool {
    let up = self.unpack();
    true
    & !up.group
    & RANKS.iter().any(|&r| r == up.rank)
}}

impl Card {
// two cards have the same color
pub fn same_color(&self,other:&Card) -> bool {
    0 == ((self.code ^ other.code) & SUIT_COLOR_MASK)
}}

impl Card {
// other is next ascending rank to self
pub fn rank_next(&self,other:Card) -> bool {
    let rs:u8 = self.code & 0xf;
    let ro:u8 = other.code & 0xf;
    ro == RANK_INFO[rs as usize].next
}}

/*----------------------------------------------------------------------
Deck object
*/

const NDECK:usize = 52;

#[derive(Clone, Debug)]
pub struct Deck {
    pub kind:Kind,
    pub cards:[u8;NDECK],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Blank,         // not yet initialized
    Selectors,     // random bits used in shuffling
    Canonical,     // card codes with bit flags for fast classification
    Unicode,       // unicode for display
}

pub static BLANK:Lazy<Deck> = Lazy::new(|| {
    Deck {
        kind  : Kind::Blank,
        cards : [0;NDECK],
    }
});

// convenient card codes
pub static CANONICAL:Lazy<Deck> = Lazy::new(|| {
    let mut out:Deck = (*BLANK).clone();
    out.kind = Kind::Canonical;
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


// end mod deck --------------------------------------------------------
