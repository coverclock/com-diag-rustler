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
fn test_gcra_050_sanity() {
    let mut this: gcra::Gcra = gcra::GCRA;
    this.init(1, 2, 3);
    this.reset(4);
    assert!(gcra::SIZE_OF_GCRA == this.size_of());
}

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
    println!("O1={}", original.to_string());
    original.reset(8);
    println!("O2={}", original.to_string());
    let mut duplicate: gcra::Gcra = original;
    println!("D 3={}", duplicate.to_string());
    duplicate.init(1, 3, 5);
    println!("D4={}", duplicate.to_string());
    duplicate.reset(7);
    println!("D5={}", duplicate.to_string());
}

#[test]
fn test_gcra_240_clone() {
    let mut original: gcra::Gcra = gcra::Gcra::new().init(2, 4, 6);
    println!("O1={}", original.to_string());
    original.reset(8);
    println!("O2={}", original.to_string());
    let mut duplicate: gcra::Gcra = original.clone();
    println!("D3={}", duplicate.to_string());
    duplicate.init(1, 3, 5);
    println!("D4={}", duplicate.to_string());
    duplicate.reset(7);
    println!("D5={}", duplicate.to_string());
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
    let result = harness::simulate(& mut shaper, & mut policer, burstsize, iterations);
    assert!(harness::fabs(result.0 - 1024.0) < (1024.0 / 100.0));
    assert!(harness::fabs(result.1 - 1024.0) < (1024.0 / 100.0));
}

use std::sync;

#[test]
fn test_gcra_500_exercised() {
    let frequency: ticks::Ticks = ticks::frequency();
    let increment: ticks::Ticks = gcra::increment(512, 1, frequency);
    let burstsize: usize = 64;
    let limit: ticks::Ticks = gcra::jittertolerance(increment, burstsize as throttle::Events);
    let total: usize = 512 * 60;
    let now: ticks::Ticks = ticks::now();
    let shape: gcra::Gcra = gcra::Gcra::new().init(increment, 0, now);
    let mshape: sync::Mutex<gcra::Gcra> = sync::Mutex::new(shape);
    let amshape: sync::Arc<sync::Mutex<gcra::Gcra>> = sync::Arc::new(mshape);
    let police: gcra::Gcra = gcra::Gcra::new().init(increment, limit, now);
    let mpolice: sync::Mutex<gcra::Gcra> = sync::Mutex::new(police);
    let ampolice: sync::Arc<sync::Mutex<gcra::Gcra>> = sync::Arc::new(mpolice);
    let result = harness::exercise_gcra(amshape, ampolice, burstsize, total);
    assert!(result.0 == 0);
    assert!(result.1 == 0);
}

#[test]
fn test_gcra_600_exercised() {
    let frequency: ticks::Ticks = ticks::frequency();
    let increment: ticks::Ticks = gcra::increment(1024, 1, frequency);
    let burstsize: usize = 64;
    let limit: ticks::Ticks = gcra::jittertolerance(increment, burstsize as throttle::Events);
    let total: usize = 1024 * 60;
    let now: ticks::Ticks = ticks::now();
    let shape: gcra::Gcra = gcra::Gcra::new().init(increment, 0, now);
    let mshape: sync::Mutex<gcra::Gcra> = sync::Mutex::new(shape);
    let amshape: sync::Arc<sync::Mutex<gcra::Gcra>> = sync::Arc::new(mshape);
    let police: gcra::Gcra = gcra::Gcra::new().init(increment, limit, now);
    let mpolice: sync::Mutex<gcra::Gcra> = sync::Mutex::new(police);
    let ampolice: sync::Arc<sync::Mutex<gcra::Gcra>> = sync::Arc::new(mpolice);
    let result = harness::exercise_gcra(amshape, ampolice, burstsize, total);
    assert!(result.0 == 0);
    assert!(result.1 == 0);
}
