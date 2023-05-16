/*----------------------------------------------------------------------

(c) Copyright Bert Douglas 2023.
SPDX-License-Identifier: AGPL-3.0-or-later
*/

fn main() {

    //deck::test_shuffle();


    use colored::Colorize;

    println!("{}","       ".truecolor(0,255,0).on_truecolor(0,255,0));


    let s = ""      .to_string()
        + &"  "     .truecolor(0,255,0)  .on_truecolor(0,255,0)       .to_string()
        + &"10\u{2665}"      .truecolor(255,0,0)    .on_truecolor(255,255,255).bold().underline()   .to_string()
        + &"  "     .truecolor(0,255,0)  .on_truecolor(0,255,0)       .to_string()
        ;

    let t = ""      .to_string()
        + &"  "     .truecolor(0,255,0)  .on_truecolor(0,255,0)       .to_string()
        + &"A \u{2660}"      .truecolor(0,0,0)    .on_truecolor(255,255,255).bold().underline()   .to_string()
        + &"  "     .truecolor(0,255,0)  .on_truecolor(0,255,0)       .to_string()
        ;


    println!("{}",s);
    println!("{}",t);


    println!("{}","       ".truecolor(0,255,0).on_truecolor(0,255,0));


/*
    println!("{:?}",d);

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

