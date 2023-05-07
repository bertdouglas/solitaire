/*----------------------------------------------------------------------
Represent the layout of cards on table

A layout is the minimum amount of information needed to perform
undo/redo operations. There is an ordered list of piles of cards. Each
pile is an ordered list of zero or more cards. Each card may be face up
or face down.

There are two representations, packed and unpacked. The packed
representation consists of a single, compact, self contained block of
memory.  All the data is in one place and there are no pointers or
references to other data.  So it is suitable for use as a key in a hash
table.  The unpacked representation allows for convenient manipulation.

The layout does not contain sufficient information to properly display
the cards.  For example, some piles may be displayed in a spread out
and overlapping arrangement with one or more cards face up. Other piles
are displayed in a tight stack, with only the top card visible. Also
there is no information about the relative location or role of the
various piles.  All of this other information is stored elsewhere and
is associated with the layout by the ordering of the piles.
*/

#![allow(dead_code)]
use crate::card::Card;
use fixedstr::fstr;


/*----------------------------------------------------------------------
(c) Copyright Bert Douglas 2023.

This is an original work of Bert Douglas, begun in 2023.  It is
available for use according to the terms of this license:
    GNU Affero General Public License v3.0 or later
    https://www.gnu.org/licenses/agpl-3.0.txt
    SPDX-License-Identifier: AGPL-3.0-or-later

Commercial licenses may be negotiated by contacting me at:
  <georgehdouglas@gmail.com>
*/

/*----------------------------------------------------------------------
*/

pub const PACK_SIZE:usize = 62;  // number of groups + number of cards

#[derive(Clone, Debug, Default)]
pub struct Layout {
    piles: Vec<Pile>,
}

#[derive(Clone, Debug, Default)]
pub struct Pile {
    cards: Vec<Card>,
}

#[derive(Clone, Copy, Debug)]
pub struct LayoutPacked {
    cards: [Card;PACK_SIZE],
}

impl Default for LayoutPacked {
    fn default() -> LayoutPacked {
        LayoutPacked { cards: [Card { code: 0 };PACK_SIZE] }
    }
}

impl Layout {

/*----------------------------------------------------------------------
Move n cards from the tail of one pile to the tail of another pile.

This will panic on an invalid move.  A move is considered invalid only
if the piles or cards referenced do not exist.  There is no check
for consistency with the rules of the game.
*/

pub fn move_tail(&mut self, from_pile:usize, to_pile:usize, n:usize) {
}

pub fn to_text(&self) -> String {
    let out = String::new();
    out
}

pub fn pack(&self) -> LayoutPacked {
    // check for enough room
    Default::default()
}

pub fn unpack(lp:LayoutPacked) -> Layout {
    Default::default()
}

/*----------------------------------------------------------------------
Set tail of pile face up or face down
*/

pub fn set_tail_face_up(&mut self, ipile:usize, n:usize, up:bool) {
    let pile = &mut self.piles[ipile];
    let len = pile.cards.len();
    for i in (len-n)..len {
        pile.cards[i].set_face_up(up);
    }
}

/*----------------------------------------------------------------------
Reverse order and reverse "face up" sense of all cards in pile
This corresponds to turning over a stack of cards.
*/
pub fn flip_pile(&mut self, ipile:usize) {
}


}  // end impl Layout
// end mod layout ------------------------------------------------------
