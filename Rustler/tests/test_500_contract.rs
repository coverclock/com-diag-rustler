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
use rustler::contract::contract;

mod harness;

#[test]
fn test_contract_220_copy() {
    let mut original: contract::Contract = contract::Contract::new().init(2, 4, 6, 8, 10);
    println!("O1={}", original.to_string());
    original.reset(12);
    println!("O2={}", original.to_string());
    let mut duplicate: contract::Contract = original;
    println!("D3={}", duplicate.to_string());
    duplicate.init(1, 3, 5, 7, 9);
    println!("D4={}", duplicate.to_string());
    duplicate.reset(11);
    println!("D5={}", duplicate.to_string());
}

#[test]
fn test_contract_240_clone() {
    let mut original: contract::Contract = contract::Contract::new().init(2, 4, 6, 8, 10);
    println!("O1={}", original.to_string());
    original.reset(12);
    println!("O2={}", original.to_string());
    let mut duplicate: contract::Contract = original.clone();
    println!("D3={}", duplicate.to_string());
    duplicate.init(1, 3, 5, 7, 9);
    println!("D4={}", duplicate.to_string());
    duplicate.reset(11);
    println!("D5={}", duplicate.to_string());
}

#[test]
fn test_contract_400_simulated() {
    let frequency: ticks::Ticks = ticks::frequency();
    let peakincrement: ticks::Ticks = gcra::increment(2048, 1, frequency);
    let burstsize: usize = 512;
    let jittertolerance: ticks::Ticks = gcra::jittertolerance(peakincrement, burstsize as throttle::Events);
    let sustainedincrement: ticks::Ticks = gcra::increment(1024, 1, frequency);
    let bursttolerance: ticks::Ticks = contract::bursttolerance(peakincrement, 0, sustainedincrement, burstsize as throttle::Events);
    let iterations: usize = 1000000;
    let now: ticks::Ticks = ticks::now();
    let mut shaper = contract::Contract::new().init(peakincrement, 0, sustainedincrement, bursttolerance, now);
    let mut policer = contract::Contract::new().init(peakincrement, jittertolerance, sustainedincrement, bursttolerance, now);
    /**/
    harness::simulate(& mut shaper, & mut policer, burstsize, iterations);
}
