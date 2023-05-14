/*----------------------------------------------------------------------
Miscellaneous module

Small things that don't fit anywhere else
*/

#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::{SystemTime};
use rand::Rng;

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
Convert boolean to all ones or all zeros
  true  -> 0xff
  false -> 0x00

Pretty trivial, but a little tricky.  Relies on rust internal
representation of booleans and twos complement signed integers. This
code will be inlined by the compiler. It should be quite fast, as there
are no branches.

See the rust reference:
https://doc.rust-lang.org/reference/types/boolean.html

    An object with the boolean type has a size and alignment of 1 each.
    The value false has the bit pattern 0x00 and the value true has the
    bit pattern 0x01. It is undefined behavior for an object with the
    boolean type to have any other bit pattern.
*/

pub fn bool_to_allbits(b:bool) -> u8 {
    let mut v = b as i8;   // false => 0x00    true => 0x01
    v = -v;                // false => 0x00    true => 0xff
    v as u8
}

#[test]
fn test_bool_to_allbits() {
    fn t(b:bool, a:u8) {
        let c = bool_to_allbits(b);
        assert_eq!(c,a);
    }
    t(true,  0xff);
    t(false, 0x00);
}

/*----------------------------------------------------------------------
Time helper functions
*/

// u128 nanosecond resolution timestamps
pub fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

// f64 duration in seconds from two timestamps
pub fn duration(ts1:&u128, ts2:&u128) -> f64 {
    assert!(ts2 > ts1);
    /*
    Timestamps are in nanoseconds in u128.  This will not fit in a f64.
    Subtract first. For durations less than 1 day, there should only be
    about 16 bits of seconds. It takes about 30 bits to represent
    nanoseconds giving about 46 bits total.  This is less than the 51
    bits available in f64.
    */
    let nu:u128 = ts2 - ts1;
    let nf:f64 = nu as f64;
    nf * 1e-9
}

#[test]
fn test_time_and_duration() {
    /*
    Get a timestamp. Sleep for 100 ms. Get another timestamp. Check
    duration is 100ms (with some tolerance). Perhaps this is better as
    a example or a bench where runtime could be longer. It would also
    be good to do multiple samples and find the variance.
    */

    //FIXME
}

/*----------------------------------------------------------------------
Make a random vector of u8
specify length vector and range of each value
*/

// FIXME add some statistical tests for randomness
// FIXME optimize using "fill" method of rng
pub fn rand_vec_u8(n:usize, r:u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vu8:Vec<u8> = vec![];
    for _ in 0..n {
        vu8.push(rng.gen_range(0..r));
    }
    vu8
}

// End misc module -----------------------------------------------------
