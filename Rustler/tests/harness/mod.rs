/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use std::os::raw;
use std::sync;
use std::sync::mpsc;
use std::net;
use std::thread;
use rustler::ticks::ticks;
use rustler::throttle::throttle;
use rustler::fletcher::fletcher;

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

    println!("simulate: total={}B mean={}B/io maximum={}B/io latency={}s/io peak={}B/s sustained={}B/s.", total, average, maximum, mean, peak, sustained);
}

/*******************************************************************************
 * ACTUAL EVENT STREAM
 ******************************************************************************/

fn producer(limit: usize, output: & mut mpsc::SyncSender<u8>, total: & mut usize, checksum: & mut u16) {
    let mut buffer = [0u8; 65536];
    
}

fn shaper(input: & mut mpsc::Receiver<u8>, shape: & mut throttle::Throttle, output: & mut net::UdpSocket, address: & net::SocketAddrV4) {
    let mut buffer = [0u8; 65536];
    
}

fn policer(input: & mut net::UdpSocket, police: & mut throttle::Throttle, output: & mut mpsc::SyncSender<u8>) {
    let mut buffer = [0u8; 65536];
    
}

fn consumer(input: & mut mpsc::Receiver<u8>, total: & mut usize, checksum: & mut u16) {
    let mut buffer = [0u8; 65536];
    
}

pub fn actualate(shape: & mut throttle::Throttle, police: & mut throttle::Throttle, maximum: usize, total: usize) {
    let mut producertotal: usize = 1;
    let mut producerchecksum: u16 = 2;
    let mut consumertotal: usize = 3;
    let mut consumerchecksum: u16 = 4;
    
    eprintln!("actualate: Beginning.");

    let (supply_tx, supply_rx) = mpsc::sync_channel::<u8>(maximum);
    let (demand_tx, demand_rx) = mpsc::sync_channel::<u8>(maximum);
    
    let source = net::UdpSocket::bind("127.0.0.1:5555").expect("couldn't bind to address");
    let sink = net::UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    let destination = net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5555);
    
    eprintln!("actualate: Beginning.");
    
    let consuming = thread::spawn( move || { consumer(& mut demand_rx, & mut consumertotal, & mut consumerchecksum); } );
    let policing  = thread::spawn( move || { policer(& mut source, police, & mut demand_tx); } );
    let shaping   = thread::spawn( move || { shaper(& mut supply_rx, shape, & mut sink, & destination); } );
    let producing = thread::spawn( move || { producer(total, & mut supply_tx, & mut producertotal, & mut producerchecksum); } );
    
    eprintln!("actualate: Waiting.");
   
    let consumed = consuming.join();
    let policed = policing.join();
    let shaped = shaping.join();
    let produced = producing.join();

    eprintln!("actualate: Checking.");
    
    eprintln!("actualate: produced={}:{:04x}", producertotal, producerchecksum);
    eprintln!("actualate: consumer={}:{:04x}", consumertotal, consumerchecksum);

    assert!(consumertotal == producertotal);
    assert!(consumerchecksum == producerchecksum);
    
    eprintln!("actualate: Ending.");

}
