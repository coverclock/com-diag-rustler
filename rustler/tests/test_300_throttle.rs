/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use std::i64;
use rustler::throttle::throttle;

#[test]
fn test_throttle_100_events() {
    let events: throttle::Events = i64::max_value();
    assert!(events == i64::max_value());
}
