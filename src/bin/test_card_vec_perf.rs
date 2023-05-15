/*----------------------------------------------------------------------
Measure performance of card vector methods

Make sure that optimization removes copies.
Measure performance at several different length vectors.
Execution time should not depend on length.
*/

use solitaire::card::*;
use solitaire::misc::*;

use colored::Colorize;

fn main() {

    const NSIZES:usize = 4;
    let sizes:Vec<usize> = vec![1,100,10_000,1_000_000];
    let mut time_to_u8:Vec<f64> = vec![0.0;NSIZES];
    let mut time_fr_u8:Vec<f64> = vec![0.0;NSIZES];
    assert_eq!(sizes.len(), NSIZES);

    const N:usize = 100;

    for _ in 0..N {
        let mut i = 0;
        for size in &sizes {
            // from Vec<u8>
            let vin = rand_vec_u8(*size,255);
            let vin1 = vin.clone();
            let ts0 = timestamp();
            let vc:Vec<Card> = vec_card_from_vec_u8(vin1);
            let ts1 = timestamp();
            time_fr_u8[i] += duration(&ts0,&ts1);

            // to Vec<u8>
            let vc1 = vc.clone();
            let ts0 = timestamp();
            let vout = vec_u8_from_vec_card(vc1);
            let ts1 = timestamp();
            time_to_u8[i] += duration(&ts0,&ts1);

            assert_eq!(vout,vin);
            i += 1;
        }
    }

    // get averages
    for i in 0..time_to_u8.len() {
        time_to_u8[i] /= N as f64;
        time_fr_u8[i] /= N as f64;
    }

    println!("test_card_vec_perf start");
    println!("number of iterations:  {}", N);

    use format_num::NumberFormat;
    let num = NumberFormat::new();

    print!("\nvector sizes  :   ");
    for s in sizes      { print!("{:11}",s)};
    print!("\nfrom_u8 times :   ");
    for t in &time_fr_u8 { print!("{}S",num.format("10.4s", *t))};
    print!("\nto_u8 times   :   ");
    for t in &time_to_u8 { print!("{}S",num.format("10.4s", *t))};
    println!();

    // The middle two numbers seem to be most consistent
    fn err(x:&Vec<f64>) -> f64 {
        let a = x[1];
        let b = x[2];
        let err = (a-b)/f64::sqrt(a*b);
        f64::abs(err) * 100.0
    }

    let err_fr = err(&time_fr_u8);
    let err_to = err(&time_to_u8);
    println!("relative error from u8 :  {:4.1}", err_fr );
    println!("relative error to u8   :  {:4.1}", err_to );
    println!("relative error limit   :  {:4.1}", REL   );

    const REL:f64 = 20.0;   // relative error limit percent
    if (err_fr > REL) | (err_to > REL) {
        println!("{}","FAILED".truecolor(255,0,0).on_truecolor(0,0,0));
    } else {
        println!("{}","PASSED".truecolor(0,255,0).on_truecolor(0,0,0));
    }

    println!("test_card_vec_perf end");
    println!();

}
