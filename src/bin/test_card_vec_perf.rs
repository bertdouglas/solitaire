/*----------------------------------------------------------------------
Measure performance of CardVec methods

Make sure that optimization removes copies.
Measure performance at several different length vectors.
Execution time should not depend on length.
*/

#[path = "../card.rs"]
mod card;
#[path = "../misc.rs"]
mod misc;

use card::*;
use misc::*;

fn main() {

    let sizes:Vec<usize> = vec![100,10_000,1_000_000];
    let mut time_to_u8:Vec<f64> = vec![];
    let mut time_fr_u8:Vec<f64> = vec![];

    for size in &sizes {
        // from Vec<u8>
        let vin = rand_vec_u8(*size);
        let ts0 = timestamp();
        let cv:CardVec = CardVec::from_vec_u8(vin.clone());
        let ts1 = timestamp();
        time_fr_u8.push(duration(&ts0,&ts1));

        // to Vec<u8>
        let ts0 = timestamp();
        let vout = cv.to_vec_u8();
        let ts1 = timestamp();
        time_to_u8.push(duration(&ts0,&ts1));

        assert_eq!(vout,vin);
    }
    println!("vector sizes :  {:?}",sizes);
    println!("to_u8 times  :  {:?}",time_to_u8);
    println!("from_u8 times:  {:?}",time_fr_u8);

    // show deviation as per cent of geometric mean


}
