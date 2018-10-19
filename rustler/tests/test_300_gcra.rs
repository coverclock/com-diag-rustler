/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler
//

extern crate rustler;

use rustler::ticks::ticks;
use rustler::gcra::gcra;
use rustler::throttle::throttle::Throttle;

#[test]
fn test_gcra_100_increment() {
    assert!(gcra::increment(2, 1, 4) == 2);
    assert!(gcra::increment(1, 2, 4) == 8);
    assert!(gcra::increment(2, 1, 5) == 3);
    assert!(gcra::increment(1, 2, 5) == 10);
}

#[test]
fn test_gcra_200_jittertolerance() {
    assert!(gcra::jittertolerance(2, 3) == 4);
    assert!(gcra::jittertolerance(2, 0) == 0);
    assert!(gcra::jittertolerance(2, 1) == 0);
    assert!(gcra::jittertolerance(3, 2) == 3);
}

#[test]
fn test_gcra_300_one() {
    let mut throttle: gcra::Gcra = gcra::Gcra::new();
    let increment: ticks::Ticks = 100;
    let limit: ticks::Ticks = 10;
    let mut now: ticks::Ticks = 0;
    /**/
    eprintln!("gcra={}", throttle.to_string());
    throttle.init(increment, limit, now);
    eprintln!("gcra={}", throttle.to_string());
    /**/
    assert!(throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(!throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /* SUSTAINED */
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    /**/
    assert!(throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(!throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /* CONSUME LIMIT */
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    /**/
    assert!(!throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(!throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /* FILL */
    now += increment - 2;
    assert!(throttle.request(now) == 2);
    assert!(!throttle.commit());
    /**/
    assert!(!throttle.is_empty());
    assert!(throttle.is_full());
    assert!(throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(throttle.filled());
    assert!(throttle.alarmed());
    assert!(!throttle.cleared());
     /**/
    now += increment + 1;
    assert!(throttle.request(now) == 1);
    assert!(!throttle.commit());
    /**/
    assert!(!throttle.is_empty());
    assert!(throttle.is_full());
    assert!(throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /**/
    now += increment + 1;
    assert!(throttle.request(now) == 0);
    assert!(!throttle.commit());
    /**/
    assert!(!throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /* REQUEST, RE-REQUESTS, COMMIT */
    now += increment - 2;
    assert!(throttle.request(now) == 2);
    now += 1;
    assert!(throttle.request(now) == 1);
    now += 1;
    assert!(throttle.request(now) == 0);
    assert!(!throttle.commit());
    /**/
    assert!(!throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /* REQUEST, DELAY, ADMIT */
    now += increment - 2;
    assert!(throttle.request(now) == 2);
    now += 2;
    assert!(!throttle.admit(now));
    /**/
    assert!(!throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
    /* UPDATE */
    now += increment + 10;
    assert!(throttle.update(now));
    /**/
    assert!(throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(!throttle.is_alarmed());
    assert!(throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(throttle.cleared());
    /* SUSTAINED */
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    now += increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commit());
    /**/
    assert!(throttle.is_empty());
    assert!(!throttle.is_full());
    assert!(!throttle.is_alarmed());
    assert!(!throttle.emptied());
    assert!(!throttle.filled());
    assert!(!throttle.alarmed());
    assert!(!throttle.cleared());
}
