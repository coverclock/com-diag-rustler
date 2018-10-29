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
use std::thread;
use rustler::ticks::ticks;
use rustler::throttle::throttle;
use rustler::fletcher::fletcher;

extern {
    /// rand(3) <stdlib.h> libc.
    fn rand() -> raw::c_int;
}

/// Compute a random blocksize between the values 1 and maximum inclusive.
pub fn blocksize(maximum: usize) -> usize {
    unsafe {
        let size: usize = ((rand() % (maximum as raw::c_int)) + 1) as usize;

        assert!(size >= 1);
        assert!(size <= maximum);

        return size;
    }
}

/// Compute a eight-bit datum between the values 1 and maximum inclusive.
pub fn payload(maximum: u8) -> u8 {
    unsafe {
        let byte: u8 = ((rand() % (maximum as raw::c_int)) + 1) as u8;

        assert!(byte >= 1);
        assert!(byte <= maximum);

        return byte;
    }
}

/*******************************************************************************
 * SIMULATED EVENT STREAM
 ******************************************************************************/

/// Simulate a data stream through a shaping throttle and a policing throttle
/// given a maximum blocksize value and a limit on iterations.
pub fn simulate(shape: & mut throttle::Throttle, police: & mut throttle::Throttle, maximum: usize, iterations: usize) {
    let frequency: f64 = ticks::frequency() as f64;
    let mut delay: ticks::Ticks;
    let mut now: ticks::Ticks = 0;
    let mut duration: ticks::Ticks = 0;
    let mut size: usize = 0;
    let mut total: usize = 0;
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
        total += size;
        
        admissable = shape.commits(size as throttle::Events);
        assert!(admissable);
        
        admitted = police.admits(now, size as throttle::Events);
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

const DEBUG: bool = true;

fn producer(maximum: usize, mut limit: usize, output: & mpsc::SyncSender<u8>, total: & mut usize, checksum: & mut u16) {
    let mut count: usize = 0;
    let mut largest: usize = 0;
    let mut cs: fletcher::Fletcher = fletcher::Fletcher::new();
    let mut size: usize;
    let mut datum = [0u8; 1];
    
    eprintln!("producer: begin burstsize={}B", maximum);
    
    while limit > 0 {
        
        size = blocksize(maximum as usize);
        if size > limit { size = limit }
        if size > largest { largest = size; }
        *total += size;
        limit -= size;
        count += 1;
            
        if DEBUG {
            eprintln!("producer: size={}B total={}B maximum={}B/burst", size, *total, largest);
        }
       
        while size > 0 {
            
            datum[0] = payload(b'~' - b' ' + 1) + b' ' - 1;
            *checksum = cs.checksum(&datum[..]);
            output.send(datum[0]);
            size -= 1;
            
        }

        datum[0] = 0x00;
        output.send(datum[0]);

        ticks::sleep(0);
        
    }
    
    drop(output);
    
    eprintln!("producer: end total={}B mean={}B/burst maximum={}B/burst.", *total, (*total as f64) / (count as f64), largest);
}

fn shaper(input: & mpsc::Receiver<u8>, shape: & mut throttle::Throttle, output: & net::UdpSocket, address: & net::SocketAddrV4) {
    let mut buffer = [0u8; 65536];
    
}

fn policer(input: & net::UdpSocket, police: & mut throttle::Throttle, output: & mpsc::SyncSender<u8>) {
    let mut buffer = [0u8; 65536];
    
}

fn consumer(maximum: usize, input: & mpsc::Receiver<u8>, total: & mut usize, checksum: & mut u16) {
    let mut cs: fletcher::Fletcher = fletcher::Fletcher::new();
    let mut datum = [0u8; 1];
    
    eprintln!("consumer: begin burstsize={}B", maximum);
    
    while true {
        
        let result = input.recv();
        if result == Err(mpsc::RecvError) { break; }
        datum[0] = result.unwrap();
        *total += 1;
        *checksum = cs.checksum(&datum[..]);

        if DEBUG && ((*total % maximum) == 0) {
            eprintln!("consumer: total={}B", *total);
        }
        
        ticks::sleep(0);
        
    }
    
    eprintln!("consumer: end total={}B", *total);
}

/// Exercise a shaping throttle and a policing throttle by producing an
/// actual event stream, shaping it, policing it, and consuming it four threads.
/// Because we pass the throttles to the threads and don't otherwise touch
/// the objects until the threads exit and we have joined with them, we don't
/// need to protect the objects with a Mutex. However, the lifetimes of the
/// throttles must be static so that they are not destroyed when they go out
/// of scope in the caller while the threads are still running.
pub fn exercise(shape: & 'static mut throttle::Throttle, police: & 'static mut throttle::Throttle, maximum: usize, total: usize) {
    let mut producertotal: usize = 1;
    let mut producerchecksum: u16 = 2;
    let mut consumertotal: usize = 3;
    let mut consumerchecksum: u16 = 4;
    
    eprintln!("exercise: Beginning.");

    let (supply_tx, supply_rx) = mpsc::sync_channel::<u8>(maximum + 1);
    let (demand_tx, demand_rx) = mpsc::sync_channel::<u8>(maximum);
    
    let source = net::UdpSocket::bind("127.0.0.1:5555").expect("couldn't bind to address");
    let sink = net::UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    let destination = net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5555);
        
    eprintln!("exercise: Starting.");
   
    let consuming = thread::spawn( move || { consumer(maximum, & demand_rx, & mut consumertotal, & mut consumerchecksum) } );

    let policing  = thread::spawn( move || { policer(& source, police, & demand_tx) } );

    let shaping   = thread::spawn( move || { shaper(& supply_rx, shape, & sink, & destination) } );

    let producing = thread::spawn( move || { producer(maximum, total, & supply_tx, & mut producertotal, & mut producerchecksum) } );
    
    eprintln!("exercise: Waiting.");
   
    let consumed = consuming.join();
    let policed = policing.join();
    let shaped = shaping.join();
    let produced = producing.join();

    eprintln!("exercise: Checking.");
    
    eprintln!("exercise: produced={}:{:04x}", producertotal, producerchecksum);
    eprintln!("exercise: consumer={}:{:04x}", consumertotal, consumerchecksum);

    assert!(consumertotal == producertotal);
    assert!(consumerchecksum == producerchecksum);
    
    eprintln!("exercise: Ending.");

}
