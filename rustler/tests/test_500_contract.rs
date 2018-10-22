/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use rustler::ticks::ticks;
use rustler::throttle::throttle;
use rustler::gcra::gcra;
use rustler::contract::contract;

mod harness;

#[test]
fn test_contract_400_simulated() {
    let frequency: ticks::Ticks = ticks::frequency();
    let peakincrement: ticks::Ticks = gcra::increment(2048, 1, frequency);
    let burstsize: throttle::Events = 512;
    let jittertolerance: ticks::Ticks = gcra::jittertolerance(peakincrement, burstsize);
    let sustainedincrement: ticks::Ticks = gcra::increment(1024, 1, frequency);
    let bursttolerance: ticks::Ticks = contract::bursttolerance(peakincrement, 0, sustainedincrement, burstsize);
    let iterations: throttle::Events = 1000000;
    let now: ticks::Ticks = ticks::now();
    let mut shaper: contract::Contract = contract::Contract::new();
    let mut policer: contract::Contract = contract::Contract::new();
    /**/
    eprintln!("shaper={}", shaper.to_string());
    eprintln!("policer={}", policer.to_string());
    /**/
    shaper.init(peakincrement, 0, sustainedincrement, bursttolerance, now);
    policer.init(peakincrement, jittertolerance, sustainedincrement, bursttolerance, now);
    /**/
    eprintln!("shaper={}", shaper.to_string());
    eprintln!("policer={}", policer.to_string());
    /**/
    harness::simulate(& mut shaper, & mut policer, burstsize, iterations);
    /**/
    eprintln!("shaper={}", shaper.to_string());
    eprintln!("policer={}", policer.to_string());
}
