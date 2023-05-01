/*----------------------------------------------------------------------
Handle and represent groups of cards

The group object is intended to represent any possible arrangement of
cards during a game.  For many card games like solitaire, the
arrangement of cards is the entire game-state.

At first, for convenience, there will be two representations, packed
and unpacked.  The packed form is needed because of plans for automatic
solvers using exhaustive search, and the necessity to store many
states. The unpacked form will make coding and manipulation easier.
Eventually, with effort, manipulation might be done on the packed form
directly.  Or, it may turn out that the unpacked form is small enough,
and the packed form can be abandoned.

Anticipating many variants of solitaire, this module avoids getting
involved with such issues as
  - the purpose and meaning of each group
  - which moves are allowed
*/

use crate::card;
use fixedstr::fstr;


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
*/

pub const PACK_SIZE:usize = 62;  // number of groups + number of cards

/*
pub struct Layout {
    groups: Vec<Group>,
}

pub struct Group {
    name:  fstr<10>,
    cards: Vec<Card>,
}

pub struct LayoutPacked {
    [Card;PACK_SIZE];
}

pub struct CardPos {
    i_grp: usize,
    i_crd: usize,
}

impl GameState {
fn pub move(&mut self, from:&CardPos, to:&CardPos) {
    self
}}

impl GameState {
fn pub text_repr(&self) -> String {
}}

impl GameState {
fn pub pack (&self) -> GameStatePacked {
    // check for enough room
}}

impl GameState {
fn pub unpack(gspacked:GameStatePacked) -> GameState {
}}

impl GameState {
fn set_face_up(&mut self) {
}}
*/

// end mod group
