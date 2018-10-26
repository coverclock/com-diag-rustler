/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use std::os::raw;
use std::sync::mpsc;
use std::net;
use rustler::ticks::ticks;
use rustler::throttle::throttle;

/*******************************************************************************
 * SIMULATED EVENT STREAM
 ******************************************************************************/

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
    /// rand(3) <stdlib.h> libc.
    fn rand() -> raw::c_int;
}

/// Compute a random blocksize between the values 1 and maximum inclusive.
pub fn blocksize(maximum: throttle::Events) -> throttle::Events {
    unsafe {
        let size: throttle::Events = ((rand() % (maximum as raw::c_int)) + 1) as throttle::Events;
        assert!(size >= 1);
        assert!(size <= maximum);
        return size;
    }
}

/// Simulate a data stream through a shaping throttle and a policing throttle
/// given a maximum blocksize value and a limit on iterations.
pub fn simulate(shape: & mut throttle::Throttle, police: & mut throttle::Throttle, maximum: throttle::Events, iterations: throttle::Events) {
    let frequency: f64 = ticks::frequency() as f64;
    let mut delay: ticks::Ticks;
    let mut now: ticks::Ticks = 0;
    let mut duration: ticks::Ticks = 0;
    let mut size: throttle::Events = 0;
    let mut total: u64 = 0;
    let mut rate: f64;
    let mut peak: f64 = 0.0;
    let mut admissable: bool;
    let mut admitted: bool;
        
    for ii in 0..iterations {
        
        delay = shape.request(now);
        assert!(delay >= 0);
        now += delay;
        duration += delay;
        
        if ii <= 0 {
            // Do nothing.
        } else if delay <= 0 {
            // Do nothing.
        } else {
            rate = (size as f64) * frequency / (delay as f64);
            if rate > peak {
                peak = rate;
            }
        }
        
        delay = shape.request(now);
        assert!(delay == 0);
        
        size = blocksize(maximum);
        assert!(size > 0);
        assert!(size <= maximum);
        total += size as u64;
        
        admissable = shape.commits(size);
        assert!(admissable);
        
        admitted = police.admits(now, size);
        assert!(admitted);
        
    }
    
    delay = shape.get_expected();
    now += delay;
    duration += delay;
    
    admissable = shape.update(now);
    assert!(admissable);
    
    admitted = police.update(now);
    assert!(admitted);

    let average: f64 = (total as f64) / (iterations as f64);
    let seconds: f64 = (duration as f64) / frequency;
    let mean: f64 = seconds / (iterations as f64);
    let sustained: f64 = (total as f64) * frequency / (duration as f64);

    println!("simulate: total={}B mean={}B/io maximum={}B/io latency={}s/io peak={}B/s sustained={}B/s", total, average, maximum, mean, peak, sustained);
}

/*******************************************************************************
 * ACTUAL EVENT STREAM
 ******************************************************************************/

fn producer(limit: u64, output: & mut mpsc::SyncSender<u8>, total: & mut u64, checksum: & mut u16, done: & mut mpsc::Sender<bool>) {
    
}

fn shaper(input: & mut mpsc::Receiver<u8>, shape: & mut throttle::Throttle, output: & mut net::UdpSocket, address: & net::SocketAddr, done: & mut mpsc::Sender<bool>) {
    
}

fn policer(limit: u64, input: & mut net::UdpSocket, police: & mut throttle::Throttle, output: & mut mpsc::SyncSender<u8>, done: & mut mpsc::Sender<bool>) {
    
}

fn consumer(input: & mut mpsc::Receiver<u8>, total: & mut u64, checksum: & mut u16, done: & mut mpsc::Sender<bool>) {
    
}

pub fn actualize(shape: & mut throttle::Throttle, police: & mut throttle::Throttle, maximum: throttle::Events, iterations: throttle::Events) {

}
