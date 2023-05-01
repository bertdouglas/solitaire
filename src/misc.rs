/*----------------------------------------------------------------------
Miscellaneous module

Small things that don't fit anywhere else
*/

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
Time helper functions
*/

pub fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

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
