/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

/*
fn blocksize(maximum: throttle::Events) -> throttle::Events {
    return maximum / 2;
}
*/

/*
extern crate rand;

use rand::Rng;

fn blocksize(maximum: throttle::Events) -> throttle::Events {
    let mut rng = rand::thread_rng();
    let size: throttle::Events = rng.gen_range(0, maximum) + 1;
    
    return size;
}
*/

extern {
    fn rand() -> i32; // <stdlib.h> x86_64 gcc 7.3.0 sizeof(int)==4
}
    
pub fn blocksize(maximum: i64) -> i64 {
    unsafe {
        let size: i64 = ((rand() % (maximum as i32)) + 1) as i64;
        assert!(size >= 1);
        assert!(size <= maximum);
        return size;
    }
}
