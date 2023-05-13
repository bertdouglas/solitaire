/*----------------------------------------------------------------------
Card Module

Create and manipulate cards and decks of cards.
Ignore any possible future where there may be different kinds of
cards or decks.
*/

#![allow(dead_code)]
#![allow(unused_variables)]

use std::str;
use fixedstr::fstr;
use crate::misc;

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
const SUIT_IS_RED     :u8 = 0x10;
const SUIT_COLOR_MASK :u8 = 0x10;
const SUIT_IS_ROUND   :u8 = 0x20;
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
        let red:bool = 0 != (((si.code as u8)<<4) & SUIT_IS_RED);
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

Pile:
    0 => this is a card
    1 -> this is pile marker

Reserved bit must be set to 0.

+---------+----------+-------+-------+-------+-------+-------+-------+
|    7    |    6     |   5   |   4   |   3   |   2   |   1   |   0   |
+---------+----------+-------+-------+-------+-------+-------+-------+
| Pile=0  | FaceUp   |      Suit     |              Rank             |
+---------+----------+-------+-------+-------+-------+-------+-------+
| Pile=1  | Reserved |  N = number of cards in pile [0..63]          |
+---------+----------+-------+-------+-------+-------+-------+-------+

*/

const CARD_UNICODE_BASE:u32 = 0x1F000;

#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Card {
    pub code: u8,
}

#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct CardVec {
    pub cards: Vec<Card>,
}

// FiXME check that copy is optimized away in these two functions
impl CardVec {
pub fn to_vec_u8(self) -> Vec<u8> {
    self.cards.into_iter()
        .map(|x| x.code)
        .collect()
}}

impl CardVec {
pub fn from_vec_u8(vec:Vec<u8>) -> CardVec {
    CardVec {cards: vec.into_iter()
        .map(|x| Card{code:x})
        .collect()
    }
}}

#[test]
fn test_card_vec() {
    for _ in 0..10 {
        for size in 1..1000 {
            let vin = misc::rand_vec_u8(size);
            let cv:CardVec = CardVec::from_vec_u8(vin.clone());
            let vout = cv.to_vec_u8();
            assert_eq!(vout,vin);
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CardUnpacked {
    pub pile    : bool,   // pile or card
    pub pcount  : u8,     // if pile then count of cards in pile
    pub face_up : bool,   // remaining field for cards only
    pub suit    : u8,
    pub rank    : u8,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CardInfo {
    pub n_suits:  usize,
    pub n_ranks:  usize,
    pub n_cards:  usize,
}

impl Card {
pub fn info() -> CardInfo {
    CardInfo {
        n_suits:  N_SUITS,
        n_ranks:  N_RANKS,
        n_cards:  N_SUITS * N_RANKS,
    }
}}

impl Card {
pub fn set_face_up(&mut self, up:bool) {
    // true => 0xff, false => 0x00
    let v:u8 = misc::bool_to_allbits(up);
    let m:u8 = 0b0_1_00_0000;
    self.code &= !m;                // remove old bit
    self.code |= v&m;               // insert new bit
}}

#[test]
fn test_set_face_up() {
    fn t(a:u8,b:u8,up:bool) {
        let mut s = Card { code: a };
        s.set_face_up(up);
        assert_eq!(s.code, b);
    }
    t(0b0_0_00_1010, 0b0_0_00_1010, false );
    t(0b0_0_01_1111, 0b0_1_01_1111, true  );
    t(0b0_1_10_0111, 0b0_0_10_0111, false );
    t(0b0_1_11_0101, 0b0_1_11_0101, true  );
}

impl Card {
pub fn unpack(&self) ->CardUnpacked {
    CardUnpacked {
        pile    : ( self.code & 0b1_0_00_0000) != 0,
        pcount  : ( self.code & 0b0_0_11_1111) >> 0,
        face_up : ( self.code & 0b0_1_00_0000) != 0,
        suit    : ((self.code & 0b0_0_11_0000) >> 4),
        rank    : ((self.code & 0b0_0_00_1111) >> 0),
    }
}}

impl Card {
pub fn pack(cu:&CardUnpacked) -> Card {
    Card { code :
        0
        |  ((cu.pile    as u8) << 7)
        |  ((cu.face_up as u8) << 6)
        |  ((cu.suit    as u8) << 4)
        |  ((cu.rank    as u8) << 0)
    }
}}

#[test]
fn test_pack_and_unpack() {
    // packed -> unpacked -> packed -> check

    // unpacked -> packed -> unpacked -> check

    // FIXME
}

impl Card {

pub fn to_unicode(&self) -> char {
    let up = self.unpack();
    let vsi = &suit_info();
    let vri = &rank_info();
    // translate the suit
    let usuit:u32 = vsi[up.suit as usize].to_uni as u32;
    // translate the rank
    let urank:u32 = vri[up.rank as usize].unicode as u32;
    let u:u32 = CARD_UNICODE_BASE | usuit | urank;
    char::from_u32(u).unwrap()
}

pub fn from_unicode(c:char) -> Card {
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
    Card::pack ( &CardUnpacked {
        pile    : false,
        face_up : false,
        suit    : suit,
        rank    : ri.code as u8,
        ..Default::default()
    })
}

pub fn valid(&self) -> bool {
    let up = self.unpack();
    !up.pile & (up.rank < N_RANKS as u8)
}
}

// two cards have the same color
impl Card {
pub fn same_color(&self,other:&Card) -> bool {
    0 == ((self.code ^ other.code) & SUIT_COLOR_MASK)
}}

#[test]
fn test_same_color() {
    // FIXME
}

// other is next ascending rank to self
impl Card {
pub fn rank_next(&self,other:Card) -> bool {
    let rs:u8 = self.code & RANK_MASK;
    let ro:u8 = other.code & RANK_MASK;
    rs + 1 == ro
}}

#[test]
fn test_rank_next() {
    fn rank2card(rank:RankCode) -> Card {
        Card { code : rank as u8 }
    }
    fn t(rs: RankCode, ro:RankCode, next:bool) {
        let cs = rank2card(rs);
        let co = rank2card(ro);
        assert_eq!(cs.rank_next(co), next);
    }
    use RankCode::*;
    t(Ac,N2,true  );
    t(Ac,N3,false );
    t(N2,N3,true  );
    t(N3,N2,false );
    t(NT,Ja,true  );
    t(Ja,Qu,true  );
    t(Qu,Ki,true  );
    t(Ki,Qu,false );
}

// end mod card --------------------------------------------------------
