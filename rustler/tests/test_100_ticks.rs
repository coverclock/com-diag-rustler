/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler
//

extern crate rustler;

use rustler::ticks::ticks;

#[test]
fn test_ticks_100_frequency() {
    let frequency: ticks::Ticks = ticks::frequency();
    eprintln!("frequency={}", frequency);
    assert!(frequency == 1000000000);
}

#[test]
fn test_ticks_200_now() {
    let before: ticks::Ticks = ticks::now();
    eprintln!("before={}", before);
    ticks::sleep(0);
    let after: ticks::Ticks = ticks::now();
    eprintln!("after={}", after);
    assert!(before <= after);
}

#[test]
fn test_ticks_300_sleep() {
    let frequency: ticks::Ticks = ticks::frequency();
    let before: ticks::Ticks = ticks::now();
    ticks::sleep(frequency);
    let after: ticks::Ticks = ticks::now();
    eprintln!("elapsed={}", after - before);
    assert!(before <= after);
    assert!((after - before) >= frequency);
    assert!((after - before) <= (2 * frequency));
}
