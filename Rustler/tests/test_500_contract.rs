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
fn test_contract_050_sanity() {
    let mut this: contract::Contract = contract::CONTRACT;
    this.init(1, 2, 3, 4, 5);
    this.reset(6);
    assert!(contract::SIZE_OF_CONTRACT == this.size_of());
}

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
    let pair = harness::simulate(& mut shaper, & mut policer, burstsize, iterations);
    assert!(harness::fabs(pair.0 - 2048.0) < (2048.0 / 100.0));
    assert!(harness::fabs(pair.1 - 1024.0) < (1024.0 / 100.0));
}

use std::sync;

#[test]
fn test_contract_500_exercised() {
    let frequency: ticks::Ticks = ticks::frequency();
    let peakincrement: ticks::Ticks = gcra::increment(1024, 1, frequency);
    let burstsize: usize = 64;
    let jittertolerance: ticks::Ticks = gcra::jittertolerance(peakincrement, burstsize as throttle::Events);
    let sustainedincrement: ticks::Ticks = gcra::increment(512, 1, frequency);
    let bursttolerance: ticks::Ticks = contract::bursttolerance(peakincrement, 0, sustainedincrement, burstsize as throttle::Events);
    let total: usize = 512 * 60;
    let now: ticks::Ticks = ticks::now();
    let shape: contract::Contract = contract::Contract::new().init(peakincrement, 0, sustainedincrement, bursttolerance, now);
    let mshape: sync::Mutex<contract::Contract> = sync::Mutex::new(shape);
    let amshape: sync::Arc<sync::Mutex<contract::Contract>> = sync::Arc::new(mshape);
    let police: contract::Contract = contract::Contract::new().init(peakincrement, jittertolerance, sustainedincrement, bursttolerance, now);
    let mpolice: sync::Mutex<contract::Contract> = sync::Mutex::new(police);
    let ampolice: sync::Arc<sync::Mutex<contract::Contract>> = sync::Arc::new(mpolice);
    let result = harness::exercise_contract(amshape, ampolice, burstsize, total);
    assert!(result.0 == 0);
    assert!(result.1 == 0);
}
