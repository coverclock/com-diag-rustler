/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use rustler::ticks::ticks;
use rustler::throttle::throttle;
use rustler::throttle::throttle::Throttle;
use rustler::gcra::gcra;

mod harness;

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
fn test_gcra_220_copy() {
    let mut original: gcra::Gcra = gcra::Gcra::new().init(2, 4, 6);
    original.reset(8);
    let mut duplicate: gcra::Gcra = original;
    duplicate.init(1, 3, 5);
    duplicate.reset(7);
}

#[test]
fn test_gcra_240_clone() {
    let mut original: gcra::Gcra = gcra::Gcra::new().init(2, 4, 6);
    original.reset(8);
    let mut duplicate: gcra::Gcra = original.clone();
    duplicate.init(1, 3, 5);
    duplicate.reset(7);
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
    /**/
    eprintln!("gcra={}", throttle.to_string());
    throttle.init(increment, limit, now);
    eprintln!("gcra={}", throttle.to_string());
}

#[test]
fn test_gcra_300_fixed() {
    let mut throttle: gcra::Gcra = gcra::Gcra::new();
    let increment: ticks::Ticks = 100;
    let limit: ticks::Ticks = 10;
    let size: throttle::Events = 10;
    let mut now: ticks::Ticks = 0;
    /**/
    eprintln!("gcra={}", throttle.to_string());
    throttle.init(increment, limit, now);
    eprintln!("gcra={}", throttle.to_string());
    /* SUSTAINED */
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    /* CONSUME LIMIT */
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment - 1;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    /* FILL */
    now += size * increment - 2;
    assert!(throttle.request(now) == 2);
    assert!(!throttle.commits(size));
    now += size * increment + 1;
    assert!(throttle.request(now) == 1);
    assert!(!throttle.commits(size));
    now += size * increment + 1;
    assert!(throttle.request(now) == 0);
    assert!(!throttle.commits(size));
    /* REQUEST, RE-REQUESTS, COMMIT */
    now += size * increment - 2;
    assert!(throttle.request(now) == 2);
    now += 1;
    assert!(throttle.request(now) == 1);
    now += 1;
    assert!(throttle.request(now) == 0);
    assert!(!throttle.commits(size));
    /* REQUEST, DELAY, ADMIT */
    now += size * increment - 2;
    assert!(throttle.request(now) == 2);
    now += 2;
    assert!(!throttle.admits(now, size));
    /* SUSTAINED AGAIN */
    now += size * increment + 10;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    assert!(throttle.commits(size));
    /**/
    eprintln!("gcra={}", throttle.to_string());
    throttle.init(increment, limit, now);
    eprintln!("gcra={}", throttle.to_string());
}

#[test]
fn test_gcra_300_variable() {
    let mut throttle: gcra::Gcra = gcra::Gcra::new();
    let increment: ticks::Ticks = 100;
    let limit: ticks::Ticks = 10;
    let mut size: throttle::Events;
    let mut now: ticks::Ticks = 0;
    let maximum: usize = 32768;
    /**/
    eprintln!("gcra={}", throttle.to_string());
    throttle.init(increment, limit, now);
    eprintln!("gcra={}", throttle.to_string());
    /* SUSTAINED */
    now = 0;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    /* CONSUME LIMIT */
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += (size * increment) - 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    /* FILL */
    now += (size * increment) - 2;
    assert!(throttle.request(now) == 2);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(!throttle.commits(size));
    now += (size * increment) + 1;
    assert!(throttle.request(now) == 1);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(!throttle.commits(size));
    now += (size * increment) + 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(!throttle.commits(size));
    /* REQUEST, RE-REQUESTS, COMMIT */
    now += (size * increment) - 2;
    assert!(throttle.request(now) == 2);
    now += 1;
    assert!(throttle.request(now) == 1);
    now += 1;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(!throttle.commits(size));
    /* REQUEST, DELAY, ADMIT */
    now += (size * increment) - 2;
    assert!(throttle.request(now) == 2);
    now += 2;
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(!throttle.admits(now, size));
    /* SUSTAINED AGAIN */
    now += (size * increment) + 10;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    now += size * increment;
    assert!(throttle.request(now) == 0);
    size = harness::blocksize(maximum) as throttle::Events;
    assert!(throttle.commits(size));
    /**/
    eprintln!("gcra={}", throttle.to_string());
    throttle.init(increment, limit, now);
    eprintln!("gcra={}", throttle.to_string());
}

#[test]
fn test_gcra_400_simulated() {
    let frequency: ticks::Ticks = ticks::frequency();
    let increment: ticks::Ticks = gcra::increment(1024, 1, frequency);
    let burstsize: usize = 32768;
    let limit: ticks::Ticks = gcra::jittertolerance(increment, burstsize as throttle::Events);
    let iterations: usize = 1000000;
    let now: ticks::Ticks = ticks::now();
    let mut shaper = gcra::Gcra::new().init(increment, 0, now);
    let mut policer = gcra::Gcra::new().init(increment, limit, now);
    /**/
    harness::simulate(& mut shaper, & mut policer, burstsize, iterations);
}

/*

#[test]
fn test_gcra_500_exercised() {
    let frequency: ticks::Ticks = ticks::frequency();
    let increment: ticks::Ticks = gcra::increment(512, 1, frequency);
    let burstsize: usize = 64;
    let limit: ticks::Ticks = gcra::jittertolerance(increment, burstsize as throttle::Events);
    let total: usize = 512 * 60;
    let now: ticks::Ticks = ticks::now();
    static mut shaper: Option<& mut gcra::Gcra> = None;
    static mut policer: Option<& mut gcra::Gcra> = None;
    // THIS IS NOT RIGHT: still trying to figure out the right way to do this.
    unsafe {
    shaper = Some(gcra::Gcra::new());
    policer = Some(gcra::Gcra::new());
    /**/
    eprintln!("shaper={}", shaper.unwrap().to_string());
    eprintln!("policer={}", policer.unwrap().to_string());
    /**/
    shaper.unwrap().init(increment, 0, now);
    policer.unwrap().init(increment, limit, now);
    /**/
    eprintln!("shaper={}", shaper.unwrap().to_string());
    eprintln!("policer={}", policer.unwrap().to_string());
    /**/
    harness::exercise(shaper.unwrap(), policer.unwrap(), burstsize, total);
    /**/
    eprintln!("shaper={}", shaper.unwrap().to_string());
    eprintln!("policer={}", policer.unwrap().to_string());
    }
}

#[test]
fn test_gcra_600_exercised() {
    let frequency: ticks::Ticks = ticks::frequency();
    let increment: ticks::Ticks = gcra::increment(1024, 1, frequency);
    let burstsize: usize = 64;
    let limit: ticks::Ticks = gcra::jittertolerance(increment, burstsize as throttle::Events);
    let total: usize = 1024 * 60;
    let now: ticks::Ticks = ticks::now();
    static mut shaper: Option<gcra::Gcra> = None;
    static mut policer: Option<gcra::Gcra> = None;
    // THIS IS NOT RIGHT: still trying to figure out the right way to do this.
    unsafe {
    /**/
    eprintln!("shaper={}", shaper.to_string());
    eprintln!("policer={}", policer.to_string());
    /**/
    shaper.init(increment, 0, now);
    policer.init(increment, limit, now);
    /**/
    eprintln!("shaper={}", shaper.to_string());
    eprintln!("policer={}", policer.to_string());
    /**/
    harness::exercise(& mut shaper, & mut policer, burstsize, total);
    /**/
    eprintln!("shaper={}", shaper.to_string());
    eprintln!("policer={}", policer.to_string());
    }
}

*/
