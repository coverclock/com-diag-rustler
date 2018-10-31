/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use std::os::raw;
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
    
    eprintln!("simulate: shape={}", shape.as_string());
    eprintln!("simulate: police={}", police.as_string());
    eprintln!("simulate: maximum={}", maximum);
    eprintln!("simulate: iterations={}", iterations);
     
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

    eprintln!("simulate: shape={}", shape.as_string());
    eprintln!("simulate: police={}", police.as_string());
    eprintln!("simulate: total={}B mean={}B/io maximum={}B/io latency={}s/io peak={}B/s sustained={}B/s.", total, average, maximum, mean, peak, sustained);
}

/*******************************************************************************
 * ACTUAL EVENT STREAM
 ******************************************************************************/

use std::sync;
use std::sync::mpsc;
use std::net;

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
            
        if DEBUG { eprintln!("producer: size={}B total={}B maximum={}B/burst.", size, *total, largest); }
       
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
    let frequency: f64 = ticks::frequency() as f64;
    let mut buffer = [0u8; 65536];
    let before: ticks::Ticks;
    let after: ticks::Ticks;
    let mut now: ticks::Ticks;
    let mut delay: ticks::Ticks;
    let mut duration: ticks::Ticks;
    let mut accumulated: ticks::Ticks = 0;
    let mut count: usize = 0;
    let mut rate: f64;
    let mut peak: f64 = 0.0;
    let mut size: usize = 0;
    let mut eof: bool = false;
    let mut largest: usize = 0;
    let mut alarmed: bool;
    let mut total: usize = 0;
    
    eprintln!("shaper: begin");
    
    before = ticks::now();
    
    while true {
        
        now = ticks::now();
        delay = shape.request(now);
        if DEBUG { eprintln!("consumer: delay={}s.", (delay as f64) / frequency); }
        assert!(delay >= 0);
        
        duration = delay;
        accumulated += delay;
        
        ticks::sleep(delay);
        
        now = ticks::now();
        delay = shape.request(now);
        assert!(delay == 0);
        
        if count == 0 {
            // Do nothing.
        } else if duration == 0 {
            // Do nothing.
        } else {
            rate = ((size as f64) * frequency) / (duration as f64);
            if rate > peak { peak = rate; }
        }
        
        size = 0;
        while true {
            
            match input.recv() {
                Ok(value) => { buffer[size] = value; size+=1; },
                Err(_) => { eof = true; }
            }
            if eof { break; }
            if buffer[size] == 0x00 { break; }
        
        }
        if eof { break; }
        if size > largest { largest = size; }
        total += size;
        
        match output.send_to(&buffer[..size], address) {
            Ok(_) => { },
            Err(_) => { panic!(); }
        }
        
        alarmed = !shape.commits(size as throttle::Events);
        assert!(!alarmed);
        count += 1;
       
        if DEBUG { eprintln!("shaper: size={}B total={}B maximum={}B/burst.", size, total, largest); }
        
    }
    
    now = ticks::now();
    shape.update(now);
    delay = shape.get_expected();
    if DEBUG { eprintln!("consumer: delay={}s", (delay as f64) / frequency); }
    ticks::sleep(delay);
    now = ticks::now();
    shape.update(now);
    after = now;
    
    buffer[0] = 0x00;
    size = 1;
    match output.send_to(&buffer[..size], address) {
        Ok(_) => { },
        Err(_) => { panic!(); }
    }
    
    let average: f64 = (accumulated as f64) / (count as f64) / frequency;
    let mean: f64 = (total as f64) / (count as f64);
    let sustained: f64 = (total as f64) * frequency / ((after - before) as f64);
    
    eprintln!("shaper: end total={}B mean={}B/burst maximum={}B/burst delay={}s/burst peak={}B/s sustained={}B/s", total, mean, largest, average, peak, sustained);    
}

fn policer(input: & net::UdpSocket, police: & mut throttle::Throttle, output: & mpsc::Sender<u8>) {
    let mut eof: bool = false;
    let mut buffer = [0u8; 65536];
    let frequency: f64 = ticks::frequency() as f64;
    let before: ticks::Ticks;
    let after: ticks::Ticks;
    let mut now: ticks::Ticks = 0;
    let mut then: ticks::Ticks;
    let mut size: usize = 0;
    let mut count: usize = 0;
    let mut rate: f64;
    let mut peak: f64 = 0.0;
    let mut total: usize = 0;
    let mut largest: usize = 0;
    let mut admissable: bool;
    let mut admitted: usize = 0;
    let mut policed: usize = 0;
    let mut index: usize;
    
    eprintln!("policer: start");
    
    before = ticks::now();
    
    while !eof {       
        
        match input.recv_from(& mut buffer) {
            Ok(value) => { size = value.0; }
            Err(_) => { panic!(); }
        }
        assert!(size > 0);

        if buffer[size - 1] == 0x00 {
            eof = true;
            size = 0;
        }

        then = now;
        now = ticks::now();
        
        if count == 0 {
            // Do nothing.
        } else if size == 0 {
            // Do nothing.
        } else if now <= then {
            // Do nothing.
        } else {
            rate = (size as f64) * frequency / ((now - then) as f64);
            if rate > peak { peak = rate; }
        }
        
        if size > 0 {
        
            total += size;
            if size > largest { largest = size; }
            
            admissable = police.admits(now, size as throttle::Events);
            if admissable {
                admitted += size;
                if DEBUG { eprintln!("policer: admitted={} size={}B total={}B.", admitted, size, total); }
            } else {
                policed += size;
                if DEBUG { eprintln!("policer: policed={} size={}B total={}B.", policed, size, total); }
            }
            
            index = 0;
            while index < size {
                output.send(buffer[index]);
                index += 1;
            }
            
            count += 1;
        
        } else if eof {
            
            police.update(now);
            
        } else {
            
            panic!();
            
        }
        
        ticks::sleep(0);
        
    }
    
    after = ticks::now();
    
    drop(output);
    
    let mean: f64 = (total as f64) / (count as f64);
    let sustained: f64 = (total as f64) * frequency / ((after - before) as f64);
    
    eprintln!("policer: count={} admitted={} policed={}", count, admitted, policed);
    eprintln!("policer: end total={}B mean={}B/burst maximum={}B/burst peak={}B/s sustained={}B/s", total, mean, largest, peak, sustained);    
}

fn consumer(maximum: usize, input: & mpsc::Receiver<u8>, total: & mut usize, checksum: & mut u16) {
    let mut cs: fletcher::Fletcher = fletcher::Fletcher::new();
    let mut eof: bool = false;
    let mut datum = [0u8; 1];
    
    eprintln!("consumer: begin burstsize={}B", maximum);
    
    while true {

        match input.recv() {
            Ok(value) => { datum[0] = value; *total += 1; *checksum = cs.checksum(&datum[..]); },
            Err(_) => { eof = true; }
        }
        if eof { break; }

        if DEBUG && ((*total % maximum) == 0) { eprintln!("consumer: total={}B.", *total); }
        
        ticks::sleep(0);
        
    }
    
    eprintln!("consumer: end total={}B", *total);
}

/*

/// Exercise a shaping throttle and a policing throttle by producing an
/// actual event stream, shaping it, policing it, and consuming it four threads.
pub fn exercise(shape: & mut throttle::Throttle, police: & mut throttle::Throttle, maximum: usize, total: usize) {
    let mut producertotal: usize = 1;
    let mut producerchecksum: u16 = 2;
    let mut consumertotal: usize = 3;
    let mut consumerchecksum: u16 = 4;
    
    eprintln!("exercise: Beginning.");

    let (supply_tx, supply_rx) = mpsc::sync_channel::<u8>(maximum + 1);
    let (demand_tx, demand_rx) = mpsc::channel::<u8>();

    let source = net::UdpSocket::bind("127.0.0.1:5555").expect("couldn't bind to address");
    let sink = net::UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    let destination = net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 5555);
       
    eprintln!("exercise: Starting.");
   
    let consuming = thread::spawn( move || { consumer(maximum, & demand_rx, & mut consumertotal, & mut consumerchecksum) } );
    let policing  = thread::spawn( move || { policer(& source, police, & demand_tx) } );
    let shaping   = thread::spawn( move || { shaper(& supply_rx, shape, & sink, & destination) } );
    let producing = thread::spawn( move || { producer(maximum, total, & supply_tx, & mut producertotal, & mut producerchecksum) } );
    
    eprintln!("exercise: Waiting.");

    match consuming.join() {
        Ok(_) => { },
        Err(_) => { panic!(); }
    }
    match policing.join() {
        Ok(_) => { },
        Err(_) => { panic!(); }
    }
    match shaping.join() {
        Ok(_) => { },
        Err(_) => { panic!(); }
    }
    match producing.join() {
        Ok(_) => { },
        Err(_) => { panic!(); }
    }

    eprintln!("exercise: Checking.");
    
    eprintln!("exercise: produced={}:{:04x}.", producertotal, producerchecksum);
    eprintln!("exercise: consumer={}:{:04x}.", consumertotal, consumerchecksum);

    assert!(consumertotal == producertotal);
    assert!(consumerchecksum == producerchecksum);
    
    eprintln!("exercise: Ending.");

}

*/
