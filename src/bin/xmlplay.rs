/*----------------------------------------------------------------------
Attempt to read xml game state file from the KDE game "kpat" which is a
collection of "patience" games.

The goal is to be able to play parallel games with kpat and compare
game state.  This is needed for testing the details of the stock and
waste, which are difficult to observe from the UI.

Kpat seems to use european nomenclature for the games.  I guess this
reflects the location of most of the KDE developers. So far I have
found these:
  patience  <-->  solitaire
     talon  <-->  stock
      play  <-->  tableau
    target  <-->  foundation
      pile  <-->  waste
*/

/*----------------------------------------------------------------------
Key excerpts of kpat xml file, sufficient to show all tags, attributes
and nesting.

Game is a header followed by a list of states, one marked current.
States are a list of moves.
Moves are a list of cards.

The deal-number and card-id are a deep mystery and can remain so.  We
just need to read this, not to write it.  Those fields do not contain
any useful information for us.

    <?xml version="1.0" encoding="UTF-8"?>
    <kpat-game game-type="klondike" game-type-options="1" deal-number="88783540">
      <state>
        <move pile="talon" position="0">
          <card id="1966600" suit="hearts" rank="eight" turn="face-down"/>
          <card id="0655875" suit="hearts" rank="
        </move>
        <move pile="play0" position="0">
          <card id="1704455" suit="hearts" rank="seven" turn="face-up"/>
        </move>
      </state>
      <state current="true">
        <move pile="pile" position="4">
          <card id="0262146" suit="clubs" rank="two" turn="face-up"/>
        </move>
      </state>
    </kpat-game>
*/

use yaserde_derive::YaDeserialize;

#[derive(Default, PartialEq, Debug, YaDeserialize, Clone)]
#[yaserde(root = "kpat-game")]
struct KpatGame {
    #[yaserde(attribute, rename = "game-type")]
    game_type : String,
    #[yaserde(attribute, rename = "game-type-options")]
    game_type_options : u32,
    #[yaserde(attribute, rename = "deal-number")]
    deal_number : u32,
    #[yaserde(rename = "state")]
    states : Vec<State>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize, Clone)]
struct State {
    #[yaserde(attribute)]
    current : bool,
    #[yaserde(rename = "move")]
    moves : Vec<Move>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize, Clone)]
struct Move {
    #[yaserde(attribute)]
    pile : String,
    #[yaserde(attribute)]
    position : u32,
    #[yaserde(rename = "card")]
    cards : Vec<Card>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize, Clone)]
struct Card {
    #[yaserde(attribute)]
    id : u32,
    #[yaserde(attribute)]
    suit : String,
    #[yaserde(attribute)]
    rank : String,
    #[yaserde(attribute)]
    turn : String,
}

fn load_kpat_xml(path:&str) -> KpatGame {
    use std::fs;
    use yaserde::de::from_str;

    let content = fs::read_to_string(path)
        .expect("Failed to read the file.");

    let kg: KpatGame = from_str(&content).unwrap();
    kg
}

fn main() {
    let kg:KpatGame = load_kpat_xml("./data/4.kpat.save.xml");
    println!("{:#?}", kg);
}

