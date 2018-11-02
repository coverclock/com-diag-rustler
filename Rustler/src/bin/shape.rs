/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate clap;
extern crate rustler;

use std::io;
use std::io::Read;
use std::io::Write;
use std::io::ErrorKind;
use clap::Arg;
use clap::App;
use rustler::ticks::ticks;
use rustler::throttle::throttle;
use rustler::throttle::throttle::Throttle;
use rustler::gcra::gcra;
use rustler::contract::contract;

fn main() {
    let frequency: ticks::Ticks = ticks::frequency();
    let before: ticks::Ticks;
    let after: ticks::Ticks;
    let mut shape: contract::Contract = contract::Contract::new();
    let mut total: u64 = 0;
    let mut peak: f64 = 0.0;
    let mut count: usize = 0;
    let mut now: ticks::Ticks;
    let mut then: ticks::Ticks;
    let mut rate: f64;
    let mut length: usize;
    let mut buffer = [0u8; 65536] ; // Does Rust really have no way to allocate a u8 array on the heap sized at run time?
    let mut delay: ticks::Ticks;
    let mut admissable: bool;

    let matches = App::new("shape")
                          .version("1.0")
                          .author("Chip Overclock <coverclock@diag.com>")
                          .about("Shapes the data stream according a traffic contract.")
                          .arg(Arg::with_name("Debug")
                               .short("D")
                               .help("Enables debug output."))
                          .arg(Arg::with_name("Verbose")
                               .short("V")
                               .help("Enable verbose output."))
                           .arg(Arg::with_name("peakrate")
                               .short("p")
                               .help("Sets peak rate.")
                               .takes_value(true)
                               .value_name("BYTESPERSECOND"))
                           .arg(Arg::with_name("sustainedrate")
                               .short("s")
                               .help("Sets sustained rate.")
                               .takes_value(true)
                               .value_name("BYTESPERSECOND"))
                           .arg(Arg::with_name("burstsize")
                               .short("b")
                               .help("Sets maximum burst size.")
                               .takes_value(true)
                               .value_name("BYTES"))
                          .get_matches();

    let debug: bool = matches.is_present("Debug");

    let verbose: bool = matches.is_present("Verbose");

    let mut peakrate: throttle::Events = 1;
    if matches.is_present("peakrate") {
        let value = match usize::from_str_radix(matches.value_of("peakrate").unwrap(), 10) {
            Ok(value) => value,
            Err(_) => 0,
        };
        peakrate = value as throttle::Events;
    }
    let peakincrement: ticks::Ticks = gcra::increment(peakrate, 1, frequency);

    let mut sustainedrate: throttle::Events = 1;
    if matches.is_present("sustainedrate") {
        let value = match usize::from_str_radix(matches.value_of("sustainedrate").unwrap(), 10) {
            Ok(value) => value,
            Err(_) => 0,
        };
        sustainedrate = value as throttle::Events;
    }
    let sustainedincrement: ticks::Ticks = gcra::increment(sustainedrate, 1, frequency);

    let mut burstsize: throttle::Events = 1;
    if matches.is_present("burstsize") {
        let value = match usize::from_str_radix(matches.value_of("burstsize").unwrap(), 10) {
            Ok(value) => value,
            Err(_) => 0,
        };
        burstsize = value as throttle::Events;
    }
    let bursttolerance: ticks::Ticks = contract::bursttolerance(peakincrement, 0, sustainedincrement, burstsize);

    before = ticks::now();
    shape.init(peakincrement, 0 /* jittertolerance */, sustainedincrement, bursttolerance, before);
    
    if verbose { eprintln!("Contract: {}", shape.to_string()) }

    loop {
        
        length = match io::stdin().read(& mut buffer[..(burstsize as usize)]) {
            Ok(0) => break,
            Ok(value) => value,
            Err(ref error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(_) => break,     
        };
        if debug { eprintln!("Read: {}", length); }

        now = ticks::now();
        delay = shape.request(now);
        if debug { eprintln!("Delay: {}s.", (delay as f64) / (frequency as f64)); }
        ticks::sleep(delay);

        match io::stdout().write_all(&buffer[..length]) {
            Ok(_) => { },
            Err(_) => break,
        }
        if debug { eprintln!("Written: {}", length); }
        
        then = now;
        now = ticks::now();
        admissable = shape.admits(now, length as throttle::Events);
        if !admissable { eprintln!("Admissable: {}!", admissable); }

        if count <= 0 {
            // Do nothing.
        } else if length <= 0 {
            // Should never happen.
        } else if now <= then {
            // Should never happen.
        } else {
            rate = (length as f64) * (frequency as f64) / ((now - then) as f64);
            if rate > peak {
                peak = rate
            }
        }

        total += length as u64;
        count += 1;
        
    }

    now = ticks::now();
    shape.update(now);
    
    delay = shape.get_expected();
    if debug { eprintln!("Delay: {}s.", (delay as f64) / (frequency as f64)); }
    ticks::sleep(delay);
    
    after = ticks::now();
    shape.update(now);

    if verbose {
        eprintln!("Total: {}B.", total);
        eprintln!("Average: {}B/io.", (total as f64) / (count as f64));
        eprintln!("Peak: {}Bps.", peak);
        eprintln!("Sustained: {}Bps.", (total as f64) * (frequency as f64) / ((after - before) as f64));
    }

}
